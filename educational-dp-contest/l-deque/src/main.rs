fn resolve(xs: &[usize]) -> i64 {
    let n = xs.len();
    let mut dp = vec![vec![0_i64; n + 1]; n + 1];

    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len;
            // 左からとった場合と右からとった場合
            dp[i][j] = (xs[i] as i64 - dp[i + 1][j]).max(xs[j - 1] as i64 - dp[i][j - 1]);
        }
    }
    dp[0][n]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_0() {
        assert_eq!(resolve(&vec![10, 80, 90, 30]), 10);
    }
}
