use std::io::{self, Read};

const MOD: usize = 1_000_000_007;

fn resolve(k: &str, d: usize) -> usize {
    // dp[r][less]
    // r: ここまでの桁和 mod d
    // less = 0: ここまで K と一致している
    // less = 1: すでに K より小さいことが確定している
    // dp[r][less] は「今まで作った途中の数字たちを、桁和 mod D と less で分類して数えている」
    // 桁和だけならdp[r]で良い
    // たとえば、123と213は桁和は同じ
    // それだとkを超えてしまうのでlessで状態を管理する必要がある
    // 問題が[1,2,3]の集合で作れるDの倍数は何個？
    // だったらdp[r]でよくなるが、この問題は上限があるためdp[r][less]という状態を持つ必要がある
    let mut dp = vec![vec![0usize; 2]; d];

    // まだ 1 桁も決めていない状態
    // 桁和 mod d = 0
    // まだ K と一致している
    dp[0][0] = 1;

    for ch in k.chars() {
        // この桁でおける最大値
        let limit = ch.to_digit(10).unwrap() as usize;
        let mut next = vec![vec![0usize; 2]; d];

        for r in 0..d {
            // less:0 ここまでKと完全に一致
            // less:1 すでにKより小さいことが確定している
            for less in 0..2 {
                // ここまでの桁和 mod dがrで、lessのprefixがいくつあるか
                let cur = dp[r][less];
                if cur == 0 {
                    continue;
                }

                let upper = if less == 1 { 9 } else { limit };

                for digit in 0..=upper {
                    //ここまでの桁和の余り r に、次の桁 digit を足した後の余り
                    let new_r = (r + digit) % d;

                    // すでに小さいことが確定しているので次のlessは1
                    let new_less = if less == 1 {
                        1
                    } else if digit < limit {
                        1
                    // less == 0 and digit == limit
                    // まだKと一致している
                    } else {
                        0
                    };

                    next[new_r][new_less] += cur;
                    next[new_r][new_less] %= MOD;
                }
            }
        }

        dp = next;
    }

    // dp[0][0] + dp[0][1] は、0 以上 K 以下で桁和が d の倍数の個数
    // 問題は 1 以上 K 以下なので、0 を除外するために 1 引く
    (dp[0][0] + dp[0][1] + MOD - 1) % MOD
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let k = iter.next().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();

    println!("{}", resolve(k, d));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve("30", 4), 6);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve("1000000009", 1), 2);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve("98765432109876543210", 58), 635_270_834);
    }

    #[test]
    fn small_01() {
        // 1..=9 のうち、桁和が 3 の倍数
        // 3, 6, 9
        assert_eq!(resolve("9", 3), 3);
    }

    #[test]
    fn small_02() {
        // 1..=10 のうち、桁和が 1 の倍数
        // 全部
        assert_eq!(resolve("10", 1), 10);
    }

    #[test]
    fn small_03() {
        // 1..=20 のうち、桁和が 2 の倍数
        // 2, 4, 6, 8, 11, 13, 15, 17, 19, 20
        assert_eq!(resolve("20", 2), 10);
    }
}
