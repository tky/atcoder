const MOD: u64 = 1_000_000_007;

fn resolve(k: usize, a: &Vec<Vec<usize>>) -> u64 {
    let len = a.len();

    // dp[t][i][j]:
    //   長さ t の walk で、頂点 i から頂点 j へ行く方法数
    //
    // dp[t + 1][i][j]:
    //   長さ t の walk で i から v まで行き、
    //   最後に辺 v -> j を 1 本追加する
    //
    //   dp[t + 1][i][j] += dp[t][i][v] * a[v][j]
    //
    // dp[0][i][i] = 1
    //   長さ 0 の walk は、同じ頂点にとどまる 1 通り
    let mut dp = vec![vec![vec![0_u64; len]; len]; k + 1];

    for i in 0..len {
        dp[0][i][i] = 1;
    }

    for t in 0..k {
        for i in 0..len {
            for j in 0..len {
                for v in 0..len {
                    // dp[t][i][v]:
                    //   長さ t で i から v へ行く walk の数
                    //
                    // a[v][j] == 1:
                    //   v から j へ 1 本の辺で行ける
                    //
                    // つまり、
                    //   i -> ... -> v
                    // という長さ t の walk の後ろに v -> j をつなげることで、
                    //
                    //   i -> ... -> v -> j
                    //
                    // という長さ t + 1 の i から j への walk を作れる。
                    // そのため、dp[t][i][v] 通りを dp[t + 1][i][j] に足す。
                    if a[v][j] == 1 {
                        dp[t + 1][i][j] += dp[t][i][v];
                    }
                }
            }
        }
    }

    let mut sum = 0_u64;
    for i in 0..len {
        for j in 0..len {
            sum += dp[k][i][j];
        }
    }

    sum
}

// メモリ改良版
// dp[t + 1] は dp[t] だけから作れるので、
// dp は直前の状態 cur だけ持てばよい。
fn resolve2(k: usize, a: &Vec<Vec<usize>>) -> u64 {
    let n = a.len();

    // cur[i][j]:
    //   現在の長さ t で、i から j へ行く walk の数
    //
    // 最初は長さ 0。
    // 長さ 0 で i から i へ行く walk は「何もしない」1 通り。
    let mut cur = vec![vec![0_u64; n]; n];
    for i in 0..n {
        cur[i][i] = 1;
    }

    for _ in 0..k {
        let mut next = vec![vec![0_u64; n]; n];

        for i in 0..n {
            for v in 0..n {
                if cur[i][v] == 0 {
                    continue;
                }

                for j in 0..n {
                    // cur[i][v]:
                    //   長さ t で i から v へ行く walk の数
                    //
                    // a[v][j] == 1:
                    //   v から j へ 1 本の辺で行ける
                    //
                    // つまり、
                    //   i -> ... -> v -> j
                    // という長さ t + 1 の walk を作れる。
                    if a[v][j] == 1 {
                        next[i][j] += cur[i][v];
                        next[i][j] %= MOD;
                    }
                }
            }
        }

        cur = next;
    }

    cur.iter()
        .flatten()
        .copied()
        .fold(0_u64, |acc, x| (acc + x) % MOD)
}

// dp[t+1][i][j] = for(v = 0 to N - 1) {dp[t][i][v]*dp[v][j]}
// これはdp[t+1] = dp[t] * A
// たとえば２歩の場合
// (A^2)[i][j] = for (v = 0 to 1) A[i][v]*A[v][j]
// A^2の成分がiからjへいくパスの数になっている
//
// dp[0]は単位行列になる -> 自分から自分ヘは1通り、それ以外は0
// dp[0] = I, dp[1] = A, dp[2] = A^2,,,
//
// すなわちこれはA^kを求めればiからへいくパスの数を計算したことになる

type Matrix = Vec<Vec<u64>>;
fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut res = vec![vec![0; n]; n];

    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..n {
                res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }

    res
}
fn identity(n: usize) -> Matrix {
    let mut id = vec![vec![0; n]; n];
    for i in 0..n {
        id[i][i] = 1;
    }
    id
}
fn mat_pow(a: &Matrix, mut k: u64) -> Matrix {
    let n = a.len();
    let mut base = a.clone();
    let mut result = identity(n);

    while k > 0 {
        if k % 2 == 1 {
            result = mul(&result, &base);
        }
        base = mul(&base, &base);
        k /= 2;
    }

    result
}

fn resolve3(k: u64, a: &Matrix) -> u64 {
    let ak = mat_pow(a, k);

    let mut ans = 0;
    for i in 0..a.len() {
        for j in 0..a.len() {
            ans = (ans + ak[i][j]) % MOD;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let a = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
        ];
        assert_eq!(resolve(2, &a), 6);
        assert_eq!(resolve2(2, &a), 6);
    }

    #[test]
    fn sample_02() {
        let a = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        assert_eq!(resolve(3, &a), 3);
        assert_eq!(resolve2(3, &a), 3);
    }

    #[test]
    fn sample_03() {
        let a = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(resolve(2, &a), 1);
        assert_eq!(resolve2(2, &a), 1);
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn sample_01() {
        let a = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]];
        assert_eq!(resolve3(2, &a), 3);
    }

    #[test]
    fn sample_02() {
        let a = vec![
            vec![0, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
        ];
        assert_eq!(resolve3(3, &a), 10);
    }

    #[test]
    fn sample_03() {
        let a = vec![
            vec![0, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 0],
        ];
        assert_eq!(resolve3(100, &a), 881888694);
    }

    #[test]
    fn k_zero_returns_n() {
        let a = vec![vec![0, 1], vec![1, 0]];
        // 長さ0のパスは各頂点から自分自身への1通りずつ
        assert_eq!(resolve3(0, &a), 2);
    }

    #[test]
    fn single_vertex_no_edge() {
        let a = vec![vec![0]];
        // 長さ1以上では進めない
        assert_eq!(resolve3(1, &a), 0);
        assert_eq!(resolve3(10, &a), 0);
    }

    #[test]
    fn two_vertices_one_directed_edge() {
        let a = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(resolve3(1, &a), 1);
        assert_eq!(resolve3(2, &a), 0);
    }

    #[test]
    fn cycle_of_two_vertices() {
        let a = vec![vec![0, 1], vec![1, 0]];
        // 長さ1: 0->1, 1->0 の2通り
        assert_eq!(resolve3(1, &a), 2);
        // 長さ2: 0->1->0, 1->0->1 の2通り
        assert_eq!(resolve3(2, &a), 2);
        // 長さ3でも2通り
        assert_eq!(resolve3(3, &a), 2);
    }

    #[test]
    fn complete_digraph_without_self_loops_n3() {
        let a = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        assert_eq!(resolve3(1, &a), 6);
        assert_eq!(resolve3(2, &a), 12);
    }
}
fn main() {
    println!("Hello, world!");
    // 3行2列の行列
    // A(3,2)
    let A = vec![vec![1, 2], vec![4, 5], vec![7, 8]];
    // 外側が行：内側が列
    // let A = vec![vec![0; 2]; 3];
    println!("{:?}", A);
    // A[0] 1行目
    println!("{:?}", A[0]);
    println!("{:?}", A[1]);
    println!("{:?}", A[2]);
}
