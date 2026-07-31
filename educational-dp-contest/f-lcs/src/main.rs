fn lcs(s: &str, t: &str) -> String {
    let ss = s.chars().collect::<Vec<char>>();
    let ts = t.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..s.len() {
        for j in 1..t.len() {
            if ss[i - 1] == ts[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut i = s.len();
    let mut j = t.len();

    let mut ans = Vec::new();

    while i > 0 && j > 0 {
        if ss[i - 1] == ts[j - 1] {
            ans.push(ss[i - 1]);
            i -= 1;
            j -= 1;
        } else {
            if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
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
    }
}
fn main() {
    println!("Hello, world!");
}
