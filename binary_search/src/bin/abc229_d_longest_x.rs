// https://atcoder.jp/contests/abc229/tasks/abc229_d

// 'X' と '.' からなる文字列Sの '.' を最大K個 'X' に置き換えて、
// 連続する 'X' の長さの最大値を求める
// |S| <= 2*10^5 なので全区間 O(N^2) は不可
// ヒント: 区間 [l, r) 内の '.' の個数は累積和で O(1)。
// 左端lを固定すると「'.' がK個以下に収まる右端」には単調性があるので二分探索できる
fn resolve(s: &str, k: usize) -> usize {
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
        assert_eq!(resolve("XX...X.X.X.", 2), 5);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve("XXXX", 200000), 4);
    }
}
