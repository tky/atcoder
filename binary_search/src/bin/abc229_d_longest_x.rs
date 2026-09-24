// https://atcoder.jp/contests/abc229/tasks/abc229_d

// 'X' と '.' からなる文字列Sの '.' を最大K個 'X' に置き換えて、
// 連続する 'X' の長さの最大値を求める
// |S| <= 2*10^5 なので全区間 O(N^2) は不可
// ヒント: 区間 [l, r) 内の '.' の個数は累積和で O(1)。
// 左端lを固定すると「'.' がK個以下に収まる右端」には単調性があるので二分探索できる
fn resolve(s: &str, k: usize) -> usize {
    let mut ans = 0;

    let chars: Vec<char> = s.chars().collect();

    // dots[r]: [0,r)のdotの数
    // dots[r] - dots[l]が[l, r)がdotの個数になる
    let mut dots = vec![0; chars.len() + 1];

    for i in 0..chars.len() {
        dots[i + 1] = dots[i] + if chars[i] == '.' { 1 } else { 0 }
    }

    for i in 0..chars.len() {
        let pos = binary_search(&dots, dots[i] + k + 1);
        ans = ans.max(pos - 1 - i);
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

fn resolve1(s: &str, k: usize) -> usize {
    let mut ans = 0;

    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let mut tmp = 0;
        let vs = &chars[i..];
        let mut count = 0;

        for j in 0..vs.len() {
            if vs[j] == '.' {
                count += 1;
                if count > k {
                    break;
                }
            }
            tmp += 1;
        }
        ans = ans.max(tmp);
    }
    ans
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

    #[test]
    fn cross_check_with_naive() {
        let mut seed: usize = 42;
        let mut next = move |m: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) % m
        };

        for _ in 0..100 {
            let len = next(30) + 1;
            let k = next(len + 2);
            let s: String = (0..len)
                .map(|_| if next(2) == 0 { 'X' } else { '.' })
                .collect();
            assert_eq!(resolve(&s, k), resolve1(&s, k), "s={s}, k={k}");
        }
    }
}
