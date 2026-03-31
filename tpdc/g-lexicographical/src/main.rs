// https://atcoder.jp/contests/tdpc/tasks/tdpc_lexicographical
const NONE: u32 = u32::MAX;

fn add_cap(a: u64, b: u64, cap: u64) -> u64 {
    let s = a.saturating_add(b);
    if s > cap { cap } else { s }
}

fn solve(s: &str, k: u64) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();

    // next_pos[i][c] = i 以降で文字 c が最初に現れる位置。なければ NONE。
    let mut next_pos = vec![[NONE; 26]; n + 1];
    for i in (0..n).rev() {
        next_pos[i] = next_pos[i + 1];
        let c = (bytes[i] - b'a') as usize;
        next_pos[i][c] = i as u32;
    }

    // dp[i] = s[i..] から作れる異なる空でない部分列の個数（k で打ち切り）
    let mut dp = vec![0_u64; n + 1];
    for i in (0..n).rev() {
        let mut total = 0_u64;
        for c in 0..26 {
            let j = next_pos[i][c];
            if j == NONE {
                continue;
            }
            let cnt = add_cap(1, dp[j as usize + 1], k);
            total = add_cap(total, cnt, k);
        }
        dp[i] = total;
    }

    if dp[0] < k {
        return "Eel".to_string();
    }

    // K 番目を復元
    let mut rem = k;
    let mut pos = 0usize;
    let mut ans = String::new();

    loop {
        for c in 0..26 {
            let j = next_pos[pos][c];
            if j == NONE {
                continue;
            }

            let cnt = add_cap(1, dp[j as usize + 1], k);

            if rem > cnt {
                rem -= cnt;
                continue;
            }

            ans.push((b'a' + c as u8) as char);

            if rem == 1 {
                return ans;
            }

            rem -= 1;
            pos = j as usize + 1;
            break;
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve("eel", 1), "e");
        assert_eq!(solve("eel", 2), "ee");
        assert_eq!(solve("eel", 3), "eel");
        assert_eq!(solve("eel", 4), "el");
        assert_eq!(solve("eel", 5), "l");
        assert_eq!(solve("eel", 6), "Eel");
    }

    #[test]
    fn abc_all() {
        assert_eq!(solve("abc", 1), "a");
        assert_eq!(solve("abc", 2), "ab");
        assert_eq!(solve("abc", 3), "abc");
        assert_eq!(solve("abc", 4), "ac");
        assert_eq!(solve("abc", 5), "b");
        assert_eq!(solve("abc", 6), "bc");
        assert_eq!(solve("abc", 7), "c");
        assert_eq!(solve("abc", 8), "Eel");
    }

    #[test]
    fn aba_all() {
        assert_eq!(solve("aba", 1), "a");
        assert_eq!(solve("aba", 2), "aa");
        assert_eq!(solve("aba", 3), "ab");
        assert_eq!(solve("aba", 4), "aba");
        assert_eq!(solve("aba", 5), "b");
        assert_eq!(solve("aba", 6), "ba");
        assert_eq!(solve("aba", 7), "Eel");
    }

    #[test]
    fn cba_all() {
        assert_eq!(solve("cba", 1), "a");
        assert_eq!(solve("cba", 2), "b");
        assert_eq!(solve("cba", 3), "ba");
        assert_eq!(solve("cba", 4), "c");
        assert_eq!(solve("cba", 5), "ca");
        assert_eq!(solve("cba", 6), "cb");
        assert_eq!(solve("cba", 7), "cba");
        assert_eq!(solve("cba", 8), "Eel");
    }

    #[test]
    fn repeated_chars() {
        assert_eq!(solve("aaa", 1), "a");
        assert_eq!(solve("aaa", 2), "aa");
        assert_eq!(solve("aaa", 3), "aaa");
        assert_eq!(solve("aaa", 4), "Eel");
    }

    #[test]
    fn single_char() {
        assert_eq!(solve("a", 1), "a");
        assert_eq!(solve("a", 2), "Eel");
    }

    #[test]
    fn large_k_overflow_guard() {
        assert_eq!(solve("a", 1_000_000_000_000_000_000), "Eel");
        assert_eq!(solve("aaa", 1_000_000_000_000_000_000), "Eel");
    }

    #[test]
    fn sample_2_from_statement() {
        assert_eq!(
            solve("deoxyribonucleicacid", 1_000_000_000_000_000_000),
            "Eel"
        );
    }

    #[test]
    fn lexicographical_order_check() {
        // 異なる部分列の辞書順:
        // a, aa, aab, ab, b
        assert_eq!(solve("aab", 1), "a");
        assert_eq!(solve("aab", 2), "aa");
        assert_eq!(solve("aab", 3), "aab");
        assert_eq!(solve("aab", 4), "ab");
        assert_eq!(solve("aab", 5), "b");
        assert_eq!(solve("aab", 6), "Eel");
    }
}
