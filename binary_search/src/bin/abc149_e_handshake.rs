// https://atcoder.jp/contests/abc149/tasks/abc149_e

// ゲストN人(パワーA_i)。握手は「左手のゲストx、右手のゲストy」の順序対で、
// 同じ (x, y) は一度きり(x == y も可、(x,y)と(y,x)は別)。全部で N^2 通り。
// M回の握手で得られる幸福度 Σ(A_x + A_y) の最大値を求める。
// N <= 10^5, M <= N^2 = 10^10 なので「全ペアを並べて上からM個」は不可。
//
// 2段構えの問題(Step 2 + Step 1 の合わせ技):
// 1. 「和が t 以上のペアだけ全部選ぶ」としたときのペア数は、tを下げると単調に増える。
//    「ペア数が M 以上になる境界の t」を答えの二分探索で探す(ペア数のカウントは
//    ソート + 二分探索: Snuke Festival でやった形)
// 2. 境界 t が決まったら、「和が t+1 以上のペア」は全部選び、残りは和がちょうど t の
//    ペアで埋める。「和が t 以上のペアの総和」を高速に出すには累積和が要る
//    (各 x に対し「A_y >= t - A_x となる y」の個数と、その A_y たちの合計)
fn resolve(a: &[usize], m: usize) -> usize {
    let mut min: usize = 0;
    let mut max: usize = 2 * 100_000 + 1;

    let mut sorted_a = a.to_vec();
    sorted_a.sort_unstable();

    while min < max {
        let mid = (min + max) / 2;
        let count = count_pairs(&sorted_a, mid);

        if count >= m {
            min = mid + 1;
        } else {
            max = mid;
        }
    }
    let t = min - 1;

    let mut suffix = vec![0; a.len() + 1];
    for i in (0..a.len()).rev() {
        suffix[i] = sorted_a[i] + suffix[i + 1];
    }

    let mut total = 0;
    for &x in &sorted_a {
        let pos = binary_search(&sorted_a, t.saturating_sub(x));
        total += x * (a.len() - pos) + suffix[pos];
    }
    total -= (count_pairs(&sorted_a, t) - m) * t;
    total
}

fn resolve1(a: &[usize], m: usize) -> usize {
    let n = a.len();

    let mut ans = vec![0; n * n];

    let mut idx = 0;
    for l in 0..n {
        for r in 0..n {
            ans[idx] = a[l] + a[r];
            idx = idx + 1;
        }
    }

    ans.sort();

    ans[n * n - m..].iter().sum()
}

// 和がt以上になる順序対(x, y)の個数
fn count_pairs(sorted_a: &[usize], t: usize) -> usize {
    let n = sorted_a.len();

    let mut count = 0;
    for i in 0..n {
        count += n - binary_search(sorted_a, t.saturating_sub(sorted_a[i]));
    }
    count
}

fn binary_search(vs: &[usize], v: usize) -> usize {
    let mut left = 0;
    let mut right = vs.len();

    while left < right {
        let mid = (left + right) / 2;

        if vs[mid] < v {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[10, 14, 19, 34, 33], 3), 202);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[1, 3, 5, 110, 24, 21, 34, 5, 3], 14), 1837);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &[67597, 52981, 5828, 66249, 75177, 64141, 40773, 79105, 16076],
                73
            ),
            8128170
        );
    }

    // count_pairs の対照実装: 全順序対を数えるだけ
    fn count_pairs_naive(a: &[usize], t: usize) -> usize {
        a.iter()
            .flat_map(|&x| a.iter().map(move |&y| x + y))
            .filter(|&s| s >= t)
            .count()
    }

    #[test]
    fn count_pairs_hand_calculated() {
        // sorted_a = [1, 2, 3] の全9ペアの和: [2, 3, 3, 4, 4, 4, 5, 5, 6]
        let a = [1, 2, 3];
        assert_eq!(count_pairs(&a, 2), 9); // 最小和ちょうど → 全部
        assert_eq!(count_pairs(&a, 3), 8);
        assert_eq!(count_pairs(&a, 4), 6);
        assert_eq!(count_pairs(&a, 5), 3);
        assert_eq!(count_pairs(&a, 6), 1); // 最大和ちょうど → 1個
        assert_eq!(count_pairs(&a, 7), 0); // 最大和超え → 0個
    }

    #[test]
    fn count_pairs_saturating_boundary() {
        // t <= A_x のとき t - A_x がアンダーフローする経路(相手は誰でもよい = n個)
        let a = [1, 2, 3];
        assert_eq!(count_pairs(&a, 0), 9);
        assert_eq!(count_pairs(&a, 1), 9);
        // 1要素・自分との握手のみ
        let single = [5];
        assert_eq!(count_pairs(&single, 10), 1);
        assert_eq!(count_pairs(&single, 11), 0);
        assert_eq!(count_pairs(&single, 1), 1);
    }

    #[test]
    fn resolve_cross_check_with_naive() {
        let mut seed: usize = 123;
        let mut next = move |m: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) % m
        };

        for _ in 0..200 {
            let n = next(8) + 1;
            // 値域を 1..=10 と狭くして「和がちょうど t*」の同点を多発させる
            // (端数調整 (count - m) * t の経路を重点的に踏む)
            let a: Vec<usize> = (0..n).map(|_| next(10) + 1).collect();
            let m = next(n * n) + 1; // 1..=n^2
            assert_eq!(resolve(&a, m), resolve1(&a, m), "a={a:?}, m={m}");
        }

        // 両端: m = 1(最強ペアのみ)と m = n^2(全ペア)
        let a = vec![3, 1, 4, 1, 5];
        assert_eq!(resolve(&a, 1), resolve1(&a, 1));
        assert_eq!(resolve(&a, 25), resolve1(&a, 25));
        // 全員同じパワー(全ペア同点 = 調整が最大限働く)
        let same = vec![7, 7, 7, 7];
        for m in 1..=16 {
            assert_eq!(resolve(&same, m), resolve1(&same, m), "m={m}");
        }
    }

    #[test]
    fn count_pairs_cross_check_with_naive() {
        let mut seed: usize = 42;
        let mut next = move |m: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) % m
        };

        for _ in 0..100 {
            let len = next(20) + 1;
            let mut a: Vec<usize> = (0..len).map(|_| next(50) + 1).collect();
            a.sort_unstable();
            // 和の値域(2..=100)の外側も含めて突く
            let t = next(110);
            assert_eq!(
                count_pairs(&a, t),
                count_pairs_naive(&a, t),
                "a={a:?}, t={t}"
            );
        }
    }
}
