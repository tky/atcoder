// https://atcoder.jp/contests/abc353/tasks/abc353_c

// f(x, y) = (x + y) % 10^8 として、全ペア (i < j) の f(A_i, A_j) の総和を求める
// (総和自体を10^8で割るのではない点に注意)
// N <= 3*10^5 なので全ペア O(N^2) は不可
// ヒント: A_i < 10^8 なので x + y < 2*10^8。つまり mod で値が変わるのは
// x + y >= 10^8 のペアだけで、そのとき引かれる量は常にちょうど 10^8。
// 「x + y >= 10^8 になるペアの個数」をどう数えるか?
fn resolve(a: &[usize]) -> usize {
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
        assert_eq!(resolve(&[3, 50000001, 50000002]), 100000012);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[1, 3, 99999999, 99999994, 1000000]), 303999988);
    }
}
