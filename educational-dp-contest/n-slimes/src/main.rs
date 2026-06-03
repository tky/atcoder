fn resolve(xs: &[i64]) -> i64 {
    let len = xs.len();
    let mut dp = vec![vec![0_i64; len + 1]; len + 1];

    // prefix[i]: 先頭から i 個分の合計
    //
    // 例:
    // xs = [10, 20, 30, 40]
    //
    // prefix[0] = 0
    // prefix[1] = 10
    // prefix[2] = 10 + 20 = 30
    // prefix[3] = 10 + 20 + 30 = 60
    // prefix[4] = 10 + 20 + 30 + 40 = 100
    //
    // 区間 xs[i..j] の合計は、
    //
    // prefix[j] - prefix[i]
    //
    // で求められる。
    let mut prefix = vec![0_i64; len + 1];
    for i in 1..=len {
        prefix[i] = prefix[i - 1] + xs[i - 1];
    }

    let inf = i64::MAX / 4;

    // l は区間の長さ。
    //
    // dp[i][j] は短い区間の dp[i][k], dp[k][j] に依存するので、
    // 区間長 l が小さいものから順に計算する。
    for l in 2..=len {
        for i in 0..=len - l {
            let j = i + l;

            // dp[i][j]:
            //   区間 xs[i..j] を 1 つにまとめる最小コスト
            //
            // 区間 xs[i..j] を最後に合体する直前、
            // どこかの k で
            //
            //   xs[i..k] と xs[k..j]
            //
            // の 2 つに分かれている。
            //
            // そのため、i < k < j となる分割点 k を全探索する。
            let mut minimum = inf;

            for k in i + 1..j {
                // k で分割する場合:
                //
                //   左側 xs[i..k] を 1 つにまとめるコスト: dp[i][k]
                //   右側 xs[k..j] を 1 つにまとめるコスト: dp[k][j]
                //
                // この時点では「左右をそれぞれ 1 つにするまでのコスト」だけを見る。
                // 最後に左右を合体するコスト sum(i, j) は、ループ後に足す。
                let cost = dp[i][k] + dp[k][j];
                minimum = minimum.min(cost);
            }

            // 最後に、左右 2 つのスライムを合体する。
            //
            // そのコストは区間 xs[i..j] の総和。
            //
            // この合体コストは分割点 k に依存しない。
            // どこで分けても、
            //
            //   sum(i, k) + sum(k, j) = sum(i, j)
            //
            // になる。
            //
            // そのため、k のループ内では dp[i][k] + dp[k][j] の最小値だけを探し、
            // ループの外で固定コスト sum(i, j) を足す。
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
