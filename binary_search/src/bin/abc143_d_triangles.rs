// https://atcoder.jp/contests/abc143/tasks/abc143_d

// N本の棒から3本選んで三角形を作れる組み合わせの数を求める
// 三角形の成立条件: a <= b <= c なら a + b > c だけ確認すればよい(残り2式は自動で成立)
// N <= 2*10^3 なので O(N^2 log N) までOK。2辺を固定して3辺目の範囲を二分探索
fn resolve(sticks: &[usize]) -> usize {
    let mut sticks = sticks.to_vec();
    sticks.sort_unstable();

    let mut ans = 0;

    let n = sticks.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let pos = binary_search(&sticks[j + 1..], sticks[i] + sticks[j]);
            ans += pos;
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
        assert_eq!(resolve(&[3, 4, 2, 1]), 1);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[1, 1000, 1]), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve(&[218, 786, 704, 233, 645, 728, 389]), 23);
    }
}
