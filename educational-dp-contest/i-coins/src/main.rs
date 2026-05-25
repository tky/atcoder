fn resolve(ps: &[f64]) -> f64 {
    let len = ps.len();
    // i枚のコインを投げたときに表がj枚の確率
    let mut dp = vec![vec![0.0; len + 1]; len + 1];

    dp[0][0] = 1.0;

    for i in 0..len {
        for j in 0..=i {
            // dp[i + 1][j] には複数の前状態から確率が流れ込む。
            // 例:
            //   dp[i][j]     から裏が出る -> dp[i + 1][j]
            //   dp[i][j - 1] から表が出る -> dp[i + 1][j]
            // そのため代入ではなく加算する。
            dp[i + 1][j] += dp[i][j] * (1.0 - ps[i]);
            dp[i + 1][j + 1] += dp[i][j] * ps[i];
        }
    }
    let mut result = 0.0;
    for n in ((len / 2) + 1)..=len {
        result += dp[len][n];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let ps = vec![0.3, 0.6, 0.8];
        assert_eq!(resolve(&ps), 0.612);
    }

    #[test]
    fn sample_02() {
        let ps = vec![0.5];
        assert_eq!(resolve(&ps), 0.5);
    }

    #[test]
    fn sample_03() {
        let ps = vec![0.42, 0.01, 0.42, 0.99, 0.42];
        assert_eq!(resolve(&ps), 0.3821815872);
    }
}

fn main() {
    println!("Hello, world!");
}
