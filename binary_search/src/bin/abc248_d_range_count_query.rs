// https://atcoder.jp/contests/abc248/tasks/abc248_d

// 各クエリ (l, r, x): A_l..=A_r の中に x がいくつあるか (l, r は1-indexed)
// N, Q <= 2*10^5 なのでクエリごとに範囲を走査する O(NQ) は不可
// ヒント: 値ごとに「その値が現れるindexのリスト」を作っておくと、
// 区間内の個数 = 境界2つの二分探索の差 で求まる
fn resolve(a: &[usize], queries: &[(usize, usize, usize)]) -> Vec<usize> {
    let mut positions = vec![Vec::new(); a.len() + 1];

    for (i, &v) in a.iter().enumerate() {
        positions[v].push(i + 1);
    }

    let mut ans = Vec::new();

    for &(l, r, x) in queries {
        let left = binary_search(&positions[x], l);
        let right = binary_search(&positions[x], r + 1);
        ans.push(right - left);
    }
    ans
}

fn binary_search(vs: &[usize], v: usize) -> usize {
    let mut left = 0;
    let mut right = vs.len();

    while left < right {
        let mid = (left + right) / 2;

        if vs[mid] < v {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
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
    #[test]
    fn edge_r_boundary() {
        // 区間の右端ちょうどにxがある
        assert_eq!(resolve(&[3, 1, 4, 1, 5], &[(1, 1, 3)]), vec![1]);
    }

    #[test]
    fn edge_x_not_in_a() {
        // xがAに一度も現れない(かつ max(A) より大きい)
        assert_eq!(resolve(&[1, 1], &[(1, 2, 2)]), vec![0]);
    }
}
