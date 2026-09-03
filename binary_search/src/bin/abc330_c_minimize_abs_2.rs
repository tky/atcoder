// https://atcoder.jp/contests/abc330/tasks/abc330_c

// 非負整数 x, y を選んで |x^2 + y^2 - D| の最小値を求める
// D <= 2*10^12
// ヒント: x を全探索(x^2 <= D の範囲なので高々 ~1.42*10^6 通り)。
// x を固定すると y^2 を D - x^2 に近づけたい。最適な y の候補は
// 「√(D - x^2) の切り捨てと切り上げ」の2つだけ(212Cの「境界の両隣」と同じ考え方)
fn resolve(d: usize) -> usize {
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
        assert_eq!(resolve(21), 1);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(998244353), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve(264428617), 32);
    }
}
