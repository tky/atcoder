// https://atcoder.jp/contests/abc070/tasks/abc070_d

// N頂点の木(連結・無向・辺N-1本・閉路なし)。i番目の辺は a_i と b_i を距離 c_i で結ぶ。
// Q個のクエリと整数K: クエリjでは「頂点 x_j から**頂点Kを経由して**頂点 y_j へ
// 移動する最短距離」を答える。
//
// Step 5 木の1問目。考えどころ:
// - 「Kを経由する最短距離」は2つの距離に分解できる。どう分解する?
// - Qは最大10^5。クエリのたびに探索すると全体でどのくらいの計算量になる?
//   → 何かを**前計算**しておけば各クエリO(1)で答えられる。何を前計算する?
// - 前計算はDijkstraでもできる(重み付き最短路だから)。でも木には
//   「任意の2頂点間の経路がちょうど1本しかない」という性質がある。
//   経路が1本しかないなら、「より短い別ルート」は存在しない — すると
//   Dijkstraの装備(heap・緩和の不等式・popガード)のうち、要らなくなるのはどれ?
//   逆に、無向の木を素直に辿るとき**それでも必要なまま残るもの**は何?
//   (規約「積むときに印」はここでも生きる)
// - 重み付きなので「BFSのレベル=距離」(重み1専用)は使えない。でも上の考察が
//   正しければ、キューでもスタックでも好きな順で辿って距離を足し込むだけでよい
// - N=10^5の直線の木がありうる。再帰DFSで書くと何が起きる(deep_pathが検出する)
// - x_j ≠ K, y_j ≠ K は保証されるが、依存しない実装に(K自身の距離は自然に0)
// - 答えは最大 ~2*10^14(サンプル3が32bit超えを見せてくる)→ usize(64bit)
// 制約: 3 ≤ N ≤ 10^5, 1 ≤ c_i ≤ 10^9, 与えられるグラフは木,
//       1 ≤ Q ≤ 10^5, 1 ≤ K ≤ N, x_j ≠ y_j, x_j ≠ K, y_j ≠ K
//
// resolve: edges[i] = (a, b, c) は a-b を距離cで結ぶ無向辺(頂点は1-indexed)。
// queries[j] = (x, y)。返り値 = 各クエリの答えを順に並べたVec
fn resolve(
    n: usize,
    edges: &[(usize, usize, usize)],
    k: usize,
    queries: &[(usize, usize)],
) -> Vec<usize> {
    let mut distances = vec![usize::MAX; n];
    distances[k - 1] = 0;
    let mut stack = vec![k - 1];

    let g = build_graph(n, edges);

    while let Some(v) = stack.pop() {
        for &(to, w) in g[v].iter() {
            if distances[to] == usize::MAX {
                distances[to] = distances[v] + w;
                stack.push(to);
            }
        }
    }

    queries
        .iter()
        .map(|&(x, y)| distances[x - 1] + distances[y - 1])
        .collect()
}

// (頂点、距離）
type Graph = Vec<Vec<(usize, usize)>>;

