// https://atcoder.jp/contests/abc330/tasks/abc330_c

// D <= 2*10^12
// ヒント: x を全探索(x^2 <= D の範囲なので高々 ~1.42*10^6 通り)。
// x を固定すると y^2 を D - x^2 に近づけたい。最適な y の候補は
// 「√(D - x^2) の切り捨てと切り上げ」の2つだけ(212Cの「境界の両隣」と同じ考え方)
fn resolve(d: usize) -> usize {
    let n = (d as f64).sqrt() as usize + 1;
    let mut vs = vec![0usize; n + 1];

    for i in 1..=n {
        vs[i] = i * i;
    }

    let mut ans = usize::MAX;

    for &x in vs.iter() {
        let y = x.abs_diff(d);
        let pos = binary_search(&vs, y);
        if pos < vs.len() {
            ans = ans.min((x + vs[pos]).abs_diff(d));
        }
        if pos > 0 {
            ans = ans.min((x + vs[pos - 1]).abs_diff(d));
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

    #[test]
    fn sample_04() {
        assert_eq!(resolve(1), 0); // D が平方数 (0²+1²)
        assert_eq!(resolve(2), 0); // 1²+1²、ぴったり一致の見逃し検出
    }

    #[test]
    fn sample_05() {
        assert_eq!(resolve(22), 2); // floor側が正解になるケース
    }
}
