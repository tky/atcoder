// https://atcoder.jp/contests/tdpc/tasks/tdpc_semiexp

fn resolve(n: usize, k: usize) -> usize {
    let mut dp = vec![0; k];
    dp[1] = 1;
    for _ in 1..n {
        let sum = dp.iter().sum();
        for j in (1..k).rev() {
            dp[j] = dp[j - 1];
        }
        dp[0] = sum;
    }
    dp[1..].iter().sum()
}

fn resolve2(n: usize, k: usize) -> usize {
    let mut dp = vec![0; k];
    let mut head = 0;
    let mut sum = 1;
    dp[1] = 1;

    for _ in 1..n {
        let old_sum = sum;
        head = (head + k - 1) % k;
        // 更新前の論理dp[k-1]
        let dropped = dp[head];
        dp[head] = old_sum;
        sum = 2 * old_sum - dropped;
    }
    sum - dp[head]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_k_2() {
        assert_eq!(resolve(3, 2), 1);
        assert_eq!(resolve(4, 2), 1);
        assert_eq!(resolve(5, 2), 2);
        assert_eq!(resolve(6, 2), 3);
        assert_eq!(resolve2(3, 2), 1);
        assert_eq!(resolve2(4, 2), 1);
        assert_eq!(resolve2(5, 2), 2);
        assert_eq!(resolve2(6, 2), 3);
    }

    #[test]
    fn small_k_3() {
        assert_eq!(resolve(3, 3), 1);
        assert_eq!(resolve(4, 3), 3);
        assert_eq!(resolve(5, 3), 5);
        assert_eq!(resolve(10, 3), 105);

        assert_eq!(resolve2(3, 3), 1);
        assert_eq!(resolve2(4, 3), 3);
        assert_eq!(resolve2(5, 3), 5);
        assert_eq!(resolve2(10, 3), 105);
    }
}
