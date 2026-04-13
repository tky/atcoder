fn resolve(xs: &[usize], k: usize) -> usize {
    // dp[i][j]: 先頭から i 人で、飴をちょうど j 個配る方法数
    let mut dp = vec![vec![0; k + 1]; xs.len() + 1];
    dp[0][0] = 1;

    for i in 1..=xs.len() {
        for j in 0..=k {
            for l in 0..=xs[i - 1] {
                if l > j {
                    break;
                }
                dp[i][j] += dp[i - 1][j - l];
            }
        }
    }
    dp[xs.len()][k]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&vec![1, 2, 3], 4), 5);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&vec![9], 10), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve(&vec![0, 0], 0), 1);
    }
}
