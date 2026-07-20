fn lcs(s: &str, t: &str) -> String {
    let ss = s.chars().collect::<Vec<char>>();
    let ts = t.chars().collect::<Vec<char>>();

    // dp の i, j は「位置」ではなく「先頭から何文字使うか」。
    // だから dp[i][j] の末尾文字は S[i-1], T[j-1]。
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            // S[i] = T[j]なのでLCSを伸ばせる
            if ss[i - 1] == ts[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                // S[i]を使わない場合
                // S[1..i-1]とT[1..j]、あるいはS[1..i]とT[1..j-1]の問題となる
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut i = ss.len();
    let mut j = ts.len();
    let mut out: Vec<char> = Vec::new();

    while i > 0 && j > 0 {
        // ss[i-1] = ts[j-1]で、その文字を使った
        // この文字はLCSに含まれる
        if ss[i - 1] == ts[j - 1] {
            out.push(ss[i - 1]);
            i -= 1;
            j -= 1;
        // dp[i-1][j] は、ss[i-1] を使わない場合の LCS 長
        // dp[i][j-1] は、ts[j-1] を使わない場合の LCS 長
        //
        // dp[i-1][j] > dp[i][j-1] なので、
        // ss[i-1] を使わない状態へ戻る方が長い LCS を保てる。
        // よって ss[i-1] を捨てて i を 1 減らす。
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    out.reverse();
    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_length() {
        assert_eq!(lcs("axyb", "abyxb"), "ayb");
        assert_eq!(lcs("aa", "xayaz"), "aa");
    }
}
fn main() {
    println!("Hello, world!");
}
