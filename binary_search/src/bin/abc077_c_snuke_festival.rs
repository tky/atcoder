// https://atcoder.jp/contests/abc077/tasks/arc084_a

// 祭壇: 上パーツA_i / 中パーツB_j / 下パーツC_k から1つずつ選び、
// A_i < B_j < C_k を満たす組み合わせの総数を求める
// N <= 10^5 なので全探索 O(N^3) は不可。何を固定して何を二分探索するか?
fn resolve(tops: &[usize], middles: &[usize], bottoms: &[usize]) -> usize {
    let mut tops = tops.to_vec();
    let mut middles = middles.to_vec();
    let mut bottoms = bottoms.to_vec();

    tops.sort_unstable();
    middles.sort_unstable();
    bottoms.sort_unstable();

    let mut ans = 0;

    for middle in middles {
        let t = binary_search(&tops, middle);
        let b = binary_search(&bottoms, middle + 1);
        ans += (t) * (b);
    }

    ans
}

/// vsのv以上の値が初めてでてくるindexを返す
///
/// ```text
/// let index = binary_search(vs, v);
/// vs[0..index]のすべての値はv未満
/// vs[index..]のすべての値はv以上となる
/// ```
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
        assert_eq!(resolve(&[1, 5], &[2, 4], &[3, 6]), 3);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[1, 1, 1], &[2, 2, 2], &[3, 3, 3]), 27);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &[3, 14, 159, 2, 6, 53],
                &[58, 9, 79, 323, 84, 6],
                &[2643, 383, 2, 79, 50, 288]
            ),
            87
        );
    }

    #[test]
    fn sample_04() {
        assert_eq!(resolve(&[1, 5], &[6], &[3, 6]), 0);
    }

    #[test]
    fn sample_05() {
        assert_eq!(resolve(&[2, 5], &[1], &[3, 6]), 0);
    }

    #[test]
    fn sample_06() {
        assert_eq!(resolve(&[2], &[2], &[5]), 0); // a < b 側の等号(2 < 2 は不成立)
    }
}
