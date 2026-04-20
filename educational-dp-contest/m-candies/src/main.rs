fn resolve(xs: &[usize], k: usize) -> usize {
    // dp[i][j]: 先頭から i 人で、飴をちょうど j 個配る方法数
    let mut dp = vec![vec![0; k + 1]; xs.len() + 1];
    dp[0][0] = 1;

    for i in 1..=xs.len() {
        // dp[i][j] = for (l = 0 to min(xs[i-1], j) Sum dp[i-1][j-l]
        //
        // dpのiは人数を表している
        // dp[1][j] 先頭から1人で使った状態, dp[2][j] 先頭から2人で使った場合
        // 一方xsは0-indexなので1人目の上限はxs[0], 2人目の上限はxs[1]
        // dpのiとxsのiでindexが一つずれているように見える
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

fn resolve2(xs: &[usize], k: usize) -> usize {
    // dp[i][j]: 先頭から i 人で、飴をちょうど j 個配る方法数
    let mut dp = vec![vec![0; k + 1]; xs.len() + 1];
    dp[0][0] = 1;

    for i in 1..=xs.len() {
        for j in 0..=k {
            let mut prefix = vec![0; k + 2];
            for j in 0..=k {
                prefix[j + 1] = prefix[j] + dp[i - 1][j];
            }
            for j in 0..=k {
                // dp[i][j] = prefix[j+1] - prefix[max(0, j-xs[i])]
                let left = if j >= xs[i - 1] { j - xs[i - 1] } else { 0 };
                dp[i][j] = prefix[j + 1] - prefix[left];
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
        assert_eq!(resolve2(&vec![1, 2, 3], 4), 5);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&vec![9], 10), 0);
        assert_eq!(resolve2(&vec![9], 10), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve(&vec![0, 0], 0), 1);
        assert_eq!(resolve2(&vec![0, 0], 0), 1);
    }
}
