// https://atcoder.jp/contests/abc252/tasks/abc252_e

use std::cmp::Reverse;

// N都市M道路(連結・単純・無向)。道路iは都市 A_i と B_i を距離 C_i で結ぶ。
// 連結を保ったまま N-1 本だけ残す。残した道路だけで都市1から都市iへ移動する
// 距離を d_i とするとき、d_2+...+d_N を最小化する選び方を1つ出力する
// (残す道路の番号を順不同で)。**複数解はどれでも正解** → テストは検証器方式(ABC168 Dと同じ)。
//
// Dijkstraひねり2問目: 出力のひねり(経路復元)。考えどころ:
// - 「和の最小化」と言われると組合せ探索に見えるが、まず各 d_i を単独で考える。
//   どう選んでも d_i は**元のグラフでの最短距離 dist_i より小さくできない**(なぜ?)。
//   では、全頂点が**同時に** dist_i を達成する選び方は存在するか?
//   存在するなら、それ以上の工夫は要らない(下界の総和を達成=文句なしの最小)
// - ヒントは ABC168 D の origin(道標)。あのときBFSで「どの頂点から来たか」を覚えた。
//   今回はDijkstraで「dist[y] を最後に更新したのは**どの辺か**」を覚える。
//   緩和のたびに上書きしていくと、全部確定したあとに残っているのは何か?
// - 頂点2..Nそれぞれに記録された辺を集めるとちょうどN-1本。これで連結になる理由も
//   一言で言えるか?(どの頂点も、記録された辺をたどっていくと必ず…)
// - 辺の番号が答えになるので、隣接リストに番号も乗せる: (to, cost, 辺番号) など
// - Σd_i は計算も出力もしない(しなくて済むのがこの問題の種明かし)。
//   距離は最大 ~2*10^14 → 前問と同じく64bit、番兵usize::MAX
// 制約: 2 ≤ N ≤ 2*10^5, N-1 ≤ M ≤ 2*10^5, 1 ≤ A < B ≤ N, 1 ≤ C ≤ 10^9,
//       (A,B)は全ペア相異(単純)、連結
//
// resolve: edges[j] = 道路j+1 = (a, b, c) で a,b は1-indexed。
// 返り値 = 残す道路の番号(1-indexed)N-1個、順不同
fn resolve(n: usize, edges: &[(usize, usize, usize)]) -> Vec<usize> {
    let mut heap = std::collections::BinaryHeap::new();
    // (距離、頂点）
    heap.push(Reverse((0, 0)));

    let mut distances = vec![usize::MAX; n];
    distances[0] = 0;

    let graph = build_graph(n, edges);

    let mut routes = vec![usize::MAX; n];

    while let Some(Reverse((dist, v))) = heap.pop() {
        if dist > distances[v] {
            continue;
        }

        for &(index, y, w) in graph[v].iter() {
            if dist + w < distances[y] {
                routes[y] = index;
                distances[y] = dist + w;
                heap.push(Reverse((dist + w, y)));
            }
        }
    }

    routes[1..].iter().map(|index| index + 1).collect()
}

type Graph = Vec<Vec<(usize, usize, usize)>>;

type Edge = (usize, usize, usize);

