fn resolve(target: usize, k: usize, ratings: &[usize]) -> f64 {
    // dp[r][i] := 選手iがrラウンド目に勝ち進む確率
    let mut dp = vec![vec![0.0; ratings.len()]; k + 1];

    for i in 0..ratings.len() {
        dp[0][i] = 1.0;
    }

    // dp[r][i] = dp[r-1][i] * (dp[r-1][j] * win_rate(i, j) for all j in target_index(i, r))
    // iがr-1まで勝つ確率:dp[r-1][i]
    for r in 1..=k {
        for i in 0..ratings.len() {
            let (start, end) = target_index(i, r);
            let mut sum = 0.0;
            for j in start..end {
                // iの対戦あいてjがr-1まで勝つ確率:dp[r-1][j] * iがjに勝つ確率:win_rate(i, j)
                sum += dp[r - 1][j] * win_rate(i, j, ratings);
            }
            dp[r][i] = dp[r - 1][i] * sum;
        }
    }
    dp[k][target]
}

// p番目の人とq番目の人が対戦して、p番目の人が勝つ確率
fn win_rate(p: usize, q: usize, ratings: &[usize]) -> f64 {
    let diff = ratings[q] as f64 - ratings[p] as f64;
    1.0 / (1.0 + 10.0_f64.powf(diff / 400.0))
}

