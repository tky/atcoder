fn resolve(n: usize, intervals: &[(usize, usize, i64)]) -> i64 {
    // by_r[r]: 右端がrの区間たち
    let mut by_r = vec![vec![]; n + 1];

    for &(l, r, a) in intervals {
        by_r[r].push((l, a));
    }

    // dp[j]: 今みている位置までで、最後の1の位置がjの時の最大スコア
    let mut dp = vec![0_i64; n + 1];
    for r in 1..=n {
        // 位置 r を 1 にする
        // 以前の最後の 1 の位置がどこでも、r に 1 を置けば最後の 1 は r になる

        //現在、最後の 1 の位置が 0, 1, 2, ..., r-1 の状態の中で最大スコア
        let best = dp[0..r].iter().copied().max().unwrap();
        dp[r] = dp[r].max(best);

        // 右端が r の区間 [l, r] を処理する
        // 最後の 1 の位置 j が l 以上なら、この区間には 1 がある
        for &(l, a) in &by_r[r] {
            // 最後の 1 の位置が l, l+1, ..., r の状態は、
            // 区間 [l, r] の中に 1 があることが確定しているので、
            // それらすべてに a を足す
            for j in l..=r {
                dp[j] += a;
            }
        }
    }
    *dp.iter().max().unwrap()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let intervals = vec![(1, 3, 10), (2, 4, -10), (3, 5, 10)];

        assert_eq!(resolve(5, &intervals), 20);
    }

    #[test]
    fn sample_02() {
        let intervals = vec![(1, 1, -100), (2, 2, -100), (3, 3, -100)];

        // 全部 0 にすれば、どの区間も満たさないので 0
        assert_eq!(resolve(3, &intervals), 0);
    }

    #[test]
    fn separated_positive_intervals() {
        let intervals = vec![(1, 1, 10), (5, 5, 10)];

        // 10001 にすれば両方満たせる
        assert_eq!(resolve(5, &intervals), 20);
    }

    #[test]
    fn one_interval_positive() {
        let intervals = vec![(2, 5, 10)];

        // 2..=5 のどこかに 1 を置けば +10
        assert_eq!(resolve(5, &intervals), 10);
    }

    #[test]
    fn one_interval_negative() {
        let intervals = vec![(2, 5, -10)];

        // 2..=5 に 1 を置かなければよいので 0
        assert_eq!(resolve(5, &intervals), 0);
    }

    #[test]
    fn positive_and_negative_overlap() {
        let intervals = vec![(1, 3, 10), (2, 4, -100), (4, 5, 10)];

        // 10001 なら [1,3] と [4,5] を満たし、[2,4] は満たさない
        // score = 10 + 10 = 20
        assert_eq!(resolve(5, &intervals), 20);
    }

    #[test]
    fn all_positive_can_be_satisfied() {
        let intervals = vec![(1, 3, 5), (2, 4, 7), (3, 5, 11)];

        // 例えば 00100 なら全部満たす
        assert_eq!(resolve(5, &intervals), 23);
    }

    #[test]
    fn choose_not_to_take_negative_even_with_positive() {
        let intervals = vec![(1, 5, 100), (3, 3, -1000)];

        // 位置3以外に1を置けば +100 だけ取れる
        assert_eq!(resolve(5, &intervals), 100);
    }
}