fn build_graph(n: usize, edges: &[(usize, usize, usize)]) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to, w) in edges.iter() {
        g[from - 1].push((to - 1, w));
        g[to - 1].push((from - 1, w));
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
        let edges = [(1, 2, 1), (1, 3, 1), (2, 4, 1), (3, 5, 1)];
        let queries = [(2, 4), (2, 3), (4, 5)];
        assert_eq!(resolve(5, &edges, 1, &queries), vec![3, 2, 4]);
    }

    #[test]
    fn sample_02() {
        // スター: 全クエリがK=2(葉)経由を強制される
        let edges = [
            (1, 2, 1),
            (1, 3, 3),
            (1, 4, 5),
            (1, 5, 7),
            (1, 6, 9),
            (1, 7, 11),
        ];
        let queries = [(1, 3), (4, 5), (6, 7)];
        assert_eq!(resolve(7, &edges, 2, &queries), vec![5, 14, 22]);
    }

    #[test]
    fn sample_03() {
        // 直線、全辺10^9: 答え 1.7*10^10 は32bit超え
        let edges: Vec<(usize, usize, usize)> =
            (1..10).map(|i| (i, i + 1, 1_000_000_000)).collect();
        assert_eq!(resolve(10, &edges, 1, &[(9, 10)]), vec![17_000_000_000]);
    }

    #[test]
    fn handmade() {
        // Kが x-y の経路上にない → 寄り道して戻る(頂点2を2回通る)
        //   1-2(1), 2-3(2), 2-4(3)。K=4, (1,3) → (1+3)+(3+2) = 9
        let edges = [(1, 2, 1), (2, 3, 2), (2, 4, 3)];
        assert_eq!(resolve(4, &edges, 4, &[(1, 3)]), vec![9]);
        // 最小の木 N=3、Kが経路の真ん中
        let edges = [(1, 2, 5), (2, 3, 7)];
        assert_eq!(resolve(3, &edges, 2, &[(1, 3), (3, 1)]), vec![12, 12]);
    }

    // ---- naive: 辺の緩和を変化がなくなるまで繰り返してKからの距離を出す
    //      (Bellman-Ford風、別原理) ----
    fn naive(
        n: usize,
        edges: &[(usize, usize, usize)],
        k: usize,
        queries: &[(usize, usize)],
    ) -> Vec<usize> {
        let inf = usize::MAX / 2;
        let mut dist = vec![inf; n];
        dist[k - 1] = 0;
        loop {
            let mut changed = false;
            for &(a, b, c) in edges {
                let (a, b) = (a - 1, b - 1);
                if dist[a] + c < dist[b] {
                    dist[b] = dist[a] + c;
                    changed = true;
                }
                if dist[b] + c < dist[a] {
                    dist[a] = dist[b] + c;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        queries
            .iter()
            .map(|&(x, y)| dist[x - 1] + dist[y - 1])
            .collect()
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261109u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..200 {
            let n = 3 + next(20);
            // 木の生成: 頂点v(2..=n)を 1..v のどれかにつなぐ(必ず木になる)
            let edges: Vec<(usize, usize, usize)> = (2..=n)
                .map(|v| (1 + next((v - 1) as u64), v, 1 + next(100)))
                .collect();
            let k = 1 + next(n as u64);
            let queries: Vec<(usize, usize)> = (0..10)
                .filter_map(|_| {
                    let x = 1 + next(n as u64);
                    let y = 1 + next(n as u64);
                    // 問題の保証(x≠y, x≠K, y≠K)を満たすものだけ採用
                    (x != y && x != k && y != k).then_some((x, y))
                })
                .collect();
            assert_eq!(
                resolve(n, &edges, k, &queries),
                naive(n, &edges, k, &queries),
                "n={n} k={k} edges={edges:?} queries={queries:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 深さ最悪: 直線 1-2-...-N 全辺10^9、K=1、クエリ(2,N)をQ回
        // 答えは毎回 (1 + (N-1)) * 10^9 = N * 10^9
        let n = 100_000;
        let w = 1_000_000_000;
        let edges: Vec<(usize, usize, usize)> = (1..n).map(|i| (i, i + 1, w)).collect();
        let queries: Vec<(usize, usize)> = (0..100_000).map(|_| (2, n)).collect();
        let expected = vec![n * w; queries.len()];
        assert_eq!(resolve(n, &edges, 1, &queries), expected);
    }

    #[test]
    fn wide_star() {
        // 幅最悪: 中心1に葉2..=Nが重み1でぶら下がる。K=2(葉)。
        // 葉どうしのクエリ(i, i+1)は全て 2+2 = 4
        let n = 100_000;
        let edges: Vec<(usize, usize, usize)> = (2..=n).map(|v| (1, v, 1)).collect();
        let queries: Vec<(usize, usize)> = (3..n).map(|i| (i, i + 1)).collect();
        let expected = vec![4; queries.len()];
        assert_eq!(resolve(n, &edges, 2, &queries), expected);
    }
}
