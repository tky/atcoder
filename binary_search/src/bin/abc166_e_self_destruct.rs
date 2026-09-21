// https://atcoder.jp/contests/abc166/tasks/abc166_e

// N人(1-indexed)から2人選ぶ。条件: |i - j| = A_i + A_j
// を満たすペア (i < j) の個数を求める
// N <= 2*10^5 なので全ペア O(N^2) は不可
// ヒント: i < j なら |i - j| = j - i。条件式を移項して
// 「iだけの式」と「jだけの式」に分離できないか?
// 分離できれば「値が一致する組の個数」を数える問題になる
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
        assert_eq!(resolve(&[2, 3, 3, 1, 3, 1]), 3);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[5, 2, 4, 2, 8, 8]), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(&[
                3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8,
                3, 2, 7, 9, 5
            ]),
            22
        );
    }
}
