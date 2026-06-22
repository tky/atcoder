use std::io::{self, Read};

// 最終的なグループ分けを復元する必要はない。
// ただし、最大スコアを得るために、各 mask で
// 「1つのグループ sub」として切り出す候補は全部試す。
//
// score[mask] := mask に含まれるうさぎを 1 グループにしたときの得点
// dp[mask]    := mask に含まれるうさぎを自由にグループ分けしたときの最大得点
fn resolve(a: &[Vec<i64>]) -> i64 {
    let n = a.len();
    let size = 1_usize << n;

    // score[mask]:
    //   mask のうさぎたちを 1 グループにしたときの得点
    let mut score = vec![0_i64; size];

    // 各 mask について、含まれるペア (i, j) を総当たりで調べる。
    // 1つの mask あたり O(N^2)、mask は 2^N 個あるので、
    // 全体では O(2^N * N^2)。
    // N <= 16 なので十分に間に合う。
    for mask in 0..size {
        let mut sum = 0_i64;

        for i in 0..n {
            if (mask >> i) & 1 == 0 {
                continue;
            }

            for j in (i + 1)..n {
                if (mask >> j) & 1 == 1 {
                    sum += a[i][j];
                }
            }
        }

        score[mask] = sum;
    }

    // dp[mask]:
    //   mask のうさぎたちをいくつかのグループに分けたときの最大得点
    let mut dp = vec![i64::MIN; size];
    dp[0] = 0;

    for mask in 1..size {
        // 最初の sub は mask 自身。
        // つまり、mask に含まれるうさぎ全員を 1 グループにする候補から試す。
        let mut sub = mask;

        while sub > 0 {
            // sub を 1 グループとして切り出す。
            //
            // rest = mask ^ sub は、sub に含まれない残りのうさぎ集合。
            // rest をどうグループ分けするかは dp[rest] に任せる。
            let rest = mask ^ sub;

            // mask を小さい順に処理している。
            // sub は mask の空でない部分集合なので、
            // rest は mask から bit を消した集合になり、数値として必ず mask より小さい。
            // したがって、この時点で dp[rest] は計算済み。
            dp[mask] = dp[mask].max(dp[rest] + score[sub]);

            // bit DP でよく使う部分集合列挙。
            //
            // sub = sub - 1 だけだと、mask に含まれない bit が立った集合まで列挙してしまう。
            // (sub - 1) & mask とすることで、mask の部分集合だけを次々に列挙できる。
            sub = (sub - 1) & mask;
        }
    }

    dp[size - 1]
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0_i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    println!("{}", resolve(&a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let a = vec![vec![0, 10, 20], vec![10, 0, -100], vec![20, -100, 0]];

        assert_eq!(resolve(&a), 20);
    }

    #[test]
    fn sample_02() {
        let a = vec![vec![0, -10], vec![-10, 0]];

        assert_eq!(resolve(&a), 0);
    }

    #[test]
    fn sample_03() {
        let a = vec![
            vec![0, 1000000000, 1000000000, 1000000000, 1000000000],
            vec![1000000000, 0, 1000000000, 1000000000, 1000000000],
            vec![1000000000, 1000000000, 0, -1, -1],
            vec![1000000000, 1000000000, -1, 0, -1],
            vec![1000000000, 1000000000, -1, -1, 0],
        ];

        assert_eq!(resolve(&a), 6999999997);
    }
}
