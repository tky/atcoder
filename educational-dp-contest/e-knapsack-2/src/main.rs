use std::usize;

fn resolve(ws: &[usize], vs: &[usize], total: usize) -> usize {
    let len = ws.len();
    let max_v = vs.iter().sum();
    // dp[i][j]
    // 品物iまでで価値ちょうど価値vを作るときの最小の重さ
    // 先ほど違い、価重さが10^9なので価値のdpにする必要がある
    let mut dp = vec![vec![usize::MAX / 4; max_v + 1]; len + 1];

    dp[0][0] = 0;

    for i in 0..len {
        let w = ws[i];
        let v = vs[i];
        for j in 0..max_v {
            // 使わない場合で初期化
            dp[i + 1][j] = dp[i][j];
            //　使える場合は緩和
            if j >= v {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j - v] + w);
            }
        }
    }
    let mut result = 0;

    for i in 1..=max_v {
        if dp[len][i] <= total {
            result = i;
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_0() {
        let result = resolve(&[3, 4, 5], &[30, 50, 60], 8);
        assert_eq!(result, 90);
    }
}
