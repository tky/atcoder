// https://atcoder.jp/contests/abc231/tasks/abc231_c

// 各クエリx: A_i >= x となるiの個数を求める
// ソートして「x以上が初めて現れる位置」を二分探索で求めれば、個数は len - その位置
fn resolve(heights: &[usize], queries: &[usize]) -> Vec<usize> {
    let mut heights = heights.to_vec();
    heights.sort();

    let mut ans = Vec::new();

    for &q in queries {
        let mut left = 0;
        let mut right = heights.len();

        while left < right {
            let mid = (left + right) / 2;
            // heights[0..left]はq未満の集合
            if heights[mid] < q {
                left = mid + 1;
            // heights[right..]はq以上（qを含む)
            } else {
                right = mid;
            }
        }
        ans.push(heights.len() - right);
    }
    ans
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[100, 160, 130], &[120]), vec![2]);
    }

    #[test]
    fn sample_02() {
        assert_eq!(
            resolve(&[1, 2, 3, 4, 5], &[6, 5, 4, 3, 2]),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn edge_cases() {
        assert_eq!(resolve(&[100, 130, 160], &[50]), vec![3]); // 全部 >= q
        assert_eq!(resolve(&[5, 5, 5, 5, 5], &[5, 6]), vec![5, 0]); // 重複 / 全部 < q
    }
}