// i番目の人が第kラウンドで当たる相手のインデックス
fn target_index(i: usize, k: usize) -> (usize, usize) {
    // 第kラウンドでは、2^k人が対戦する。つまり、i番目の人は、同じブロックの中の人と当たる。
    // 1ラウンド2人
    // 2ラウンド4人
    // 3ラウンド8人 ...
    let width = 1usize << k; // 2^k
    let half = 1usize << (k - 1); // 2^(k-1)

    // i番目の人が当たる相手の範囲の開始位置
    // 8人、1ラウンド目だとすると(0, 1), (2, 3), (4, 5), (6, 7)
    // 0, 1 => 0
    // 2, 3 => 2
    // のように、i番目の人が当たる相手の範囲の開始位置は、iをwidthで割ってwidthをかけることで求められる。
    let block_start = (i / width) * width;
    // i番目の人が左半分にいるか、右半分にいるか判定するための境界位置
    let mid = block_start + half;

    // 左半分にいるならば、後半と当たる。右半分にいるならば、前半と当たる。
    if i < mid {
        (mid, block_start + width)
    } else {
        (block_start, mid)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) {
        assert!(
            (a - b).abs() <= eps,
            "expected {b}, but got {a} (diff = {})",
            (a - b).abs()
        );
    }

    #[test]
    fn win_rate_is_half_for_same_rating() {
        let ratings = vec![1500, 1500];
        approx_eq(win_rate(0, 1, &ratings), 0.5, 1e-12);
        approx_eq(win_rate(1, 0, &ratings), 0.5, 1e-12);
    }

    #[test]
    fn win_rate_is_complementary() {
        let ratings = vec![2000, 1600];
        let p01 = win_rate(0, 1, &ratings);
        let p10 = win_rate(1, 0, &ratings);
        approx_eq(p01 + p10, 1.0, 1e-12);
    }

    #[test]
    fn target_index_round_1() {
        // 8人トーナメント
        assert_eq!(target_index(0, 1), (1, 2));
        assert_eq!(target_index(1, 1), (0, 1));
        assert_eq!(target_index(2, 1), (3, 4));
        assert_eq!(target_index(3, 1), (2, 3));
        assert_eq!(target_index(4, 1), (5, 6));
        assert_eq!(target_index(5, 1), (4, 5));
        assert_eq!(target_index(6, 1), (7, 8));
        assert_eq!(target_index(7, 1), (6, 7));
    }

    #[test]
    fn target_index_round_2() {
        // 8人トーナメント
        assert_eq!(target_index(0, 2), (2, 4));
        assert_eq!(target_index(1, 2), (2, 4));
        assert_eq!(target_index(2, 2), (0, 2));
        assert_eq!(target_index(3, 2), (0, 2));
        assert_eq!(target_index(4, 2), (6, 8));
        assert_eq!(target_index(5, 2), (6, 8));
        assert_eq!(target_index(6, 2), (4, 6));
        assert_eq!(target_index(7, 2), (4, 6));
    }

    #[test]
    fn target_index_round_3() {
        // 8人トーナメント
        assert_eq!(target_index(0, 3), (4, 8));
        assert_eq!(target_index(1, 3), (4, 8));
        assert_eq!(target_index(2, 3), (4, 8));
        assert_eq!(target_index(3, 3), (4, 8));
        assert_eq!(target_index(4, 3), (0, 4));
        assert_eq!(target_index(5, 3), (0, 4));
        assert_eq!(target_index(6, 3), (0, 4));
        assert_eq!(target_index(7, 3), (0, 4));
    }

    #[test]
    fn resolve_two_players_equal_rating() {
        let ratings = vec![1500, 1500];
        approx_eq(resolve(0, 1, &ratings), 0.5, 1e-12);
        approx_eq(resolve(1, 1, &ratings), 0.5, 1e-12);
    }

    #[test]
    fn resolve_two_players_different_rating() {
        let ratings = vec![2000, 1600];
        let expected0 = win_rate(0, 1, &ratings);
        let expected1 = win_rate(1, 0, &ratings);

        approx_eq(resolve(0, 1, &ratings), expected0, 1e-12);
        approx_eq(resolve(1, 1, &ratings), expected1, 1e-12);
        approx_eq(
            resolve(0, 1, &ratings) + resolve(1, 1, &ratings),
            1.0,
            1e-12,
        );
    }

    #[test]
    fn resolve_four_players_same_rating() {
        let ratings = vec![1500, 1500, 1500, 1500];
        for i in 0..4 {
            approx_eq(resolve(i, 2, &ratings), 0.25, 1e-12);
        }
    }

    #[test]
    fn resolve_eight_players_same_rating() {
        let ratings = vec![1500; 8];
        for i in 0..8 {
            approx_eq(resolve(i, 3, &ratings), 1.0 / 8.0, 1e-12);
        }
    }

    #[test]
    fn sum_of_probabilities_is_one_for_four_players() {
        let ratings = vec![1800, 1700, 1600, 1500];
        let total: f64 = (0..4).map(|i| resolve(i, 2, &ratings)).sum();
        approx_eq(total, 1.0, 1e-10);
    }

    #[test]
    fn sum_of_probabilities_is_one_for_eight_players() {
        let ratings = vec![2100, 1800, 1750, 1600, 1550, 1500, 1450, 1300];
        let total: f64 = (0..8).map(|i| resolve(i, 3, &ratings)).sum();
        approx_eq(total, 1.0, 1e-10);
    }

    #[test]
    fn stronger_player_has_higher_probability_in_two_player_case() {
        let ratings = vec![2200, 1200];
        let p0 = resolve(0, 1, &ratings);
        let p1 = resolve(1, 1, &ratings);
        assert!(p0 > p1, "expected player 0 to have higher win probability");
    }

    #[test]
    fn stronger_players_tend_to_have_higher_probability_in_ordered_case() {
        let ratings = vec![2000, 1900, 1800, 1700];
        let p0 = resolve(0, 2, &ratings);
        let p1 = resolve(1, 2, &ratings);
        let p2 = resolve(2, 2, &ratings);
        let p3 = resolve(3, 2, &ratings);

        assert!(p0 > p1, "expected p0 > p1, but got p0={p0}, p1={p1}");
        assert!(p1 > p2, "expected p1 > p2, but got p1={p1}, p2={p2}");
        assert!(p2 > p3, "expected p2 > p3, but got p2={p2}, p3={p3}");
    }
}
