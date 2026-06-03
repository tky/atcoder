const MOD: usize = 1_000_000_007;

fn resolve(a: &[&[usize]]) -> usize {
    let n = a.len();

    // dp[s]:
    //   集合 s に含まれる女性たちを使って、
    //   先頭から popcount(s) 人の男性を割り当てる方法数
    //
    // s は bit で集合を表す。
    //
    // 例:
    //   s = 0b0101 なら、女性 0 と女性 2 を使った状態。
    //
    // popcount(s) 人の女性を使っているので、
    // すでに同じ人数の男性を割り当て済み。
    // したがって、次に割り当てる男性は popcount(s) 番目。
    let mut dp = vec![0_usize; 1_usize << n];
    dp[0] = 1;

    for s in 0_usize..(1_usize << n) {
        // 次に割り当てる男性
        let i = s.count_ones() as usize;

        if i >= n {
            continue;
        }

        for j in 0..n {
            // 女性 j がまだ使われておらず、
            // 男性 i と女性 j の相性が良いなら割り当てられる。
            //
            // (s & (1_usize << j)) == 0
            //   -> s の j bit 目が 0
            //   -> 女性 j はまだ集合 s に含まれていない
            //   -> まだ使える
            //
            // a[i][j] == 1
            //   -> 男性 i と女性 j は相性が良い
            //   -> ペアにできる
            if (s & (1_usize << j)) == 0 && a[i][j] == 1 {
                // s:
                //   今までに使った女性集合
                //
                // next:
                //   s に女性 j を追加した集合
                let next = s | (1_usize << j);

                // dp[s]:
                //   状態 s まで来る方法数
                //
                // 状態 s から next に遷移できるので、
                // 「s まで来る方法数」を「next に来る方法数」に足す。
                //
                // これは「今の状態までの作り方を、次の状態へ流す」
                // 配る DP の書き方。
                dp[next] += dp[s];
                dp[next] %= MOD;
            }
        }
    }

    dp[(1_usize << n) - 1]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let a: &[&[usize]] = &[&[0, 1], &[1, 0]];
        assert_eq!(resolve(a), 1);
    }

    #[test]
    fn sample_all_one_2x2() {
        let a: &[&[usize]] = &[&[1, 1], &[1, 1]];
        assert_eq!(resolve(a), 2);
    }

    #[test]
    fn sample_02() {
        let a: &[&[usize]] = &[&[0, 1], &[0, 1]];
        assert_eq!(resolve(a), 0);
    }

    #[test]
    fn sample_03() {
        let a: &[&[usize]] = &[&[1]];
        assert_eq!(resolve(a), 1);
    }

    #[test]
    fn sample_04() {
        let a: &[&[usize]] = &[&[0]];
        assert_eq!(resolve(a), 0);
    }

    #[test]
    fn sample_05() {
        let a: &[&[usize]] = &[&[1, 1], &[1, 1]];
        assert_eq!(resolve(a), 2);
    }

    #[test]
    fn sample_06() {
        let a: &[&[usize]] = &[&[1, 0], &[1, 0]];
        assert_eq!(resolve(a), 0);
    }

    #[test]
    fn sample_07() {
        let a: &[&[usize]] = &[&[1, 1, 0], &[1, 0, 1], &[0, 1, 1]];
        assert_eq!(resolve(a), 2);
    }
}

fn main() {
    println!("Hello, world!");
}
