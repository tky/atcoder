// https://atcoder.jp/contests/abc213/tasks/abc213_d

use itertools::Itertools;

// N個の都市とN-1本の道(つながっているので木)。都市1から出発し:
// - 現在地に未訪問の隣接都市があれば、その中で番号最小の都市へ進む
// - なければ、1つ前にいた都市へ戻る(都市1で戻る先がなければ終了)
// 訪れた都市の列(戻りの通過も含む)を出力する。
//
// 前問からのステップアップ:
// - この移動ルールは「隣接リストを番号順に見る再帰DFS」そのもの。
//   ただし今回は訪問の「順序」と「戻り」が答えになる
// - 記録のタイミングが鍵: 頂点に入ったときに加え、
//   子の探索から戻ってくるたびに自分をもう一度記録する(オイラーツアー)
// - 隣接リストは番号最小から進むために各頂点でソートしておく
// - N <= 2*10^5 … 一直線の木で再帰が20万段になる。前問で踏んだアレの出番
fn resolve(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut tour = Vec::new();

    let g = build_graph(n, edges);
    let mut visited = vec![false; n];
    let mut origin = vec![n + 1; n];
    let mut stack = vec![0];
    let mut cursor = vec![0usize; n];

    while let Some(v) = stack.pop() {
        tour.push(v + 1);
        visited[v] = true;

        // 頂点ごとにどこまで読んだか覚えておく
        // 先頭から未訪問のnodeを調べるとO(N^2)になってしまう
        // たとえばこんな実装
        // if let Some(next) = g[v].iter().find(|&&k| !visited[k]).copied() {
        while cursor[v] < g[v].len() && visited[g[v][cursor[v]]] {
            cursor[v] += 1;
        }
        if cursor[v] < g[v].len() {
            let next = g[v][cursor[v]];
            stack.push(next);
            if origin[next] > n {
                origin[next] = v;
            }
        } else {
            if v != 0 {
                stack.push(origin[v]);
            }
        }
    }

    tour
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

    for v in 0..n {
        g[v].sort_unstable();
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
        assert_eq!(
            resolve(4, &[(1, 2), (4, 2), (3, 1)]),
            vec![1, 2, 4, 2, 1, 3, 1]
        );
    }

    #[test]
    fn sample_02() {
        assert_eq!(
            resolve(5, &[(1, 2), (1, 3), (1, 4), (1, 5)]),
            vec![1, 2, 1, 3, 1, 4, 1, 5, 1]
        );
    }

    #[test]
    fn handmade() {
        assert_eq!(resolve(2, &[(2, 1)]), vec![1, 2, 1]); // 最小の木
        // 一直線 1-2-3: 行って戻るだけ
        assert_eq!(resolve(3, &[(1, 2), (2, 3)]), vec![1, 2, 3, 2, 1]);
    }

    #[test]
    fn deep_path() {
        // 一直線の木(20万頂点)。再帰で書くとここで落ちる
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        let path = resolve(n, &edges);
        assert_eq!(path.len(), 2 * n - 1); // 行き n + 帰り n-1
        assert_eq!(path[0], 1);
        assert_eq!(path[n - 1], n); // 折り返し点
        assert_eq!(*path.last().unwrap(), 1);
    }
}
