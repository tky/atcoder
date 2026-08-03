// 有向グラフ（隣接リスト）
type Graph = Vec<Vec<usize>>;

// 有向辺 (from, to)
type Edge = (usize, usize);

// 辺集合
type Edges = Vec<Edge>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to) in edges.iter() {
        g[from].push(to);
    }
    g
}

// DAG = Directed Acyclic Graph
// = 有向非巡回グラフ
// = 閉路が存在しない有向グラフ
fn resolve(edges: &Edges, n: usize) -> usize {
    let graph = build_graph(n, edges);
    // DAGなのでトポロジカル順に処理することでDPできる
    let order = topological_sort(&graph);

    // 頂点vを終点とする最長パス長
    let mut dp = vec![0_usize; n];

    for from in order {
        for &to in graph[from].iter() {
            dp[to] = dp[to].max(dp[from] + 1);
        }
    }

    *dp.iter().max().unwrap()
}

/// Kahn のアルゴリズム
/// DAG なら、入次数 0 の頂点が少なくとも 1 つある
/// ↓
/// その頂点は、他の頂点より前に置いてよい
/// ↓
/// その頂点を取り除く
/// ↓
/// その頂点から出ている辺も取り除く
/// ↓
/// 新しく入次数 0 になった頂点を処理する
/// ↓
/// これを繰り返す
fn topological_sort(graph: &Graph) -> Vec<usize> {
    let n = graph.len();
    let mut indegrees = vec![0; n];

    for g in graph {
        for &to in g {
            indegrees[to] += 1;
        }
    }

    let mut queue = std::collections::VecDeque::new();
    for v in 0..n {
        if indegrees[v] == 0 {
            queue.push_back(v);
        }
    }

    let mut results = Vec::new();

    while let Some(v) = queue.pop_front() {
        results.push(v);
        for &to in graph[v].iter() {
            indegrees[to] -= 1;
            if indegrees[to] == 0 {
                queue.push_back(to);
            }
        }
    }

    results
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let edges = vec![(0, 1), (0, 2), (2, 1), (1, 3), (2, 3)];
        assert_eq!(resolve(&edges, 4), 3);
    }
}
