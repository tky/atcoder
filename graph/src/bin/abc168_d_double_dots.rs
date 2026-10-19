// https://atcoder.jp/contests/abc168/tasks/abc168_d

// N部屋M通路の洞窟(連結保証、自己ループなし、多重辺はありうる)。
// 部屋1以外の各部屋に「隣接する部屋のどれか」を指す道標を置く。
// どの部屋から出発しても、道標を辿り続けると**最短の通路数**で部屋1に着くようにしたい。
//
// 出力: 不可能なら "No"。可能なら "Yes" と、部屋2..=Nの道標(複数解はどれでも正解)。
//   → 制約で「どの2部屋も行き来できる(連結)」が保証されているので、実はNoは起きない。
//     だから resolve は Option ではなく Vec を返す設計でよい。
//     返り値 ans[i-2] = 部屋i (2 <= i <= N) の道標(1-indexed)。
//
// ここからBFS(Step 2)。考えどころ:
// - 「最短で部屋1へ」= 部屋1からの最短距離が1減る方向へ進むこと。
//   部屋1からBFSしたときの origin(どこから来たか)が、そのまま道標になる
// - DFS(stack)との違いは取り出す側だけ: VecDeque の push_back / pop_front。
//   先入れ先出しにすると「距離0の部屋を全部処理 → 距離1を全部 → 距離2を…」と
//   近い順に広がるので、最初に到達した経路が最短になる
// - visited の規約(積むときに印・最初のpushにも)はキューでもそのまま使える
fn resolve(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let g = build_graph(n, edges);
    let mut visited = vec![false; n];
    let mut origin = vec![n + 1; n];

    bfs(&g, &mut visited, 0, &mut origin);

    origin[1..].iter().map(|v| v + 1).collect()
}

fn bfs(g: &Graph, visited: &mut [bool], start: usize, origin: &mut [usize]) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    visited[start] = true;

    while let Some(v) = queue.pop_front() {
        for &to in g[v].iter() {
            if !visited[to] {
                queue.push_back(to);
                visited[to] = true;
                origin[to] = v;
            }
        }
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

    // ---- 検証器: 複数解が許されるので、期待値一致ではなく性質を確かめる ----
    // 道標が正解である条件:
    //   (1) 道標の指す先が隣接している
    //   (2) 道標を1歩辿ると部屋1からの最短距離がちょうど1減る
    //       ((2)が全部屋で成り立てば、辿るたびに距離が1減るので最短歩数で部屋1に着く)
    // 最短距離は resolve とは独立な方法(辺の緩和を変化がなくなるまで繰り返す)で計算する
    fn naive_dist(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
        let inf = usize::MAX / 2;
        let mut dist = vec![inf; n];
        dist[0] = 0;
        loop {
            let mut changed = false;
            for &(a, b) in edges {
                let (a, b) = (a - 1, b - 1);
                if dist[a] + 1 < dist[b] {
                    dist[b] = dist[a] + 1;
                    changed = true;
                }
                if dist[b] + 1 < dist[a] {
                    dist[a] = dist[b] + 1;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        dist
    }

    fn validate(n: usize, edges: &[(usize, usize)], signposts: &[usize]) {
        assert_eq!(signposts.len(), n - 1, "道標は部屋2..=Nのぶん必要");
        let dist = naive_dist(n, edges);
        for i in 2..=n {
            let p = signposts[i - 2];
            assert!(
                edges
                    .iter()
                    .any(|&(a, b)| (a, b) == (i, p) || (a, b) == (p, i)),
                "部屋{i}の道標{p}: 隣接していない"
            );
            assert_eq!(
                dist[p - 1],
                dist[i - 1] - 1,
                "部屋{i}の道標{p}: 最短距離が1減っていない"
            );
        }
    }

    #[test]
    fn sample_01() {
        // 公式の出力例は [1, 2, 2]
        let edges = [(1, 2), (2, 3), (3, 4), (4, 2)];
        validate(4, &edges, &resolve(4, &edges));
    }

    #[test]
    fn sample_02() {
        // 公式の出力例は [6, 5, 5, 1, 1](複数解あり)
        let edges = [
            (3, 4),
            (6, 1),
            (2, 4),
            (5, 3),
            (4, 6),
            (1, 5),
            (6, 2),
            (4, 5),
            (5, 6),
        ];
        validate(6, &edges, &resolve(6, &edges));
    }

    #[test]
    fn handmade() {
        // 最小: 部屋2つ
        assert_eq!(resolve(2, &[(1, 2)]), vec![1]);
        // 一直線 1-2-3: 道標は一意に [1, 2]
        assert_eq!(resolve(3, &[(1, 2), (2, 3)]), vec![1, 2]);
        // スター: 全部屋が1を指すしかない
        assert_eq!(resolve(4, &[(1, 2), (1, 3), (1, 4)]), vec![1, 1, 1]);
        // ひし形 1-2-4, 1-3-4: 部屋4は2でも3でも正解 → 検証器で
        let diamond = [(1, 2), (1, 3), (2, 4), (3, 4)];
        validate(4, &diamond, &resolve(4, &diamond));
        // サイクル 1-2-3-4-1: 部屋3は2でも4でも正解
        let cycle = [(1, 2), (2, 3), (3, 4), (4, 1)];
        validate(4, &cycle, &resolve(4, &cycle));
    }

    #[test]
    fn random_cross_check() {
        // ランダムな連結グラフ(木 + 余分な辺)を作って検証器にかける
        let mut seed = 20261019u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..30 {
            let n = 2 + next(300);
            let mut edges: Vec<(usize, usize)> = Vec::new();
            // 木: 各頂点をそれ以前のどれかにつなぐ → 必ず連結
            for v in 2..=n {
                edges.push((1 + next(v as u64 - 1), v));
            }
            // 余分な辺(自己ループだけ避ける。多重辺は制約上ありうるので許す)
            for _ in 0..next(2 * n as u64) {
                let a = 1 + next(n as u64);
                let b = 1 + next(n as u64);
                if a != b {
                    edges.push((a, b));
                }
            }
            validate(n, &edges, &resolve(n, &edges));
        }
    }

    #[test]
    fn deep_path() {
        // 一直線 1-2-...-n: 各部屋は手前を指すしかない
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        let expected: Vec<usize> = (2..=n).map(|i| i - 1).collect();
        assert_eq!(resolve(n, &edges), expected);
    }

    #[test]
    fn wide_star() {
        // スター: 全部屋が1を指す
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|i| (1, i)).collect();
        assert_eq!(resolve(n, &edges), vec![1; n - 1]);
    }
}
