// n: サイコロを振る回数
fn resolve(n: usize, d: usize) -> f64 {
    if let Some((count_2, count_3, count_5)) = prime_factors(d) {
        // dp[i][x][y][z]: i回振った後で、2がx個、3がy個、5がz個
        // （ただし必要数を超えたら count_2, count_3, count_5 で打ち止め）
        // 揃っている確率
        let mut dp = vec![vec![vec![vec![0.0; count_5 + 1]; count_3 + 1]; count_2 + 1]; n + 1];
        dp[0][0][0][0] = 1.0;

        for i in 0..n {
            for x in 0..=count_2 {
                for y in 0..=count_3 {
                    for z in 0..=count_5 {
                        // 1が出る
                        dp[i + 1][x][y][z] += dp[i][x][y][z] / 6.0;
                        // 2が出る
                        dp[i + 1][count_2.min(x + 1)][y][z] += dp[i][x][y][z] / 6.0;
                        // 3が出る
                        dp[i + 1][x][count_3.min(y + 1)][z] += dp[i][x][y][z] / 6.0;
                        // 4が出る
                        dp[i + 1][count_2.min(x + 2)][y][z] += dp[i][x][y][z] / 6.0;
                        // 5が出る
                        dp[i + 1][x][y][count_5.min(z + 1)] += dp[i][x][y][z] / 6.0;
                        // 6が出る
                        dp[i + 1][count_2.min(x + 1)][count_3.min(y + 1)][z] +=
                            dp[i][x][y][z] / 6.0;
                    }
                }
            }
        }
        dp[n][count_2][count_3][count_5]
    } else {
        0.0
    }
}

fn prime_factors(n: usize) -> Option<(usize, usize, usize)> {
    let mut count_2 = 0;
    let mut count_3 = 0;
    let mut count_5 = 0;

    let mut num = n;

    while num % 2 == 0 {
        count_2 += 1;
        num /= 2;
    }

    while num % 3 == 0 {
        count_3 += 1;
        num /= 3;
    }

    while num % 5 == 0 {
        count_5 += 1;
        num /= 5;
    }

    if num != 1 {
        None
    } else {
        Some((count_2, count_3, count_5))
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn assert_almost_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < EPS,
            "actual = {}, expected = {}",
            actual,
            expected
        );
    }

    #[test]
    fn sample_1() {
        assert_almost_eq(resolve(2, 6), 15.0 / 36.0);
    }

    #[test]
    fn sample_2() {
        assert_almost_eq(resolve(1, 2), 0.5);
    }

    #[test]
    fn sample_3() {
        assert_almost_eq(resolve(3, 2), 0.875);
    }

    #[test]
    fn d_is_one() {
        assert_almost_eq(resolve(1, 1), 1.0);
        assert_almost_eq(resolve(5, 1), 1.0);
        assert_almost_eq(resolve(100, 1), 1.0);
    }

    #[test]
    fn impossible_due_to_other_prime_factor() {
        assert_almost_eq(resolve(1, 7), 0.0);
        assert_almost_eq(resolve(10, 7), 0.0);
        assert_almost_eq(resolve(3, 14), 0.0); // 14 = 2 * 7
    }

    #[test]
    fn one_roll_basic_cases() {
        assert_almost_eq(resolve(1, 2), 3.0 / 6.0); // 2,4,6
        assert_almost_eq(resolve(1, 3), 2.0 / 6.0); // 3,6
        assert_almost_eq(resolve(1, 4), 1.0 / 6.0); // 4
        assert_almost_eq(resolve(1, 5), 1.0 / 6.0); // 5
        assert_almost_eq(resolve(1, 6), 1.0 / 6.0); // 6
    }

    #[test]
    fn two_roll_basic_cases() {
        assert_almost_eq(resolve(2, 2), 1.0 - (3.0 / 6.0) * (3.0 / 6.0));
        assert_almost_eq(resolve(2, 3), 1.0 - (4.0 / 6.0) * (4.0 / 6.0));
        assert_almost_eq(resolve(2, 5), 1.0 - (5.0 / 6.0) * (5.0 / 6.0));
    }

    #[test]
    fn exact_small_enumeration_cases() {
        assert_almost_eq(resolve(2, 4), 15.0 / 36.0);
        assert_almost_eq(resolve(2, 9), 4.0 / 36.0);
        assert_almost_eq(resolve(2, 10), 6.0 / 36.0);
    }

    #[test]
    fn monotonic_in_n_for_same_d() {
        let p1 = resolve(1, 6);
        let p2 = resolve(2, 6);
        let p3 = resolve(3, 6);
        assert!(p1 <= p2 + EPS);
        assert!(p2 <= p3 + EPS);
    }

    #[test]
    fn probability_is_in_valid_range() {
        let test_cases = [(1, 1), (1, 2), (2, 6), (3, 10), (10, 360), (20, 7)];

        for (n, d) in test_cases {
            let ans = resolve(n, d);
            assert!(
                (0.0 - EPS..=1.0 + EPS).contains(&ans),
                "n = {}, d = {}, ans = {}",
                n,
                d,
                ans
            );
        }
    }
}
