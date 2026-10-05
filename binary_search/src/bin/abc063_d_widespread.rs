// https://atcoder.jp/contests/abc063/tasks/arc075_b

// N体の魔物(体力 h_i)。1回の爆発で、選んだ1体にAダメージ、他の全員にBダメージ(A > B)。
// 全員の体力を0以下にする最小の爆発回数を求める。
//
// Logsと同じ「答えの二分探索 + O(N)判定」の一本道。判定問題の設計がひと工夫:
// 「t回で全滅できるか?」
// - t回爆発すれば、誰であれ最低 B*t は食らう(全体攻撃分)
// - それでも残る魔物には「中心に指名」して差分 (A - B) を上乗せするしかない
// - 魔物iに必要な指名回数は ceil((h_i - B*t) / (A - B))(残りがなければ0回)
// - 指名の合計がt回以内に収まれば実現可能
// 単調性: tが増えるほど楽になる(可能側は大きいt = Logsと同じ向き)
fn resolve(hs: &[usize], a: usize, b: usize) -> usize {
    let mut ng = 0usize;
    let mut ok = 1_000_000_000usize;

    while ng < ok {
        let t = (ng + ok) / 2;
        if can_eliminate(hs, a, b, t) {
            ok = t;
        } else {
            ng = t + 1;
        }
    }

    ng
}

fn can_eliminate(hs: &[usize], a: usize, b: usize, t: usize) -> bool {
    let mut count = 0;
    let diff = a - b;
    for &h in hs {
        let remain = h.saturating_sub(t * b);
        if remain > 0 {
            count += remain.div_ceil(diff);
        }
    }
    count <= t
}

fn resolve1(hs: &[usize], a: usize, b: usize) -> usize {
    let mut hs = hs.to_vec();

    let diff = a - b;
    let mut count = 0;
    while !check(&hs) {
        count += 1;
        let (index, _) = hs
            .iter()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .unwrap();

        hs = hs.iter().map(|&d| d.saturating_sub(b)).collect();

        hs[index] = hs[index].saturating_sub(diff);
    }
    count
}

fn check(hs: &[usize]) -> bool {
    hs.iter().all(|&d| d == 0)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod can_eliminate_tests {
    use super::can_eliminate;
    #[test]
    fn sample_01() {
        assert_eq!(can_eliminate(&[8, 7, 4, 2], 5, 3, 1), false);
        assert_eq!(can_eliminate(&[8, 7, 4, 2], 5, 3, 2), true);
        assert_eq!(can_eliminate(&[8, 7, 4, 2], 5, 3, 3), true);
    }

    #[test]
    fn sample_02() {
        assert_eq!(can_eliminate(&[20, 20], 10, 4, 1), false);
        assert_eq!(can_eliminate(&[20, 20], 10, 4, 2), false);
        assert_eq!(can_eliminate(&[20, 20], 10, 4, 3), false);
        assert_eq!(can_eliminate(&[20, 20], 10, 4, 4), true);
        assert_eq!(can_eliminate(&[20, 20], 10, 4, 5), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[8, 7, 4, 2], 5, 3), 2);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[20, 20], 10, 4), 4);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &[900000000, 900000000, 1000000000, 1000000000, 1000000000],
                2,
                1
            ),
            800000000
        );
    }

    #[test]
    fn cross_check_with_naive() {
        // 注意: resolve1(シミュレーション)のコストは「答えの回数」に比例するので、
        // 体力を小さく絞って答えが数十回に収まる世界でだけ突き合わせる
        let mut seed: usize = 42;
        let mut next = move |m: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) % m
        };

        for _ in 0..200 {
            let n = next(8) + 1;
            let hs: Vec<usize> = (0..n).map(|_| next(50) + 1).collect();
            let b = next(9) + 1;
            let a = b + next(9) + 1; // A > B を構造的に保証
            assert_eq!(
                resolve(&hs, a, b),
                resolve1(&hs, a, b),
                "hs={hs:?}, a={a}, b={b}"
            );
        }

        // 境界: 1体だけ / 全員同体力 / A-B=1(指名の旨味が最小)
        assert_eq!(resolve(&[1], 2, 1), resolve1(&[1], 2, 1));
        assert_eq!(resolve(&[50], 2, 1), resolve1(&[50], 2, 1));
        let same = vec![30, 30, 30];
        assert_eq!(resolve(&same, 5, 4), resolve1(&same, 5, 4));
    }
}
