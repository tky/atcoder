fn resolve(hs: &[usize], k: usize) -> usize {
    let len = hs.len();
    let k = k.min(len);

    // k個前の足場までの累積最小コスト
    let mut prevs = vec![0; k];
    // prevs[0]: 1つ前からの遷移
    // prevs[1]: 2つ前からの遷移
    // ...

    // 初期化
    // 次に i = k を計算できるように、prevs[j] = dp[k - 1 - j] を作る
    //
    // prevs[0]     = dp[k - 1]
    // prevs[1]     = dp[k - 2]
    // ...
    // prevs[k - 2] = dp[1]
    // prevs[k - 1] = dp[0] = 0  <- k個前はスタート地点なので、そこまでの最小コストは0
    //
    // prevs[0] k-1の足場へ行くための最小コスト
    // prevs[1] k-2の足場へ行くための最小コスト
    for i in 0..(k - 1) {
        prevs[i] = hs[k - 1 - i].abs_diff(hs[0]);
    }

    for i in k..len {
        let mut cur = prevs[0] + hs[i].abs_diff(hs[i - 1]);
        for j in 1..k {
            cur = cur.min(prevs[j] + hs[i].abs_diff(hs[i - j - 1]));
        }
        // 次の状態へ進めるために一つずつずらす
        for j in (1..k).rev() {
            prevs[j] = prevs[j - 1];
        }
        prevs[0] = cur;
    }
    prevs[0]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[10, 30, 40, 20], 2), 30);
        assert_eq!(resolve(&[10, 10], 2), 0);
        assert_eq!(resolve(&[30, 10, 60, 10, 50], 2), 40);

        assert_eq!(resolve(&[10, 30, 40, 50, 20], 3), 30);
    }
    #[test]
    fn sample_02_k_is_greater_than_n() {
        assert_eq!(resolve(&[10, 30], 100), 20);
    }

    #[test]
    fn k_is_greater_than_n_another_case() {
        assert_eq!(resolve(&[10, 20, 10], 100), 0);
    }
}
