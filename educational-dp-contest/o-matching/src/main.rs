fn resolve(a: &[&[usize]]) -> usize {
    let n = a.len();
    // dp[s]: 集合sに含まれる女性たちを使って、先頭からpopcount(s)人の男性をちょうど割り当てる方法数
    // sは2進数のbitでもつ
    // s = 0101なら、女性0、女性2を使った状態
    let mut dp = vec![0usize; 1usize << n];
    dp[0] = 1;
    for s in 0usize..(1usize << n) - 1 {
        // 次の男性
        let i = s.count_ones() as usize;
        for j in 0..n {
            // jを含まない && iとjの相性が良い
            if s & (1usize << j) == 0 && a[i][j] == 1 {
                // s: 今までに使った女性集合
                // next: そこに1人追加した集合
                let next = s | (1usize << j);
                dp[next] += dp[s];
            }
        }
    }
    dp[(1usize << n) - 1]
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
