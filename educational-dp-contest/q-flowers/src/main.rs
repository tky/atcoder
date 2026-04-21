fn resolve(h: &[u64], aa: &[u64]) -> u64 {
    let len = h.len();
    // dp[i]: 花iを最後に選ぶ時の美しさの総和の最大値
    // dp[i] = a_i + max { dp[j] | j < i, h_j < h_i }
    let mut dp = vec![0u64; len];
    dp[0] = aa[0];

    for i in 1..len {
        let mut max = 0u64;
        for j in 0..i {
            if max < dp[j] && h[j] < h[i] {
                max = dp[j];
            }
        }
        dp[i] = aa[i] + max;
    }
    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_01() {
        assert_eq!(resolve(&vec![3, 1, 4, 2], &vec![10, 20, 30, 40]), 60);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&vec![1], &vec![10]), 10);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &vec![4, 2, 5, 8, 3, 6, 1, 7, 9],
                &vec![6, 8, 8, 4, 6, 3, 5, 7, 5]
            ),
            31
        );
    }
}
fn main() {
    println!("Hello, world!");
}
