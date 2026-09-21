// https://atcoder.jp/contests/abc353/tasks/abc353_c

const MOD: usize = 100_000_000;

// f(x, y) = (x + y) % 10^8 として、全ペア (i < j) の f(A_i, A_j) の総和を求める
// (総和自体を10^8で割るのではない点に注意)
// N <= 3*10^5 なので全ペア O(N^2) は不可
// ヒント: A_i < 10^8 なので x + y < 2*10^8。つまり mod で値が変わるのは
// x + y >= 10^8 のペアだけで、そのとき引かれる量は常にちょうど 10^8。
// 「x + y >= 10^8 になるペアの個数」をどう数えるか?
fn resolve(a: &[usize]) -> usize {
    let mut a = a.to_vec();
    a.sort_unstable();

    let n = a.len();

    let mut sum = a.iter().sum::<usize>() * (n - 1);

    for i in 0..n - 1 {
        let ai = a[i];
        let vs = &a[i + 1..];
        let pos = binary_search(&vs, MOD - ai);
        sum -= MOD * (vs.len() - pos);
    }
    sum
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

// naive implementation
#[allow(dead_code)]
fn resolve1(a: &[usize]) -> usize {
    let n = a.len();

    let mut sum = 0;
    for i in 0..n {
        for j in i + 1..n {
            let f = (a[i] + a[j]) % MOD;
            sum += f;
        }
    }
    sum
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[3, 50000001, 50000002]), 100000012);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[1, 3, 99999999, 99999994, 1000000]), 303999988);
    }

    #[test]
    fn cross_check_with_naive() {
        let mut seed: usize = 42;
        for _ in 0..50 {
            let a: Vec<usize> = (0..30)
                .map(|_| {
                    seed = seed
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    seed % (MOD - 1) + 1 // 1..=10^8-1
                })
                .collect();
            assert_eq!(resolve(&a), resolve1(&a));
        }
    }
}
