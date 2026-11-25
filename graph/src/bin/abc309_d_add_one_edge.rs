// https://atcoder.jp/contests/abc309/tasks/abc309_d

// 無向グラフ(頂点 N1+N2 個、辺 M 本)。頂点 1..N1 は互いに連結、
// 頂点 N1+1..N1+N2 も互いに連結、頂点 1 と頂点 N1+N2 は非連結。
// 成分1の頂点 u と成分2の頂点 v を選んで辺 (u,v) を**ちょうど1本**追加する。
// 追加後の 1 から N1+N2 への最短距離(辺数) d を最大化せよ。
//
// Step 5 のおまけ(軽め、総括前の復習): 距離埋め2回+足し算。考えどころ:
// - 1 と N1+N2 はもともと非連結で、橋は追加した1本だけ。すると追加後の
//   最短路は**必ずその辺を通る**。d を u, v の言葉で式にすると?
//   その式の最大化は u と v で独立に行える — 各成分で何を求めればいい?
// - **今回は木ではない**。M は最大3×10^5 で頂点数より多く、閉路がある。
//   いつもの距離埋め(スタック1本)は木専用の道具 — 閉路があると何が壊れる?
//   handmade の五角形が実例: スタックだと遠回りした値が先に書き込まれて
//   d=3 の幻が出る(正しくは2)。「最初に書いた距離が最終値」が成立する
//   条件は何だったか。正しい道具は ABC007 C の迷路で使った
// - 成分1の探索は成分2に漏れない(非連結だから勝手に止まる)。番兵は結果の
//   読み取り範囲にだけ注意
// 制約: 1 ≤ N1, N2 ≤ 1.5×10^5, 0 ≤ M ≤ 3×10^5, 重複辺なし,
//       成分1内は連結・成分2内は連結・1 と N1+N2 は非連結が保証
//
// resolve: 頂点は 1..=n1+n2(1-indexed)。edges[i] = (a, b) は無向辺。
// 返り値 = d の最大値
fn resolve(n1: usize, n2: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n1 + n2, edges);
    find_max_dist(n1 + n2, 0, &g) + find_max_dist(n1 + n2, n1 + n2 - 1, &g) + 1
}

fn find_max_dist(n: usize, start: usize, g: &Graph) -> usize {
    let mut dist = vec![usize::MAX; n];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;

    while let Some(v) = queue.pop_front() {
        for &to in g[v].iter() {
            if dist[to] == usize::MAX {
                dist[to] = dist[v] + 1;
                queue.push_back(to);
            }
        }
    }

    *dist.iter().filter(|&&v| v != usize::MAX).max().unwrap()
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
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn sample_01() {
        let edges = [(1, 2), (2, 3), (4, 5), (4, 6), (1, 3), (6, 7)];
        assert_eq!(resolve(3, 4, &edges), 5);
    }

    #[test]
    fn sample_02() {
        let edges = [
            (10, 11),
            (4, 5),
            (10, 12),
            (1, 2),
            (1, 5),
            (5, 6),
            (2, 4),
            (3, 5),
            (9, 10),
            (2, 5),
            (1, 4),
            (11, 12),
            (9, 12),
            (8, 9),
            (5, 7),
            (3, 7),
            (3, 6),
            (3, 4),
            (8, 12),
            (9, 11),
        ];
        assert_eq!(resolve(7, 5, &edges), 4);
    }

    #[test]
    fn handmade() {
        // 五角形 C5 + 孤立頂点6: 1からの正しい最遠距離は2(答え 2+0+1=3)。
        // スタックDFSの距離埋めだと遠回りの3が書き込まれて4と誤答する
        let penta = [(1, 2), (2, 3), (3, 4), (4, 5), (1, 5)];
        assert_eq!(resolve(5, 1, &penta), 3);
        // M=0: 両成分とも孤立頂点1個ずつ。追加辺そのものが最短路で d=1
        assert_eq!(resolve(1, 1, &[]), 1);
    }

    // ---- naive: 追加辺を全組み合わせ試し、都度BFSで最短距離(定義そのまま) ----
    fn bfs_dist(n: usize, adj: &[Vec<usize>], s: usize) -> Vec<usize> {
        let mut d = vec![usize::MAX; n + 1];
        d[s] = 0;
        let mut q = VecDeque::from([s]);
        while let Some(v) = q.pop_front() {
            for &to in adj[v].iter() {
                if d[to] == usize::MAX {
                    d[to] = d[v] + 1;
                    q.push_back(to);
                }
            }
        }
        d
    }

    fn naive(n1: usize, n2: usize, edges: &[(usize, usize)]) -> usize {
        let n = n1 + n2;
        let mut best = 0;
        for u in 1..=n1 {
            for v in (n1 + 1)..=n {
                let mut adj = vec![Vec::new(); n + 1];
                for &(a, b) in edges.iter().chain([(u, v)].iter()) {
                    adj[a].push(b);
                    adj[b].push(a);
                }
                best = best.max(bfs_dist(n, &adj, 1)[n]);
            }
        }
        best
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261125u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        // 頂点 offset+1..=offset+sz の連結グラフ(ランダム木+余り辺で閉路つき)
        fn random_component(
            sz: usize,
            offset: usize,
            es: &mut HashSet<(usize, usize)>,
            next: &mut impl FnMut(u64) -> usize,
        ) {
            let mut label: Vec<usize> = (0..sz).map(|i| offset + 1 + i).collect();
            for i in (1..sz).rev() {
                let r = next((i + 1) as u64);
                label.swap(i, r);
            }
            for i in 1..sz {
                let p = next(i as u64);
                let (a, b) = (label[p], label[i]);
                es.insert((a.min(b), a.max(b)));
            }
            for _ in 0..next((sz + 1) as u64) {
                let x = label[next(sz as u64)];
                let y = label[next(sz as u64)];
                if x != y {
                    es.insert((x.min(y), x.max(y)));
                }
            }
        }
        for _ in 0..300 {
            let n1 = 1 + next(8);
            let n2 = 1 + next(8);
            let mut es = HashSet::new();
            random_component(n1, 0, &mut es, &mut next);
            random_component(n2, n1, &mut es, &mut next);
            let edges: Vec<(usize, usize)> = es.into_iter().collect();
            assert_eq!(
                resolve(n1, n2, &edges),
                naive(n1, n2, &edges),
                "n1={n1} n2={n2} edges={edges:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 鎖 × 2(各 1.5×10^5): 答え = (n1-1) + (n2-1) + 1
        let n1 = 150_000;
        let n2 = 150_000;
        let mut edges: Vec<(usize, usize)> = (1..n1).map(|i| (i, i + 1)).collect();
        edges.extend((n1 + 1..n1 + n2).map(|i| (i, i + 1)));
        assert_eq!(resolve(n1, n2, &edges), 299_999);
    }

    #[test]
    fn wide_star() {
        // スター × 2: 成分1は中心が頂点1(最遠1)、成分2は中心が N1+1 で
        // N1+N2 は葉(最遠2)。答え = 1 + 2 + 1 = 4
        let n1 = 150_000;
        let n2 = 150_000;
        let mut edges: Vec<(usize, usize)> = (2..=n1).map(|v| (1, v)).collect();
        edges.extend((n1 + 2..=n1 + n2).map(|v| (n1 + 1, v)));
        assert_eq!(resolve(n1, n2, &edges), 4);
    }
}
