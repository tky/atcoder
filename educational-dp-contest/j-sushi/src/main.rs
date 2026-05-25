/**
 * dp[i][j][k]: 全部食べ終わるまでの回数の期待値
 * i: 1個寿司のある皿の枚数
 * j: 2個寿司のある皿の枚数
 * k: 3個寿司のある皿の枚数
 *
 * 皿の枚数をNとすると
 * dp[i][0][0]の場合、空の皿を引く確率 (N-k) / Nでその場合はまた同じ状態に戻るので
 * dp[i][0][0] =  1 + ハズレたときの残り回数の期待値 + あたったときの残り回数の期待値
 *             = 1 + ((N-i) / N) * dp[i][0][0] + (i/N) * dp[i-1][0][0]
 * dp[i][0][0]を左辺に移行して、dp[i][0][0]で囲んで、、という式変形で
 * dp[i][0][0] = N/i + dp[i-1][0][0]
 * dp[i][0][0] =  今あるi枚の皿のどれかを引くまでに平均N/i回 + 1枚減った残りの回数 という式になっている
 *
 * なお、
 * p = 1回あたりの当たる確率
 * E = 当たるまでに必要な平均回数
 * この時
 * p = 1 / E
 * なので、逆数を取ると
 * E = 1 / p
 * となり、N/iが当たり皿を1枚引くまでの平均回数ということになる
 *
 * 以下
 * dp[0][j][0] = 1 + (N-j) / N * dp[0][j][0] + K/n * dp[1][j-1][0]
 *             = N/k + dp[1][j-1][0]
 * dp[0][0][k] = N/k + dp[0][1][k-1]
 *
 * 一般化すると
 * dp[i][j][k] = 1 + ハズレた場合の残り回数 + 1個の皿の時の残り回数 + 2個の皿の時の残り回数 + 3個の皿の時の残り回数
 *             = 1 + ((N - (i + j + k)) / N) dp[i][j][k] + (i/N)dp[i-1][j][k] + (j/N) dp[i + 1][j-1][k] + (k/N) dp[i][j+1][k-1]
 *
 * よって
 * dp[i][j][k] = (N + i dp[i-1][j][k] + j dp[i+1][j-1][k] + k dp[i][j+1][k-1]) / (i + j + k)
 */

fn resolve(ds: &[usize]) -> f64 {
    let c1 = ds.iter().filter(|&&d| d == 1).count();
    let c2 = ds.iter().filter(|&&d| d == 2).count();
    let c3 = ds.iter().filter(|&&d| d == 3).count();

    let n = ds.len();

    let mut dp = vec![vec![vec![-1.0; n + 1]; n + 1]; n + 1];

    dfs(c1, c2, c3, n, &mut dp)
}

fn dfs(i: usize, j: usize, k: usize, n: usize, dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if i == 0 && j == 0 && k == 0 {
        return 0.0;
    }
    if dp[i][j][k] >= 0.0 {
        return dp[i][j][k];
    }
    let s = (i + j + k) as f64;
    let mut res = n as f64 / s;

    if i > 0 {
        res += (i as f64 / s) * dfs(i - 1, j, k, n, dp);
    }
    if j > 0 {
        res += (j as f64 / s) * dfs(i + 1, j - 1, k, n, dp);
    }
    if k > 0 {
        res += (k as f64 / s) * dfs(i, j + 1, k - 1, n, dp);
    }
    dp[i][j][k] = res;
    res
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_almost_eq(actual: f64, expected: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff < 1e-9,
            "actual = {}, expected = {}, diff = {}",
            actual,
            expected,
            diff
        );
    }

    #[test]
    fn sample_01() {
        let ds = vec![1, 1, 1];
        assert_almost_eq(resolve(&ds), 5.5);
    }

    #[test]
    fn sample_02() {
        let ds = vec![3];
        assert_almost_eq(resolve(&ds), 3.0);
    }

    #[test]
    fn sample_03() {
        let ds = vec![1, 2];
        assert_almost_eq(resolve(&ds), 4.5);
    }

    #[test]
    fn one_plate_one_sushi() {
        let ds = vec![1];
        assert_almost_eq(resolve(&ds), 1.0);
    }

    #[test]
    fn one_plate_two_sushi() {
        let ds = vec![2];
        assert_almost_eq(resolve(&ds), 2.0);
    }

    #[test]
    fn two_plates_one_sushi_each() {
        let ds = vec![1, 1];
        assert_almost_eq(resolve(&ds), 3.0);
    }

    #[test]
    fn two_plates_all_three_sushi() {
        let ds = vec![3, 3];
        assert_almost_eq(resolve(&ds), 7.875);
    }
}
