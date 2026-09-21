// https://atcoder.jp/contests/abc353/tasks/abc353_d

// f(x, y) = xとyの十進表記をこの順に連結した整数 (例: f(3, 14) = 314)
// 全ペア (i < j) の f(A_i, A_j) の総和を 998244353 で割った余りを求める
// (Cと違い、今回は総和そのものをmodで出力する)
// N <= 2*10^5 なので全ペア O(N^2) は不可
// ヒント: f(x, y) = x * 10^(yの桁数) + y と分解できる。
// Cで使った「寄与で数える」を、左側としての寄与と右側としての寄与に分けて考える。
// 総和は巨大になるので、足すたび・掛けるたびに % MOD を挟むこと
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
        assert_eq!(resolve(&[3, 14, 15]), 2044);
    }

    #[test]
    fn sample_02() {
        assert_eq!(
            resolve(&[1001, 5, 1000000, 1000000000, 100000]),
            625549048
        );
    }
}
