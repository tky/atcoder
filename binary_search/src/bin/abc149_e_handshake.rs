// https://atcoder.jp/contests/abc149/tasks/abc149_e

// ゲストN人(パワーA_i)。握手は「左手のゲストx、右手のゲストy」の順序対で、
// 同じ (x, y) は一度きり(x == y も可、(x,y)と(y,x)は別)。全部で N^2 通り。
// M回の握手で得られる幸福度 Σ(A_x + A_y) の最大値を求める。
// N <= 10^5, M <= N^2 = 10^10 なので「全ペアを並べて上からM個」は不可。
//
// 2段構えの問題(Step 2 + Step 1 の合わせ技):
// 1. 「和が t 以上のペアだけ全部選ぶ」としたときのペア数は、tを下げると単調に増える。
//    「ペア数が M 以上になる境界の t」を答えの二分探索で探す(ペア数のカウントは
//    ソート + 二分探索: Snuke Festival でやった形)
// 2. 境界 t が決まったら、「和が t+1 以上のペア」は全部選び、残りは和がちょうど t の
//    ペアで埋める。「和が t 以上のペアの総和」を高速に出すには累積和が要る
//    (各 x に対し「A_y >= t - A_x となる y」の個数と、その A_y たちの合計)
fn resolve(a: &[usize], m: usize) -> usize {
    let n = a.len();

    let mut ans = vec![0; n * n];

    let mut idx = 0;
    for l in 0..n {
        for r in 0..n {
            ans[idx] = a[l] + a[r];
            idx = idx + 1;
        }
    }

    ans.sort();

    ans[n * n - m..].iter().sum()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[10, 14, 19, 34, 33], 3), 202);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[1, 3, 5, 110, 24, 21, 34, 5, 3], 14), 1837);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &[67597, 52981, 5828, 66249, 75177, 64141, 40773, 79105, 16076],
                73
            ),
            8128170
        );
    }
}
