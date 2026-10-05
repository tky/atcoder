// https://atcoder.jp/contests/abc034/tasks/abc034_d

// 容器i: 食塩水 w_i グラム、濃度 p_i %(塩の量は w_i * p_i / 100)。
// K個選んで全部混ぜたときの濃度(%)の最大値を求める(誤差 1e-6 程度まで許容)。
//
// Step 4: 実数の二分探索。
// 「濃度 x % 以上にできるか?」を判定問題にする:
//   (Σ塩) / (Σ重さ) * 100 >= x  ⟺  Σ w_i * (p_i - x) >= 0
// つまり各容器のスコアを w_i * (p_i - x) と定義すると、
// 「スコア上位K個の合計が0以上か?」という貪欲チェックに落ちる(判定O(N log N))。
// 単調性: xを上げるほど全スコアが下がる → 可能側は小さいx。
//
// 実数ならではの注意:
// - 「+1トリック」「ok+1 < ng」は使えない(実数に「隣」はない)。
//   終了は「回数固定ループ」(例: 100回で区間は 2^-100 倍まで縮む)が定石
// - スコアは f64。f64 のソートは sort_by(|a, b| b.total_cmp(a)) を使う
fn resolve(containers: &[(usize, usize)], k: usize) -> f64 {
    let mut bottom = 0.0;
    let mut top = 100.0;

    for _ in 0..100 {
        let t = (top + bottom) / 2.0;
        if judge(containers, k, t) {
            bottom = t;
        } else {
            top = t;
        }
    }
    bottom
}

fn judge(containers: &[(usize, usize)], k: usize, t: f64) -> bool {
    let mut vs: Vec<f64> = containers
        .iter()
        .map(|&(w, p)| w as f64 * (p as f64 - t))
        .collect();

    vs.sort_unstable_by(|a, b| b.total_cmp(a));

    let sum: f64 = vs.iter().take(k).sum();
    sum >= 0.0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod judge_tests {
    use std::assert_eq;

    use super::judge;

    #[test]
    fn sample_01() {
        assert_eq!(judge(&[(100, 15), (300, 20), (200, 30)], 2, 24.8), true);
        assert_eq!(judge(&[(100, 15), (300, 20), (200, 30)], 2, 24.9), true);
        assert_eq!(judge(&[(100, 15), (300, 20), (200, 30)], 2, 25.0), true);
        assert_eq!(judge(&[(100, 15), (300, 20), (200, 30)], 2, 25.1), false);
        assert_eq!(judge(&[(100, 15), (300, 20), (200, 30)], 2, 25.2), false);
    }

    #[test]
    fn handmade_01() {
        assert_eq!(judge(&[(1, 10), (2, 20), (3, 30)], 2, 25.9), true);
        assert_eq!(judge(&[(1, 10), (2, 20), (3, 30)], 2, 26.0), true);
        assert_eq!(judge(&[(1, 10), (2, 20), (3, 30)], 2, 26.1), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-6,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn sample_01() {
        assert_close(resolve(&[(100, 15), (300, 20), (200, 30)], 2), 25.0);
    }

    // 以下は全組み合わせ列挙で検算した自作ケース
    #[test]
    fn handmade_01() {
        assert_close(resolve(&[(1, 10), (2, 20), (3, 30)], 2), 26.0);
    }

    #[test]
    fn handmade_02() {
        let c = [(5, 13), (1, 87), (4, 2), (7, 50)];
        assert_close(resolve(&c, 2), 54.625);
        assert_close(resolve(&c, 3), 38.61538461538461);
    }

    #[test]
    fn handmade_edge() {
        // 1個だけ選ぶ / 全部選ぶ / 巨大な重さ + 濃度0の1滴
        assert_close(resolve(&[(10, 50)], 1), 50.0);
        let heavy = [
            (1000000000, 100),
            (1000000000, 100),
            (1000000000, 100),
            (1, 0),
        ];
        assert_close(resolve(&heavy, 4), 99.99999996666666);
    }
}
