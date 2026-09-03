// https://atcoder.jp/contests/abc172/tasks/abc172_c

// 机A(N冊)と机B(M冊)、どちらも上から順にしか読めない
// 合計K分以内で読める最大冊数を求める
// ヒント: Aから何冊読むかを全探索(累積和で時間を出す)。
// 残り時間で読めるBの冊数は、Bの累積和の上での二分探索で求まる
fn resolve(a: &[usize], b: &[usize], k: usize) -> usize {
    let mut prefa = vec![0; a.len() + 1];
    // prefa[0]に0を入れることで、aから0冊読むという状況を自然に作れる
    for i in 0..a.len() {
        prefa[i + 1] = prefa[i] + a[i];
    }

    let mut prefb = vec![0; b.len() + 1];
    for i in 0..b.len() {
        prefb[i + 1] = prefb[i] + b[i];
    }

    let mut ans = 0;
    for i in 0..prefa.len() {
        if prefa[i] > k {
            continue;
        }
        let remain = k - prefa[i];
        // 先頭の0を除いて探す
        let b_books = binary_search(&prefb[1..], remain + 1);

        ans = ans.max(i + b_books);
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
        assert_eq!(resolve(&[60, 90, 120], &[80, 150, 80, 150], 240), 3);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[60, 90, 120], &[80, 150, 80, 150], 730), 7);
    }

    #[test]
    fn sample_03() {
        let e9 = 1_000_000_000;
        assert_eq!(resolve(&[e9; 5], &[e9; 4], 1), 0);
    }

    #[test]
    fn sample_04() {
        assert_eq!(resolve(&[100], &[1, 1], 2), 2); // Aを1冊も読まず、Bだけ読む
    }
}
