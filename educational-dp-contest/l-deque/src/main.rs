fn resolve(xs: &[usize]) -> i64 {
    let n = xs.len();
    // dp[i][j]:= 区間xs[i..j]が残っている状態で、今から手番の人が最終的に得られる得点差の最大値
    // Y - X を最大化する = X - Y を最小化する
    // なのでdpを手番ごとに分ける必要はない
    let mut dp = vec![vec![0_i64; n + 1]; n + 1];

    // dp[i][j] は区間 xs[i..j] の答え。
    // 遷移では dp[i + 1][j] と dp[i][j - 1] を参照する。
    //
    // dp[i + 1][j] も dp[i][j - 1] も、
    // dp[i][j] より区間の長さが 1 小さい。
    //
    // したがって、短い区間から順に計算する。
    //
    // len = 0:
    //   (0,0), (1,1), (2,2), ...
    //
    // len = 1:
    //   (0,1), (1,2), (2,3), ...
    //
    // len = 2:
    //   (0,2), (1,3), (2,4), ...
    //
    // len = 3:
    //   (0,3), (1,4), (2,5), ...
    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len;
            // 左からとった場合と右からとった場合
            dp[i][j] = (xs[i] as i64 - dp[i + 1][j]).max(xs[j - 1] as i64 - dp[i][j - 1]);
        }
    }
    dp[0][n]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_0() {
        assert_eq!(resolve(&vec![10, 80, 90, 30]), 10);
    }
}
