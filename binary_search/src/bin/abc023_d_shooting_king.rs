use itertools::Itertools;

// https://atcoder.jp/contests/abc023/tasks/abc023_d

// N個の風船。風船iは高さH_iから毎秒S_i上昇する。
// 時刻0に1個、以降1秒ごとに1個割る(時刻 0, 1, 2, ..., N-1 で1個ずつ)。
// 風船iを時刻tで割るとペナルティ H_i + S_i * t。
// 全風船のペナルティの最大値を、割る順序を工夫して最小化する。
//
// 最終問題: 「最大値の最小化」+ 判定問題の設計(二段の言い換え)
// 1. 答えXを決め打ち: 「全風船をペナルティX以下で割り切れるか?」
// 2. 風船iをX以下で割るには H_i + S_i * t <= X、つまり
//    時刻 (X - H_i) / S_i (切り捨て) までに割る必要がある = 「締切」が生まれる
//    (H_i > X の風船が1つでもあれば即不可能)
// 3. 締切を守れるか? は、どういう順番で割るのが最善かを考えると判定できる
//    (ヒント: 時刻 0, 1, 2, ... に1個ずつしか割れない)
//
// 値域に注意: Xは最大で H + S*(N-1) ≈ 10^9 + 10^9 * 10^5 = 10^14 のオーダー
fn resolve(balloons: &[(usize, usize)]) -> usize {
    let n = balloons.len();
    let mut bottom = 0usize;
    let mut top = balloons
        .iter()
        .map(|&(h, s)| h + s * (n - 1))
        .max()
        .unwrap();
    while bottom < top {
        let x = (bottom + top) / 2;
        if judge(balloons, x) {
            top = x;
        } else {
            bottom = x + 1;
        }
    }
    bottom
}

fn resolve1(balloons: &[(usize, usize)]) -> usize {
    let n = balloons.len();
    let mut result = usize::MAX;
    for items in balloons.iter().permutations(n) {
        let mut score = 0;
        let mut time = 0;
        for &(h, s) in items {
            score = score.max(h + (s * time));
            time += 1;
        }
        result = result.min(score);
    }
    result
}

fn judge(balloons: &[(usize, usize)], x: usize) -> bool {
    if balloons.iter().any(|&(h, _)| h > x) {
        return false;
    }

    let mut balloons = balloons.to_vec();
    balloons.sort_unstable_by_key(|&(h, s)| (x - h) / s);

    let mut times = 0;
    for &(h, s) in balloons.iter() {
        let score = h + (s * times);
        if score > x {
            return false;
        }
        times += 1;
    }
    true
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests_judge {
    use super::judge;

    #[test]
    fn sample_01() {
        assert_eq!(judge(&[(5, 6), (12, 4), (14, 7), (21, 2)], 21), false);
        assert_eq!(judge(&[(5, 6), (12, 4), (14, 7), (21, 2)], 22), false);
        assert_eq!(judge(&[(5, 6), (12, 4), (14, 7), (21, 2)], 23), true);
        assert_eq!(judge(&[(5, 6), (12, 4), (14, 7), (21, 2)], 24), true);
    }

    #[test]
    fn sample_02() {
        assert_eq!(
            judge(
                &[(100, 1), (100, 1), (100, 1), (100, 1), (100, 1), (1, 30)],
                103
            ),
            false
        );

        assert_eq!(
            judge(
                &[(100, 1), (100, 1), (100, 1), (100, 1), (100, 1), (1, 30)],
                104
            ),
            false
        );
        assert_eq!(
            judge(
                &[(100, 1), (100, 1), (100, 1), (100, 1), (100, 1), (1, 30)],
                105
            ),
            true
        );
        assert_eq!(
            judge(
                &[(100, 1), (100, 1), (100, 1), (100, 1), (100, 1), (1, 30)],
                106
            ),
            true
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[(5, 6), (12, 4), (14, 7), (21, 2)]), 23);
    }

    #[test]
    fn sample_02() {
        assert_eq!(
            resolve(&[(100, 1), (100, 1), (100, 1), (100, 1), (100, 1), (1, 30)]),
            105
        );
    }

    #[test]
    fn handmade_tiny() {
        assert_eq!(resolve(&[(1, 1)]), 1); // 1個なら時刻0で割るだけ
        assert_eq!(resolve(&[(10, 5), (10, 5)]), 15); // どちらかは時刻1 → 10 + 5
    }

    #[test]
    fn large_s() {
        // 3本とも毎秒10^9上昇: どの順でも t=0,1,2 で割るしかなく、最悪は 1 + 2*10^9
        assert_eq!(
            resolve(&[(1, 1_000_000_000), (1, 1_000_000_000), (1, 1_000_000_000)]),
            2_000_000_001
        );
    }

    #[test]
    fn cross_check_with_naive() {
        let mut seed: usize = 42;
        let mut next = move |m: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) % m
        };

        for _ in 0..100 {
            let n = next(6) + 1;
            let hs: Vec<(usize, usize)> = (0..n)
                .map(|_| (next(1_000_000_000) + 1, next(1_000_000_000) + 1))
                .collect();

            assert_eq!(resolve(&hs), resolve1(&hs), "hs={hs:?}");
        }
    }
}
