// https://atcoder.jp/contests/abc395/tasks/abc395_e

use std::cmp::Reverse;

// N頂点M辺の**有向**グラフ。頂点1から出発して頂点Nに着きたい。各ステップで:
//   - 移動: 今いる頂点から出ている有向辺を1本選んで進む。コスト1
//   - 反転: **全ての辺**の向きを一斉に反転する。コストX
// コスト総和の最小値を出力。1からNに到達可能であることは保証される。
//
// Dijkstraひねり4問目: グラフのひねり(状態拡張)。考えどころ:
// - 「全部の辺を反転」を文字通りグラフの書き換えでシミュレートすると、反転は
//   何回でも起こせるので手に負えない…ように見える。でも反転を2回すると元通り。
//   つまりグラフの姿は「表」と「裏」の**2つしかない**
// - ここで状態拡張: 「いまどの頂点にいるか」だけでは移動できる先が決まらない。
//   足りない情報は何か? それを頂点に抱き合わせて、(場所, 世界)のペアを
//   新しい「頂点」とみなす。頂点数は2N。すると辺は3種類:
//     表での移動 u→v(コスト1)/ 裏での移動 v→u(コスト1、元の辺の逆向き)/
//     その場で世界を切り替える(コストX、場所は動かない)
// - この2N頂点のグラフは普通の非負重み有向グラフ(1とX)なので、Dijkstraが
//   そのまま乗る。ゴールは「頂点Nにいる」— 世界はどちらでもよい。答えは?
// - 2N頂点の持ち方は自由: dist配列を2本持つ / vとv+nに畳む / (v, s)のタプル。
//   番兵は不正値、1-indexedの変換は入口で一度だけ、の規約はそのまま
// - 多重辺・自己ループの有無は問題文に明記なし → あっても壊れない実装に
//   (実は特別扱いしなくても壊れない。緩和の式がなぜ勝手に無害化してくれるか一言で)
// - 答えは32bitに収まらない(サンプル3が2^32超えをわざわざ見せてくる)→ usize(64bit)
// 制約: 2 ≤ N ≤ 2*10^5, 1 ≤ M ≤ 2*10^5, 1 ≤ X ≤ 10^9, 1 ≤ u_i,v_i ≤ N
//
// resolve: edges[i] = (u, v) は u→v の有向辺(1-indexed)。
// 返り値 = 頂点Nに着くまでのコスト総和の最小値
fn resolve(n: usize, x: usize, edges: &[(usize, usize)]) -> usize {
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(Reverse((0, 0)));

    let mut distances = vec![usize::MAX; n * 2];
    distances[0] = 0;
    let graph = build_graph(n, edges, x);

    while let Some(Reverse((dist, v))) = heap.pop() {
        if dist > distances[v] {
            continue;
        }
        for &(to, cost) in graph[v].iter() {
            let next = dist + cost;
            if next < distances[to] {
                distances[to] = next;
                heap.push(Reverse((next, to)));
            }
        }
    }

    distances[n - 1].min(distances[2 * n - 1])
}

// (頂点、コスト)
type Graph = Vec<Vec<(usize, usize)>>;

