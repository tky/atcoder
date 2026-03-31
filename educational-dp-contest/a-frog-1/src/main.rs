// https://atcoder.jp/contests/dp/tasks/dp_a

fn resolve(hs: &[usize]) -> usize {
    let len = hs.len();
    // dp[i]: 足場iまでたどり着いた時のコストの総和の最小値
    let mut dp = vec![usize::MAX; len];

    dp[0] = 0;

    for i in 0..(len - 1) {
        // i+1へjump
        dp[i + 1] = dp[i + 1].min(dp[i] + hs[i + 1].abs_diff(hs[i]));
        // i+2へjump
        if i + 2 < len {
            dp[i + 2] = dp[i + 2].min(dp[i] + hs[i + 2].abs_diff(hs[i]));
        }
    }
    dp[len - 1]
}

fn resolve2(hs: &[usize]) -> usize {
    let len = hs.len();

    let mut prev2 = 0;
    let mut prev1 = hs[1].abs_diff(hs[0]);

    for i in 2..len {
        let cur = (prev1 + hs[i].abs_diff(hs[i - 1])).min(prev2 + hs[i].abs_diff(hs[i - 2]));
        prev2 = prev1;
        prev1 = cur;
    }
    prev1
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[10, 30, 40, 20]), 30);
        assert_eq!(resolve2(&[10, 30, 40, 20]), 30);
        assert_eq!(resolve(&[10, 10]), 0);
        assert_eq!(resolve2(&[10, 10]), 0);
        assert_eq!(resolve(&[30, 10, 60, 10, 50]), 40);
        assert_eq!(resolve2(&[30, 10, 60, 10, 50]), 40);
    }
}
