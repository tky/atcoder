// https://atcoder.jp/contests/dp/tasks/dp_p

// N頂点の木(i番目の辺は x_i と y_i を結ぶ)。各頂点を白か黒で塗る。
// ただし**隣り合う頂点をともに黒**にしてはいけない。
// 塗り方は何通りか、10^9 + 7 で割った余りを出力。
//
// Step 5 木の4問目: 木DP第1問(数え上げ+modも初)。考えどころ:
// - Kiと真逆の流れ: 「**子が全部確定してから親**を計算する」。vの部分木の答えを
//   子の部分木の答えから組み立てる — これが木DP
// - まず「vを根とする部分木の塗り方の数」を頂点ごとに1つの数で持つ、を考えてみる。
//   すると親に合流するとき困る。隣接黒禁止の制約で、親が子について知る必要が
//   あるのは何? → その情報で場合分けして、dpを**2本**持つ(状態拡張 abc395_e と
//   同じ発想: 足りない情報を抱き合わせる)
// - 合流のしかた: vの子 c1, c2, ... の部分木は頂点を共有しない(互いに素)ので、
//   それぞれの塗り方は独立に選べる。独立な選択の組み合わせの数は和? 積?
// - 基底: 葉のdpは何になる?
// - 処理順を再帰なしで作る: いつものスタック走査でpopした順(行きがけ順)は
//   「親が必ず子より先」に並ぶ。この列を**逆から**読むと? (親の記録には
//   originで使った parent 配列がそのまま使える)
// - mod: 答えは天文学的に大きい(スターだと 2^(N-1)+1 通り)→ 掛け算・足し算の
//   **たびに** % MOD。そうすれば値は常に <2^30、積も <2^60 でusizeに収まる。
//   取り忘れるとオーバーフロー(debugビルドのpanicが教えてくれる)
// - N=1(辺ゼロ、答えは2)もありうる
// 制約: 1 ≤ N ≤ 10^5, 1 ≤ x_i, y_i ≤ N, 与えられるグラフは木
//
// resolve: edges[i] = (x, y) は x-y を結ぶ無向辺(1-indexed)。
// 返り値 = 塗り方の総数 mod 10^9+7
const MOD: usize = 1_000_000_007;

fn resolve(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);
    let mut parent = vec![usize::MAX; n];
    // 0をrootにして親の頂点を求める
    parent[0] = 0;
    let mut stack = vec![0];
    let mut order = vec![0];

    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if parent[to] == usize::MAX {
                parent[to] = v;
                stack.push(to);
                order.push(to);
            }
        }
    }
    order.reverse();

    let mut dp = vec![vec![1; 2]; n];
    for v in order {
        if v == 0 {
            continue;
        }
        let p = parent[v];

        // 白
        dp[p][0] = dp[p][0] * (dp[v][0] + dp[v][1]) % MOD;
        // 黒
        dp[p][1] = dp[p][1] * dp[v][0] % MOD;
    }
    (dp[0][0] + dp[0][1]) % MOD
}

fn resolve1(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);
    let mut dp = vec![vec![usize::MAX; 2]; n];
    let mut visited = vec![false; n];

    dfs(0, &g, &mut dp, &mut visited);
    (dp[0][0] + dp[0][1]) % MOD
}

fn dfs(v: usize, g: &Graph, dp: &mut Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    // 白
    dp[v][0] = 1;
    // 黒
    dp[v][1] = 1;

    for &to in g[v].iter() {
        if visited[to] {
            continue;
        }
        dfs(to, g, dp, visited);
        // 白
        dp[v][0] = dp[v][0] * (dp[to][0] + dp[to][1]) % MOD;
        // 黒
        dp[v][1] = dp[v][1] * dp[to][0] % MOD;
    }
}

fn main() {
    println!("Hello, world!");
}

type Graph = Vec<Vec<usize>>;

fn build_graph(n: usize, edges: &[(usize, usize)]) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to) in edges.iter() {
        g[from - 1].push(to - 1);
        g[to - 1].push(from - 1);
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(3, &[(1, 2), (2, 3)]), 5);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(4, &[(1, 2), (1, 3), (1, 4)]), 9);
    }

    #[test]
    fn sample_03() {
        // N=1: 辺ゼロ。白か黒の2通り
        assert_eq!(resolve(1, &[]), 2);
    }

    #[test]
    fn sample_04() {
        let edges = [
            (8, 5),
            (10, 8),
            (6, 5),
            (1, 5),
            (4, 8),
            (2, 10),
            (3, 6),
            (9, 2),
            (1, 7),
        ];
        assert_eq!(resolve(10, &edges), 157);
    }

    #[test]
    fn handmade() {
        // N=2: 白白・白黒・黒白(黒黒だけ禁止)
        assert_eq!(resolve(2, &[(1, 2)]), 3);
        // 鎖5頂点
        let edges = [(1, 2), (2, 3), (3, 4), (4, 5)];
        assert_eq!(resolve(5, &edges), 13);
    }

    // ---- brute: 2^N 通りを全列挙して、黒黒の辺がないものを数える(定義そのまま) ----
    fn brute(n: usize, edges: &[(usize, usize)]) -> usize {
        (0usize..1 << n)
            .filter(|mask| {
                edges
                    .iter()
                    .all(|&(x, y)| (mask >> (x - 1)) & 1 == 0 || (mask >> (y - 1)) & 1 == 0)
            })
            .count()
            % MOD
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261115u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..200 {
            let n = 1 + next(14);
            // 形: 頂点v(2..=n)の親を 1..v から選ぶ。ラベルをシャッフルして
            // 「親の番号が小さい」とは限らない木にする(根のラベルだけ1固定)
            let mut label: Vec<usize> = (0..=n).collect();
            for v in (3..=n).rev() {
                let r = 2 + next((v - 1) as u64);
                label.swap(v, r);
            }
            let edges: Vec<(usize, usize)> = (2..=n)
                .map(|v| {
                    let p = 1 + next((v - 1) as u64);
                    (label[p], label[v])
                })
                .collect();
            assert_eq!(
                resolve(n, &edges),
                brute(n, &edges),
                "n={n} edges={edges:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 深さ最悪: 鎖 10^5 頂点(答えはフィボナッチのmod)。期待値はPythonで事前計算
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &edges), 879_053_727);
    }

    #[test]
    fn wide_star() {
        // 幅最悪: スター 10^5 頂点。公式 2^(N-1) + 1 のmod(Pythonで事前計算)。
        // modを取り忘れた実装はここに来るずっと前にオーバーフローで死ぬ
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        assert_eq!(resolve(n, &edges), 303_861_761);
    }
}