fn build_graph(n: usize, edges: &[(usize, usize)], x: usize) -> Graph {
    let mut g: Graph = vec![Vec::new(); n * 2];
    for &(from, to) in edges.iter() {
        g[from - 1].push((to - 1, 1));
        g[to - 1 + n].push((from - 1 + n, 1));
    }
    for i in 0..n {
        g[i].push((i + n, x));
        g[i + n].push((i, x));
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
        let edges = [(1, 2), (2, 4), (3, 1), (3, 5), (4, 3), (5, 2)];
        // 反転なしで 1→2→4→3→5
        assert_eq!(resolve(5, 5, &edges), 4);
    }

    #[test]
    fn sample_02() {
        // 同じグラフでX=1: 1→2、反転(1)、裏の世界で 2→5(元の5→2の逆)
        let edges = [(1, 2), (2, 4), (3, 1), (3, 5), (4, 3), (5, 2)];
        assert_eq!(resolve(5, 1, &edges), 3);
    }

    #[test]
    fn sample_03() {
        // ジグザグ: 毎回「反転してから進む」しかない → (N-1)回の移動+(N-1)回の反転。
        // 答え 4294967299 = 7*613566756 + 7 は 2^32 超え
        let edges = [(2, 1), (2, 3), (4, 3), (4, 5), (6, 5), (6, 7), (8, 7)];
        assert_eq!(resolve(8, 613566756, &edges), 4294967299);
    }

    #[test]
    fn sample_04() {
        let edges = [
            (1, 3),
            (14, 18),
            (18, 17),
            (12, 19),
            (3, 5),
            (4, 6),
            (13, 9),
            (8, 5),
            (14, 2),
            (20, 18),
            (8, 14),
            (4, 9),
            (14, 8),
        ];
        assert_eq!(resolve(20, 5, &edges), 21);
    }

    #[test]
    fn handmade() {
        // N=2、辺は 2→1 のみ: 反転(10)してから進む(1)
        assert_eq!(resolve(2, 10, &[(2, 1)]), 11);
        // 自己ループ+多重辺が混ざっても壊れない
        assert_eq!(resolve(3, 100, &[(1, 1), (1, 2), (1, 2), (2, 3)]), 2);
        // Xが巨大なら反転せず表の世界だけで進む
        assert_eq!(resolve(4, 1_000_000_000, &[(1, 2), (2, 3), (3, 4)]), 3);
    }

    // ---- naive: 2N状態の緩和を変化がなくなるまで繰り返す(Bellman-Ford風、別原理)。
    //      到達不能なら usize::MAX/2 以上の値が返る(ランダム生成のスキップ判定用) ----
    fn naive(n: usize, x: usize, edges: &[(usize, usize)]) -> usize {
        let inf = usize::MAX / 2;
        // dist[s][v]: 世界s(0=表, 1=裏)で頂点v(0-indexed)にいる最小コスト
        let mut dist = [vec![inf; n], vec![inf; n]];
        dist[0][0] = 0;
        loop {
            let mut changed = false;
            for &(u, v) in edges {
                let (u, v) = (u - 1, v - 1);
                if dist[0][u] + 1 < dist[0][v] {
                    dist[0][v] = dist[0][u] + 1;
                    changed = true;
                }
                if dist[1][v] + 1 < dist[1][u] {
                    dist[1][u] = dist[1][v] + 1;
                    changed = true;
                }
            }
            for v in 0..n {
                if dist[0][v] + x < dist[1][v] {
                    dist[1][v] = dist[0][v] + x;
                    changed = true;
                }
                if dist[1][v] + x < dist[0][v] {
                    dist[0][v] = dist[1][v] + x;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        dist[0][n - 1].min(dist[1][n - 1])
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261108u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        let mut tested = 0;
        for _ in 0..300 {
            let n = 2 + next(7);
            let m = 1 + next(12);
            // 自己ループ・多重辺もあえて許して生成
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (1 + next(n as u64), 1 + next(n as u64)))
                .collect();
            let x = 1 + next(10);
            let expected = naive(n, x, &edges);
            if expected >= usize::MAX / 2 {
                continue; // 到達不能(問題の保証外)はスキップ
            }
            tested += 1;
            assert_eq!(
                resolve(n, x, &edges),
                expected,
                "n={n} x={x} edges={edges:?}"
            );
        }
        assert!(tested > 100, "有効ケースが少なすぎる: {tested}");
    }

    #[test]
    fn large_zigzag() {
        // 制約最大+答え最大級: サンプル3のジグザグをN=2*10^5に拡大。
        // 毎回「反転して進む」しかないので答えは (N-1)*(X+1) ≈ 2*10^14
        let n = 200_000;
        let x = 1_000_000_000;
        let mut edges: Vec<(usize, usize)> = (1..=n / 2).map(|i| (2 * i, 2 * i - 1)).collect();
        edges.extend((1..n / 2).map(|i| (2 * i, 2 * i + 1)));
        assert_eq!(resolve(n, x, &edges), (n - 1) * (x + 1));
    }

    #[test]
    fn large_chain() {
        // 制約最大: 表の世界の一直線。反転は一度も不要で答えは N-1
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, 1, &edges), n - 1);
    }
}
