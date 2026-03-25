// https://atcoder.jp/contests/tdpc/tasks/tdpc_contest

fn resolve(ps: &[usize]) -> usize {
    let max: usize = ps.iter().sum();
    let mut dp = vec![false; max + 1];

    dp[0] = true;

    for &p in ps {
        for i in (p..=max).rev() {
            if dp[i - p] {
                dp[i] = true;
            }
        }
    }

    dp.iter().filter(|&p| *p).count()
}

#[cfg(test)]
mod tests {
    use super::resolve;
    use std::collections::HashSet;

    #[test]
    fn sample1() {
        let ps = vec![2, 3];
        assert_eq!(resolve(&ps), 4); // 0, 2, 3, 5
    }

    #[test]
    fn sample2() {
        let ps = vec![2, 3, 5];
        assert_eq!(resolve(&ps), 7); // 0, 2, 3, 5, 7, 8, 10
    }

    #[test]
    fn single_problem() {
        let ps = vec![7];
        assert_eq!(resolve(&ps), 2); // 0, 7
    }

    #[test]
    fn duplicate_scores() {
        let ps = vec![5, 5];
        assert_eq!(resolve(&ps), 3); // 0, 5, 10
    }

    #[test]
    fn all_same_scores() {
        let ps = vec![1, 1, 1, 1];
        assert_eq!(resolve(&ps), 5); // 0,1,2,3,4
    }

    #[test]
    fn cannot_reuse_same_problem_infinitely() {
        let ps = vec![3];
        assert_eq!(resolve(&ps), 2); // 0, 3 only
    }

    #[test]
    fn zero_score_included() {
        let ps = vec![4, 6];
        assert_eq!(resolve(&ps), 4); // 0, 4, 6, 10
    }

    #[test]
    fn brute_force_check_small_case() {
        let ps = vec![1, 2, 4, 7];
        let expected = brute_force(&ps);
        assert_eq!(resolve(&ps), expected.len());
    }

    #[test]
    fn brute_force_check_another_small_case() {
        let ps = vec![3, 5, 8, 14];
        let expected = brute_force(&ps);
        assert_eq!(resolve(&ps), expected.len());
    }

    fn brute_force(ps: &[usize]) -> HashSet<usize> {
        let n = ps.len();
        let mut set = HashSet::new();

        for mask in 0..(1usize << n) {
            let mut sum = 0;
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    sum += ps[i];
                }
            }
            set.insert(sum);
        }

        set
    }
}

fn main() {
    println!("Hello, world!");
}
