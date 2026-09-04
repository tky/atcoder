// https://atcoder.jp/contests/abc212/tasks/abc212_c

// 数列Aと数列Bから1つずつ選んだときの |a - b| の最小値を求める
// N, M <= 2*10^5 なので総当たり O(NM) は不可
// ヒント: 片方をソートすると「aに一番近いbの候補」は2つに絞れる
fn resolve(a: &[usize], b: &[usize]) -> usize {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort_unstable();
    b.sort_unstable();

    let mut ans = usize::MAX;

    for i in 0..a.len() {
        let ai = a[i];
        let pos = binary_search(&b, ai);
        if pos < b.len() {
            ans = ans.min(ai.abs_diff(b[pos]));
        }
    }

    for i in 0..b.len() {
        let bi = b[i];
        let pos = binary_search(&a, bi);
        if pos < a.len() {
            ans = ans.min(bi.abs_diff(a[pos]));
        }
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
        assert_eq!(resolve(&[1, 6], &[4, 9]), 2);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[10], &[10]), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(&[82, 76, 82, 82, 71, 70], &[17, 39, 67, 2, 45, 35, 22, 24]),
            3
        );
    }
}
