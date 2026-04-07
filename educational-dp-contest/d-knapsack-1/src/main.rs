fn resolve(ws: &[usize], vs: &[usize], total: usize) -> usize {
    let len = vs.len();

    // dp[i][j]: = 品物iまでのものから重さ以下の時の価値の総和の最大値
    let mut dp = vec![vec![0; total + 1]; len + 1];

    for i in 0..len {
        let v = vs[i];
        let w = ws[i];
        for j in 1..=total {
            if j >= w {
                dp[i + 1][j] = (dp[i][j - w] + v).max(dp[i][j]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    dp[len][total]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_0() {
        let result = resolve(&[3, 4, 5], &[30, 50, 60], 8);
        assert_eq!(result, 90);
    }
}
