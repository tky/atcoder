fn chmax<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn lcs(s: &str, t: &str) -> String {
    let ss = s.chars().collect::<Vec<char>>();
    let ts = t.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; ts.len() + 1]; ss.len() + 1];

    for i in 1..=ss.len() {
        for j in 1..=ts.len() {
            // S[i] = T[j]なのでLCSを伸ばせる
            if ss[i - 1] == ts[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            }
            // S[i]を使わない場合
            // S[1..i-1]とT[1..j]の問題となり、dp[i-1][j]が答えとなる
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            // T[j]を使わない場合
            // S[1..i]とT[1..j-1]の問題となる
            dp[i][j] = dp[i][j].max(dp[i][j - 1]);
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

        // dp[i-1][j]はss[i-1]を捨てた場合の最適値
        // dp[i][j-1]はts[j-1]を捨てた場合の最適値
        // dp[i-1][j] > dp[i][j-1]ということはss[i-1]を捨てたほうが長いLCSをつくれる
        // ss[i-1] を捨てる、つまり i を 1 減らす
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        // それ以外のケースではts[j-1]を捨てる
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
