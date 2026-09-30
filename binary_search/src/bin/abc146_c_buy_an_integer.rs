// https://atcoder.jp/contests/abc146/tasks/abc146_c

// 整数 n (1 <= n <= 10^9) の価格は A*n + B*d(n) (d(n)はnの十進桁数)。
// 所持金X円で買える最大の整数を求める。1つも買えなければ0。
//
// ここからStep 2「答えで二分探索」:
// 探索対象は配列ではなく「答えの候補 n の値域 1..=10^9」そのもの。
// 鍵は単調性 —「nが買える」なら「nより小さい整数も必ず買える」か?
// (価格が n について非減少であることを確認してから使うこと)
// これが成り立つなら、値域は [買える...買える|買えない...買えない] に
// 分かれるので、境界を二分探索で見つけられる。
//
// 注意: A*n + B*d(n) は最大 10^9 * 10^9 + ... = 10^18 超で
// usizeには収まるが、掛け算の途中でXと比較する形に気を付けること
fn resolve(a: usize, b: usize, x: usize) -> usize {
    let mut ok = 0usize;
    let mut ng = 1_000_000_001usize;

    while ok + 1 < ng {
        let mid = (ok + ng) / 2;
        let v = a * mid + b * (mid.to_string().len());
        if v <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(10, 7, 100), 9);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(2, 1, 100000000000), 1000000000);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve(1000000000, 1000000000, 100), 0);
    }

    #[test]
    fn sample_04() {
        assert_eq!(resolve(1234, 56789, 314159265), 254309);
    }
}
