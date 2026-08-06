fn resolve(stone_count: usize, moves: &[usize]) -> bool {
    // dp[i]: 石が i 個ある状態で、手番のプレイヤーが勝てるなら true
    let mut dp = vec![false; stone_count + 1];

    for i in 1..=stone_count {
        // dp[i]: 石がi個ある状態で、今から手番の人が勝てるか
        // dp[i - a] は 「自分が a 個取ったあと、石が i-a 個になって、次に手番になる人が勝てるか」
        // 次に手番になる人が勝てないなら自分が勝ち
        // dp[i] = ある a <= i について dp[i - a] == false なら true そうでなければ false
        dp[i] = moves.iter().any(|&x| x <= i && !dp[i - x]);
    }

    dp[stone_count]
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