type Edges = [Edge];

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for (index, &(from, to, w)) in edges.iter().enumerate() {
        g[from - 1].push((index, to - 1, w));
        g[to - 1].push((index, from - 1, w));
    }
    g
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- 検証器: 選んだ道路が「N-1本・番号妥当・重複なし・連結」で、
    //      d_2+...+d_N が expected に一致するかを調べる(緩和の繰り返し=別原理) ----
    fn dist_sum(n: usize, edges: &[(usize, usize, usize)], chosen: &[usize]) -> Option<usize> {
        let m = edges.len();
        if chosen.len() != n - 1 {
            return None;
        }
        let mut used = vec![false; m];
        for &j in chosen {
            if j == 0 || j > m || used[j - 1] {
                return None; // 範囲外 or 重複
            }
            used[j - 1] = true;
        }
        let inf = usize::MAX / 2;
        let mut dist = vec![inf; n];
        dist[0] = 0;
        loop {
            let mut changed = false;
            for &j in chosen {
                let (a, b, c) = edges[j - 1];
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
        if dist.iter().any(|&d| d >= inf) {
            return None; // 非連結
        }
        Some(dist[1..].iter().sum())
    }

    #[test]
    fn sample_01() {
        let edges = [(1, 2, 1), (2, 3, 2), (1, 3, 10)];
        // 最小和4(道路1,2のとき d_2=1, d_3=3)。検算済み
        assert_eq!(dist_sum(3, &edges, &resolve(3, &edges)), Some(4));
    }

    #[test]
    fn sample_02() {
        let edges = [
            (1, 2, 1),
            (1, 3, 1),
            (1, 4, 1),
            (2, 3, 1),
            (2, 4, 1),
            (3, 4, 1),
        ];
        // 最小和3(スターを作る)。検算済み
        assert_eq!(dist_sum(4, &edges, &resolve(4, &edges)), Some(3));
    }

    #[test]
    fn handmade() {
        // N=2: 残せる道路は1本しかない
        assert_eq!(resolve(2, &[(1, 2, 5)]), vec![1]);
        // 最適解が一意: 道路3(重さ10)を残す理由がない → 必ず {1,2}
        let edges = [(1, 2, 1), (1, 3, 1), (2, 3, 10)];
        let mut r = resolve(3, &edges);
        r.sort_unstable();
        assert_eq!(r, vec![1, 2]);
        // タイ: d_3=2 は道路2直行でも道路1+3経由でも達成できる → {1,2} と {1,3} の両方が正解
        let edges = [(1, 2, 1), (1, 3, 2), (2, 3, 1)];
        assert_eq!(dist_sum(3, &edges, &resolve(3, &edges)), Some(3));
    }

    // ---- naive: N-1本の選び方を総当たりし、連結なものの中で最小和を求める ----
    fn brute_min(n: usize, edges: &[(usize, usize, usize)]) -> usize {
        let m = edges.len();
        let mut best = usize::MAX;
        for mask in 0u32..(1 << m) {
            if mask.count_ones() as usize != n - 1 {
                continue;
            }
            let chosen: Vec<usize> = (0..m)
                .filter(|i| mask >> i & 1 == 1)
                .map(|i| i + 1)
                .collect();
            if let Some(s) = dist_sum(n, edges, &chosen) {
                best = best.min(s);
            }
        }
        best
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261104u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..150 {
            let n = 2 + next(6); // 2..=7 (総当たりが効く範囲)
            // 木で連結を保証し、余分な辺を足す(A<B・全ペア相異・C≥1の制約を守る)
            let mut set = std::collections::HashSet::new();
            for v in 2..=n {
                let u = 1 + next(v as u64 - 1);
                set.insert((u.min(v), u.max(v)));
            }
            for _ in 0..next(n as u64) {
                let x = 1 + next(n as u64);
                let y = 1 + next(n as u64);
                if x != y {
                    set.insert((x.min(y), x.max(y)));
                }
            }
            let edges: Vec<(usize, usize, usize)> =
                set.into_iter().map(|(a, b)| (a, b, 1 + next(20))).collect();
            let r = resolve(n, &edges);
            assert_eq!(
                dist_sum(n, &edges, &r),
                Some(brute_min(n, &edges)),
                "n={n} edges={edges:?} result={r:?}"
            );
        }
    }

    #[test]
    fn large_chain() {
        // 制約最大: 一直線でM=N-1 → 全道路を残すしかない
        let n = 200_000;
        let edges: Vec<(usize, usize, usize)> = (1..n).map(|i| (i, i + 1, 1)).collect();
        let mut r = resolve(n, &edges);
        r.sort_unstable();
        assert_eq!(r, (1..n).collect::<Vec<usize>>());
    }

    #[test]
    fn large_heavy_shortcuts() {
        // 鎖(重さ1)+ 都市1から各都市への重い直行便(10^9)でM≈2*10^5。
        // どの頂点も鎖経由(≤10^5)が直行便(10^9)より近い → 最適解は鎖のみで一意
        let n = 100_000;
        let mut edges: Vec<(usize, usize, usize)> = (1..n).map(|i| (i, i + 1, 1)).collect();
        edges.extend((3..=n).map(|i| (1, i, 1_000_000_000)));
        let mut r = resolve(n, &edges);
        r.sort_unstable();
        assert_eq!(r, (1..n).collect::<Vec<usize>>());
    }
}
