// https://atcoder.jp/contests/typical90/tasks/typical90_c

// N都市とN-1本の双方向道路(つまり**木**)。好きな2都市 u, v を選んで道路を
// 1本だけ新設する。スコア = 「同じ道を2度通らずにある都市から同じ都市へ戻る
// 経路(=閉路)の道の本数」。スコアの最大値を出力。
//
// Step 5 木の5問目: 木の直径。考えどころ:
// - モデル化: 木に辺(u,v)を1本足すと閉路がちょうど1つできる。その長さは
//   「木の中のu-v経路の辺数 + 1」(木の経路は一意、はもうお馴染み)。
//   つまりスコア最大化 = **最も遠い2頂点を選ぶ** = 木の直径を求める問題
// - 愚直: 全頂点から距離埋めして全ペアの最大 → O(N²) = 10^10 で間に合わない
// - 定番の2回走査: ①適当な頂点(頂点1でよい)から最遠の頂点 u を見つける。
//   ②uから最遠の距離を測る。それが直径。O(N)が2回で済む
// - なぜ①の最遠点 u が必ず「直径の端点」になっているのか? — これが本問の
//   核心で、実装が通ったあとに議論します(先に自分の言葉で考えてみてください。
//   反例を作ろうとしてみるのが近道: 「1からの最遠点なのに、どの直径の端点でも
//   ない頂点」を持つ木は描けるか?)
// - 最遠点が複数いたらどれを選んでもいいのか?も同じ議論から出てくる
// - 重みなし+木なので、距離埋めはいつものスタック走査でそのまま(順序不問、
//   abc070_dでやった理屈)。BFSでももちろん可
// - handmadeに「頂点1が直径の途中にある木」を仕込んである。①をやらずに
//   「頂点1からの最遠距離+1」と答える実装はそこで落ちる
// 制約: 3 ≤ N ≤ 10^5, 1 ≤ A_i < B_i ≤ N, グラフは連結(=木)
//
// resolve: edges[i] = (a, b) は都市a-bを結ぶ双方向道路(1-indexed)。
// 返り値 = スコアの最大値(= 直径の辺数 + 1)
fn resolve(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);
    let (left, _) = find_farthest(n, 0, &g);
    let (_, d) = find_farthest(n, left, &g);
    d + 1
}

fn find_farthest(n: usize, from: usize, g: &Graph) -> (usize, usize) {
    let mut stack = vec![from];
    let mut distances = vec![usize::MAX; n];
    distances[from] = 0;
    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if distances[to] == usize::MAX {
                distances[to] = distances[v] + 1;
                stack.push(to);
            }
        }
    }
    distances
        .iter()
        .enumerate()
        .max_by_key(|&(_, d)| *d)
        .map(|(v, d)| (v, *d))
        .unwrap()
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(3, &[(1, 2), (2, 3)]), 3);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(5, &[(1, 2), (2, 3), (3, 4), (3, 5)]), 4);
    }

    #[test]
    fn sample_03() {
        let edges = [
            (1, 2),
            (1, 3),
            (2, 4),
            (4, 5),
            (4, 6),
            (3, 7),
            (7, 8),
            (8, 9),
            (8, 10),
        ];
        assert_eq!(resolve(10, &edges), 8);
    }

    #[test]
    fn sample_04() {
        // 完全二分木31頂点(親 = i/2)
        let edges: Vec<(usize, usize)> = (2..=31).map(|i| (i / 2, i)).collect();
        assert_eq!(resolve(31, &edges), 9);
    }

    #[test]
    fn handmade() {
        // 最小ケース: n=3 スター(直径2 → 3)
        assert_eq!(resolve(3, &[(1, 2), (1, 3)]), 3);
        // 頂点1が直径 4-2-1-3-5 の真ん中にいる木(直径4 → 5)。
        // 「頂点1からの最遠距離+1」と答える誤実装は 3 を返す
        assert_eq!(resolve(5, &[(1, 2), (1, 3), (2, 4), (3, 5)]), 5);
    }

    // ---- naive: 全頂点から距離を埋めて全ペア距離の最大+1(定義そのまま、O(N²)) ----
    fn naive(n: usize, edges: &[(usize, usize)]) -> usize {
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in edges {
            adj[a - 1].push(b - 1);
            adj[b - 1].push(a - 1);
        }
        let mut dia = 0;
        for s in 0..n {
            let mut dist = vec![usize::MAX; n];
            dist[s] = 0;
            let mut stack = vec![s];
            while let Some(v) = stack.pop() {
                for &to in adj[v].iter() {
                    if dist[to] == usize::MAX {
                        dist[to] = dist[v] + 1;
                        stack.push(to);
                    }
                }
            }
            dia = dia.max(*dist.iter().max().unwrap());
        }
        dia + 1
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
        for _ in 0..300 {
            let n = 3 + next(20);
            // 形: 頂点v(2..=n)の親を 1..v から選び、ラベルをシャッフル(根は1固定)
            let mut label: Vec<usize> = (0..=n).collect();
            for v in (3..=n).rev() {
                let r = 2 + next((v - 1) as u64);
                label.swap(v, r);
            }
            let edges: Vec<(usize, usize)> = (2..=n)
                .map(|v| {
                    let p = 1 + next((v - 1) as u64);
                    (label[p].min(label[v]), label[p].max(label[v]))
                })
                .collect();
            assert_eq!(
                resolve(n, &edges),
                naive(n, &edges),
                "n={n} edges={edges:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 深さ最悪: 鎖 10^5(直径 N-1 → 答え N)。O(N²)の愚直もここで死ぬ
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &edges), n);
    }

    #[test]
    fn wide_star() {
        // 幅最悪: スター 10^5(直径2 → 答え3)
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        assert_eq!(resolve(n, &edges), 3);
    }
}
