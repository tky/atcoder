// https://atcoder.jp/contests/abc270/tasks/abc270_c

// N頂点の木と2頂点X, Y(X != Y)が与えられる。
// XからYへの単純パス上の頂点を、通る順に出力する。
// (木なので2頂点間の単純パスはちょうど1本 — だから「順に」が一意に決まる)
//
// ヒント: 高橋君ツアーで使った origin(どこから来たか)がそのまま答えになる。
// 1. Xを出発点にDFSして、全頂点の origin(親)を記録する
// 2. Yから origin を辿って X まで遡る → 逆順にすれば X→Y のパス
// N <= 2*10^5 なので一直線の木で再帰は死ぬ(deep_pathテストあり)。明示版で
fn resolve(n: usize, x: usize, y: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut visited = vec![false; n];
    let mut origin = vec![n + 1; n];
    let g = build_graph(n, edges);

    let mut stack = vec![x - 1];

    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if !visited[to] {
                stack.push(to);
                visited[v] = true;
                if origin[to] > n {
                    origin[to] = v;
                }
            }
        }
    }
    let mut path = vec![y - 1];

    let mut v = y - 1;
    while v != x - 1 {
        v = origin[v];
        path.push(v);
    }

    path.reverse();
    path.iter().map(|&v| v + 1).collect()
}

fn main() {
    println!("Hello, world!");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(
            resolve(5, 2, 5, &[(1, 2), (1, 3), (3, 4), (3, 5)]),
            vec![2, 1, 3, 5]
        );
    }

    #[test]
    fn sample_02() {
        assert_eq!(
            resolve(6, 1, 2, &[(3, 1), (2, 5), (1, 2), (4, 1), (2, 6)]),
            vec![1, 2]
        );
    }

    #[test]
    fn handmade() {
        // 最小の木
        assert_eq!(resolve(2, 1, 2, &[(1, 2)]), vec![1, 2]);
        assert_eq!(resolve(2, 2, 1, &[(1, 2)]), vec![2, 1]); // 逆向き
        // スターの端から端(中心を経由)
        assert_eq!(resolve(4, 2, 4, &[(1, 2), (1, 3), (1, 4)]), vec![2, 1, 4]);
    }

    #[test]
    fn deep_path() {
        // 一直線の木: 1-2-...-n、端から端まで
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        let path = resolve(n, 1, n, &edges);
        assert_eq!(path.len(), n);
        assert_eq!(path[0], 1);
        assert_eq!(*path.last().unwrap(), n);
    }

    #[test]
    fn wide_star() {
        // スター: 葉から葉へ(中心の次数が最大)
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|i| (1, i)).collect();
        assert_eq!(resolve(n, 2, n, &edges), vec![2, 1, n]);
    }
}
