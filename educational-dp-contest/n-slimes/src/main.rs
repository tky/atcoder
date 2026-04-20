fn resolve(xs: &[usize]) -> usize {
    let len = xs.len();
    let mut dp = vec![vec![0; len + 1]; len + 1];

    let mut prefix = vec![0; len + 1];
    for i in 1..=len {
        prefix[i] = prefix[i - 1] + xs[i - 1];
    }

    for l in 2..=len {
        for i in 0..=len - l {
            let j = i + l;
            let mut minimum = dp[i][i + 1] + dp[i + 1][j];
            for k in i + 1..j {
                if dp[i][k] + dp[k][j] < minimum {
                    minimum = dp[i][k] + dp[k][j];
                }
            }
            dp[i][j] = minimum + prefix[j] - prefix[i];
        }
    }
    dp[0][len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&vec![10, 20, 30, 40]), 190);
    }
    #[test]
    fn sample_02() {
        assert_eq!(resolve(&vec![10, 10, 10, 10, 10]), 120);
    }
}

fn main() {
    println!("Hello, world!");
}
