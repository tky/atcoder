// https://atcoder.jp/contests/abc284/tasks/abc284_c

// N頂点M辺の単純無向グラフの「連結成分」の個数を求める。
// 連結成分 = 辺をたどって行き来できる頂点のかたまり。
// N <= 100 と小さいので計算量の心配は不要。グラフの基本動作を身につける回。
//
// 手順(グラフ問題の基本形):
// 1. 隣接リストを作る: graph[v] = vと直接つながっている頂点のリスト
//    (Vec<Vec<usize>>。無向グラフなので u->v と v->u の両方向に積む)
//    入力の頂点番号は1-indexed。内部は0-indexedに統一する(入口で変換)
// 2. visited(訪問済み)配列を用意する
// 3. 各頂点を見て、未訪問なら「新しい連結成分を発見」としてカウントし、
//    そこからDFSで到達できる頂点を全部visitedにする
//
// DFSは再帰で書いても、Vecをスタックとして使って書いてもよい(両方書いてみる価値あり)
fn resolve(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);
    let mut visited = vec![false; n];
    let mut count = 0;

    for v in 0..n {
        if !visited[v] {
            count += 1;
            dfs(&g, &mut visited, v);
        }
    }

    count
}

fn resolve2(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);
    let mut visited = vec![false; n];
    let mut count = 0;

    for v in 0..n {
        if !visited[v] {
            count += 1;
            dfs_iter(&g, &mut visited, v);
        }
    }

    count
}

fn dfs_iter(g: &Graph, visited: &mut Vec<bool>, start: usize) {
    let mut stack = vec![start];
    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        for &to in g[v].iter() {
            if !visited[to] {
                stack.push(to);
            }
        }
    }
}

fn dfs(g: &Graph, visited: &mut Vec<bool>, n: usize) {
    if visited[n] {
        return;
    }
    visited[n] = true;
    for &to in g[n].iter() {
        dfs(g, visited, to);
    }
}

type Graph = Vec<Vec<usize>>;

type Edge = (usize, usize);

type Edges = [Edge];

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to) in edges.iter() {
        g[from - 1].push(to - 1);
        g[to - 1].push(from - 1);
    }
    g
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(5, &[(1, 2), (1, 3), (4, 5)]), 2);
        assert_eq!(resolve2(5, &[(1, 2), (1, 3), (4, 5)]), 2);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(5, &[]), 5);
        assert_eq!(resolve2(5, &[]), 5);
    }

    #[test]
    fn sample_03() {
        // 完全グラフ K4
        assert_eq!(
            resolve(4, &[(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]),
            1
        );
        assert_eq!(
            resolve2(4, &[(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]),
            1
        );
    }

    #[test]
    fn handmade() {
        assert_eq!(resolve(1, &[]), 1); // 頂点1個・辺なし
        assert_eq!(resolve(6, &[(1, 2), (2, 3), (3, 1), (4, 5)]), 3); // 三角形 + 線 + 孤立点6

        assert_eq!(resolve2(1, &[]), 1); // 頂点1個・辺なし
        assert_eq!(resolve2(6, &[(1, 2), (2, 3), (3, 1), (4, 5)]), 3); // 三角形 + 線 + 孤立点6
    }

    #[test]
    #[ignore = "再帰版DFSはテストスレッドの2MBスタックを溢れさせるため"]
    fn deep_path() {
        // 20万頂点が一直線: 1-2-3-...-200000
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve2(n, &edges), 1); // 明示スタック版 → 通る
        assert_eq!(resolve(n, &edges), 1); // 再帰版 → コメントを外すと…?
    }
}
