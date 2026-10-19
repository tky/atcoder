// https://atcoder.jp/contests/abc292/tasks/abc292_d

// N頂点M辺の無向グラフ(自己ループ・多重辺あり!)。
// すべての連結成分が「頂点数 == 辺数」を満たすなら true、1つでも破れば false。
//
// 連結成分カウント(abc284)の発展形: 成分を見つけるだけでなく、
// 成分ごとに「頂点数」と「辺数」を集計する。
//
// ヒント:
// - 頂点数: DFSで塗った頂点を数えるだけ
// - 辺数: 「成分内の次数の合計 = 辺数の2倍」を使うと、辺に印を付けて回る必要がない。
//   自己ループ (v, v) も build_graph が g[v] に2回積む = 次数2を寄与するので、
//   この等式は自己ループ込みで正しく成り立つ
// - 多重辺・自己ループがあっても、visited管理(積むときに印)さえ正しければ
//   探索は何も特別扱いしなくてよい — それを確かめるのもこの問題の価値
fn resolve(n: usize, edges: &[(usize, usize)]) -> bool {
    let g = build_graph(n, edges);
    let mut visited = vec![false; n];

    for v in 0..n {
        if !visited[v] {
            let (node_count, edge_count) = dfs_iter(&g, &mut visited, v);
            if node_count != edge_count {
                return false;
            }
        }
    }
    true
}

fn dfs_iter(g: &Graph, visited: &mut [bool], start: usize) -> (usize, usize) {
    let mut stack = vec![start];
    visited[start] = true;
    let mut count = 0;
    let mut degree_sum = 0;
    while let Some(v) = stack.pop() {
        count += 1;
        degree_sum += g[v].len();
        for &to in g[v].iter() {
            if !visited[to] {
                stack.push(to);
                visited[to] = true;
            }
        }
    }
    (count, degree_sum / 2)
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
        // 自己ループ(1,1)と多重辺(2,3)x2 を含む
        assert_eq!(resolve(3, &[(2, 3), (1, 1), (2, 3)]), true);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(5, &[(1, 2), (2, 3), (3, 4), (3, 5), (1, 5)]), true);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                13,
                &[
                    (7, 9),
                    (7, 11),
                    (3, 8),
                    (1, 13),
                    (11, 11),
                    (6, 11),
                    (8, 13),
                    (2, 11),
                    (3, 3),
                    (8, 12),
                    (9, 11),
                    (1, 11),
                    (5, 13),
                    (3, 12),
                    (6, 9),
                    (1, 10)
                ]
            ),
            false
        );
    }

    #[test]
    fn handmade() {
        assert_eq!(resolve(1, &[]), false); // 孤立点: 頂点1, 辺0
        assert_eq!(resolve(1, &[(1, 1)]), true); // 自己ループ1本: 頂点1, 辺1
        assert_eq!(resolve(3, &[(1, 2), (2, 3), (3, 1)]), true); // 三角形(閉路)
        assert_eq!(resolve(2, &[(1, 2)]), false); // 木(辺が1本足りない)
        // Yesな成分 + Noな成分の混在 → false
        assert_eq!(resolve(5, &[(1, 2), (2, 3), (3, 1), (4, 5)]), false);
        assert_eq!(resolve(5, &[(1, 2), (1, 3), (1, 4), (1, 5)]), false); // スター(木): 頂点5, 辺4
    }
}
