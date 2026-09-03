// https://atcoder.jp/contests/abc248/tasks/abc248_d

// 各クエリ (l, r, x): A_l..=A_r の中に x がいくつあるか (l, r は1-indexed)
// N, Q <= 2*10^5 なのでクエリごとに範囲を走査する O(NQ) は不可
// ヒント: 値ごとに「その値が現れるindexのリスト」を作っておくと、
// 区間内の個数 = 境界2つの二分探索の差 で求まる
fn resolve(a: &[usize], queries: &[(usize, usize, usize)]) -> Vec<usize> {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(
            resolve(
                &[3, 1, 4, 1, 5],
                &[(1, 5, 1), (2, 4, 3), (1, 5, 2), (1, 3, 3)]
            ),
            vec![2, 0, 0, 1]
        );
    }
}
