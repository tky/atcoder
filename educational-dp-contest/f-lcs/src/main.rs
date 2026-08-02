fn lcs(s: &str, t: &str) -> String {
    let ss = s.chars().collect::<Vec<char>>();
    let ts = t.chars().collect::<Vec<char>>();

    // s.len() / t.len() よりもss.len() / ts.len()を使う方が安全
    // この問題は英小文字なので一致するが一般のUTF-8では一致しないことがある
    let mut dp = vec![vec![0; ts.len() + 1]; ss.len() + 1];

    for i in 1..=ss.len() {
        for j in 1..=ts.len() {
            if ss[i - 1] == ts[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut i = ss.len();
    let mut j = ts.len();

    let mut ans = Vec::new();

    while i > 0 && j > 0 {
        if ss[i - 1] == ts[j - 1] {
            ans.push(ss[i - 1]);
            i -= 1;
            j -= 1;
        // iを減らした方がdp(LCSの長さ)が大きくなるならiを減らす
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        // そうでなければ、dp[i][j-1] 側へ戻る
        // 同値の場合はどちらへ戻ってもよい
        } else {
            j -= 1;
        }
    }

    ans.reverse();

    ans.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_length() {
        assert_eq!(lcs("axyb", "abyxb"), "ayb");
        assert_eq!(lcs("aa", "xayaz"), "aa");
        assert_eq!(lcs("abc", "ab"), "ab");
    }
}
fn main() {
    println!("Hello, world!");
}
