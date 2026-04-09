// 先手が勝つ場合をtrueとしています
fn resolve(n: usize, ks: &[usize]) -> bool {
    let mut dp = vec![false; n + 1];
    dp[0] = false;

    for i in 1..=n {
        let mut win = false;
        for &k in ks {
            if k <= i {
                // dp[i]: 石がi個ある状態で、今から手番の人が勝てるか
                // dp[i - a] は 「自分が a 個取ったあと、石が i-a 個になって、次に手番になる人が勝てるか」
                // 次に手番になる人が勝てないなら自分が勝ち
                // dp[i] = ある a <= i について dp[i - a] == false なら true そうでなければ false
                win |= !dp[i - k];
                if win {
                    break;
                }
            }
        }
        dp[i] = win;
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(4, &vec![2, 3]), true);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(5, &vec![2, 3]), false);
    }
}

fn main() {
    println!("Hello, world!");
}
