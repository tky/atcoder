const MOD: usize = 1_000_000_007;

// 無向辺 (i, j)
type Edge = (usize, usize);

// 無向グラフ（隣接リスト）
type Graph = Vec<Vec<usize>>;

// 辺集合
type Edges = Vec<Edge>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to) in edges.iter() {
        g[from].push(to);
        g[to].push(from);
    }
    g
}

fn resolve(edges: &Edges, n: usize) -> usize {
    // dp[v][0]: vを白で塗る場合の、vを根とする部分木の塗り方数
    // dp[v][1]: vを黒で塗る場合の、vを根とする部分木の塗り方数
    let mut dp = vec![vec![0usize; 2]; n];

    let graph = build_graph(n, edges);

    // 0 を根にする
    // 親は存在しない値を適当に設定している
    dfs(0, usize::MAX, &graph, &mut dp);

    (dp[0][0] + dp[0][1]) % MOD
}

// 帰りがけ順で計算する DPの基本形
// 自分を初期化する
// ↓
// 子に潜る
// ↓
// 子の dp が完成して戻ってくる
// ↓
// 子の結果を使って自分の dp を更新する
fn dfs(v: usize, parent: usize, graph: &Graph, dp: &mut Vec<Vec<usize>>) {
    // 子をまだ何も処理していない時点では、
    // v 自身を白にする方法も黒にする方法も 1 通り。
    // これから子部分木の通り数を掛け合わせていくので、
    // 掛け算の単位元である 1 で初期化する。
    // 葉の場合は子がいないので、このまま dp[v][0] = dp[v][1] = 1 になる。
    dp[v][0] = 1;
    dp[v][1] = 1;

    for &to in graph[v].iter() {
        // 戻らないように
        // サイクルがないのでvisitedのようなものを持つ必要がなく、どちらから来たかだけわかれば良い
        if to == parent {
            continue;
        }
        // このdfsでvより下の子のdpが完成している
        dfs(to, v, graph, dp);

        dp[v][0] = dp[v][0] * (dp[to][0] + dp[to][1]) % MOD;
        // vが黒なら、子toは白でなければならない
        dp[v][1] = dp[v][1] * dp[to][0] % MOD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let n = 3;
        let edges = vec![(0, 1), (1, 2)];
        assert_eq!(resolve(&edges, n), 5);
    }

    #[test]
    fn sample_02() {
        let n = 4;
        let edges = vec![(0, 1), (0, 2), (2, 3)];
        assert_eq!(resolve(&edges, n), 8);
    }

    #[test]
    fn sample_03() {
        let n = 1;
        let edges = vec![];
        assert_eq!(resolve(&edges, n), 2);
    }

    #[test]
    fn line_tree_4() {
        let n = 4;
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        assert_eq!(resolve(&edges, n), 8);
    }

    #[test]
    fn star_tree_4() {
        let n = 4;
        let edges = vec![(0, 1), (0, 2), (0, 3)];
        assert_eq!(resolve(&edges, n), 9);
    }
}

fn main() {
    println!("Hello, world!");
}
