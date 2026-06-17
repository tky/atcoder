const MOD: usize = 1_000_000_007;

fn resolve(s: &[char]) -> usize {
    let n = s.len() + 1;

    let mut dp = vec![vec![0usize; n + 1]; n + 1];
    dp[1][0] = 1;

    for i in 2..=n {
        if s[i - 2] == '<' {
            // 前の行 dp[i - 1] の累積和を作る。
            //
            // pref[k]
            // = dp[i - 1][0]
            // + dp[i - 1][1]
            // + ...
            // + dp[i - 1][k]
            //
            // これにより、前の順位の連続区間の和を O(1) で取得できる。
            let mut pref = vec![0usize; i - 1];
            pref[0] = dp[i - 1][0];
            for k in 1..(i - 1) {
                pref[k] = (dp[i - 1][k] + pref[k - 1]) % MOD;
            }
            // p(i-1) < p(i)
            for j in 0..i {
                if j == 0 {
                    // 新しい最後の値が最小。
                    // '<' を満たすには直前の値がさらに小さくなければならないが、
                    // 最小値より小さい値は存在しない。
                    dp[i][j] = 0;
                } else {
                    // 直前の値の順位は 0..j-1。
                    dp[i][j] = pref[j - 1];
                }
            }
        } else {
            // suff[j] = dp[i - 1][j] + dp[i - 1][j + 1] + ... + dp[i - 1][i - 2]
            // suff[i - 2] = dp[i - 1][i - 2]
            // suff[j] = suff[j + 1] + dp[i - 1][j]
            let mut suff = vec![0usize; i - 1];

            suff[i - 2] = dp[i - 1][i - 2];
            for k in (0..(i - 2)).rev() {
                suff[k] = (suff[k + 1] + dp[i - 1][k]) % MOD;
            }
            for j in 0..i {
                if j == i - 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = suff[j];
                }
            }
        }
    }
    dp[n].iter().sum::<usize>() % MOD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let s: Vec<char> = "<><".chars().collect();
        assert_eq!(resolve(&s), 5);
    }

    #[test]
    fn sample_02() {
        let s: Vec<char> = "<<<<".chars().collect();
        assert_eq!(resolve(&s), 1);
    }

    #[test]
    fn sample_03() {
        let s: Vec<char> = ">>>><>>><>><>>><<>>".chars().collect();
        assert_eq!(resolve(&s), 217136290);
    }

    #[test]
    fn n_2_less() {
        let s: Vec<char> = "<".chars().collect();
        assert_eq!(resolve(&s), 1);
    }

    #[test]
    fn n_2_greater() {
        let s: Vec<char> = ">".chars().collect();
        assert_eq!(resolve(&s), 1);
    }

    #[test]
    fn n_3_less_less() {
        let s: Vec<char> = "<<".chars().collect();
        assert_eq!(resolve(&s), 1);
    }

    #[test]
    fn n_3_greater_greater() {
        let s: Vec<char> = ">>".chars().collect();
        assert_eq!(resolve(&s), 1);
    }

    #[test]
    fn n_3_less_greater() {
        let s: Vec<char> = "<>".chars().collect();
        assert_eq!(resolve(&s), 2);
    }

    #[test]
    fn n_3_greater_less() {
        let s: Vec<char> = "><".chars().collect();
        assert_eq!(resolve(&s), 2);
    }
}

/**
 *  s[1] == '>'の場合
 * dp[3][0]
 * 長さ3の中でp3が一番小さい
 * 直前のp2はなんでも良い
 * dp[3][0] = dp[2][0] + dp[2][1]
 * dp[3][1]
 * 長さ3の中でp3が2番目に小さい
 * 直前のp2は一番小さい値は使えない
 * dp[3][1] = dp[2][1]
 * dp[3][2]
 * p3が一番大きい > 条件違反
 * dp[3][2] = 0
 *
 * s[i-2] == '>'の場合
 *
 * dp[i][0]: 長さiの中でp[i]が一番小さい
 * 直前のp[i-1]はなんでも良い
 * dp[i][0] = dp[i-1][0->(i-2)]
 * dp[i][1]: 長さiの中でp[i]が二番目に小さい
 * 直前のp[i-1]に一番小さい値は使えない
 * dp[i][1] = dp[i-1][1->(i-2)]
 *
 * dp[i][j] = dp[i-1][j -> (i-2)]
 * dp[i][i-1] = 0
 *
 * s[1] == '<'の場合
 * dp[3][0]: 長さ3の中でp3が一番小さい > 条件違反
 * dp[3][0] = 0
 * dp[3][1]: 長さ3の中でp3が二番目に小さい > p2は一番小さい値
 * dp[3][1] = dp[2][0]
 * dp[3][2]: 長さ3の中でp3が一番大きい > p2は何を使っても良い
 * dp[3][2] = dp[2][0] + dp[2][1]
 *
 * s[i-2] == '<'の場合
 * dp[i][0]: 長さiの中でp[i]が一番小さい > 条件違反
 * dp[i][0] = 0
 * dp[i][1]: 長さiの中でp[i]が二番目に小さい > p[i-1]は一番小さい値
 * dp[i][1] = dp[i-1][0]
 * dp[i][j]: 長さiの中でp[i]が番目に小さい > p[i-1]はそれより小さいしか使えない
 * dp[i][j] = dp[i-1][0 -> (j-1)]
 *
 */
fn main() {
    println!("Hello, world!");
}
