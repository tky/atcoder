// https://atcoder.jp/contests/abc340/tasks/abc340_d

use std::cmp::Reverse;

// ステージ1..Nのゲーム。最初はステージ1だけ遊べる。ステージi(1≤i≤N-1)では
//   - A_i 秒かけてクリア → ステージ i+1 が遊べるようになる
//   - B_i 秒かけてクリア → ステージ X_i が遊べるようになる
// のどちらかを選べる。ステージNを遊べるようになるまでの最短秒数を出力。
//
// Dijkstraひねり3問目: モデル化のひねり。考えどころ:
// - 今までと違い、辺リストが与えられない。「何が頂点で、何が辺か」を自分で決める。
//   「◯秒かけて△△が遊べるようになる」という文は、グラフの言葉に訳すと何か?
// - 訳すと、このグラフは**有向**になる(今シリーズ初)。ワープは一方通行 —
//   ステージ5から2へ飛べても、2から5へは飛べない。build_graphを前問のコピーで
//   済ませる(両方向にpushする)と何が起きるか、handmadeに反例を仕込んである
// - Dijkstra本体は有向でもそのまま動く。pop時確定の議論のどこにも「無向」を
//   使った箇所がないことを確認しておくと安心して流用できる
// - X_i = i(自分に戻る)や X_i = i+1 もありうる。自己ループは最短距離に影響するか?
// - 頂点重みは今回なし → 始点の初期値は前問と変わる
// - 答えは最大 ~2*10^14(入出力例3がわざわざ32bit超えを見せてくる)→ 64bit
// 制約: 2 ≤ N ≤ 2*10^5, 1 ≤ A_i,B_i ≤ 10^9, 1 ≤ X_i ≤ N, 全て整数
//
// resolve: stages[i] = ステージi+1の (A, B, X)。Xは1-indexed。
// 返り値 = ステージNを遊べるようになるまでの最短秒数
fn resolve(n: usize, stages: &[(usize, usize, usize)]) -> usize {
    let mut heap = std::collections::BinaryHeap::new();
    // (距離、頂点)
    heap.push(Reverse((0, 0)));

    // 0からの最小距離
    let mut distances = vec![usize::MAX; n];
    distances[0] = 0;

    let graph = build_graph(n, stages);

    while let Some(Reverse((dist, v))) = heap.pop() {
        if dist > distances[v] {
            continue;
        }

        for &(to, w) in graph[v].iter() {
            let next = dist + w;
            if next < distances[to] {
                distances[to] = next;
                heap.push(Reverse((next, to)));
            }
        }
    }

    distances[n - 1]
}

type Graph = Vec<Vec<(usize, usize)>>;

fn build_graph(n: usize, stages: &[(usize, usize, usize)]) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for (from, &(w, jump_cost, jump_node)) in stages.iter().enumerate() {
        g[from].push((from + 1, w));
        g[from].push((jump_node - 1, jump_cost));
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
        let stages = [(100, 200, 3), (50, 10, 1), (100, 200, 5), (150, 1, 2)];
        assert_eq!(resolve(5, &stages), 350);
    }

    #[test]
    fn sample_02() {
        // 後退ワープを乗り継ぐのが最短(10ステージを直進1000秒ではなくワープ10秒で縫う)
        let stages = [
            (1000, 10, 9),
            (1000, 10, 10),
            (1000, 10, 2),
            (1000, 10, 3),
            (1000, 10, 4),
            (1000, 10, 5),
            (1000, 10, 6),
            (1000, 10, 7),
            (1000, 10, 8),
        ];
        assert_eq!(resolve(10, &stages), 90);
    }

    #[test]
    fn sample_03() {
        // 32bit超え
        let w = 1_000_000_000;
        let stages = [(w, w, 1); 5];
        assert_eq!(resolve(6, &stages), 5_000_000_000);
    }

    #[test]
    fn handmade() {
        // N=2: 直進かワープか。X_1=2ならワープも2に着く
        assert_eq!(resolve(2, &[(5, 3, 2)]), 3);
        assert_eq!(resolve(2, &[(5, 3, 1)]), 5); // ワープは1に戻るだけ → 直進しかない
        // 有向の罠: ステージ2の戻りワープ(2→1, 1秒)を**逆走できてしまう**実装(無向)は
        // 1→2を1秒で進んで 6 と答える。正解は 1→2(100) + 2→3(5) = 105
        assert_eq!(resolve(3, &[(100, 100, 1), (5, 1, 1)]), 105);
        // 前進ワープ: 1 -(B=1)→ 3 -(A=1000)→ 4 で 1001(直進は 1000+1+1000)
        assert_eq!(
            resolve(4, &[(1000, 1, 3), (1, 1000, 1), (1000, 1000, 1)]),
            1001
        );
        // 自己ループ(X_i = i)だらけでも直進で届く
        assert_eq!(resolve(3, &[(7, 1, 1), (8, 2, 2)]), 15);
    }

    // ---- naive: 有向辺の緩和を変化がなくなるまで繰り返す(Bellman-Ford風、別原理) ----
    fn naive(n: usize, stages: &[(usize, usize, usize)]) -> usize {
        let inf = usize::MAX / 2;
        let mut dist = vec![inf; n];
        dist[0] = 0;
        loop {
            let mut changed = false;
            for (i, &(a, b, x)) in stages.iter().enumerate() {
                if dist[i] + a < dist[i + 1] {
                    dist[i + 1] = dist[i] + a;
                    changed = true;
                }
                if dist[i] + b < dist[x - 1] {
                    dist[x - 1] = dist[i] + b;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        dist[n - 1]
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
        for _ in 0..200 {
            let n = 2 + next(25);
            let stages: Vec<(usize, usize, usize)> = (0..n - 1)
                .map(|_| (1 + next(20), 1 + next(20), 1 + next(n as u64)))
                .collect();
            assert_eq!(
                resolve(n, &stages),
                naive(n, &stages),
                "n={n} stages={stages:?}"
            );
        }
    }

    #[test]
    fn large_chain() {
        // 制約最大: 直進1秒、ワープは高コストで1に戻るだけ → 答えは N-1
        let n = 200_000;
        let stages: Vec<(usize, usize, usize)> =
            (0..n - 1).map(|_| (1, 1_000_000_000, 1)).collect();
        assert_eq!(resolve(n, &stages), n - 1);
    }

    #[test]
    fn large_weights() {
        // 重み最大: 入出力例3の制約最大版。答えは (N-1)*10^9 ≈ 2*10^14
        let n = 200_000;
        let w = 1_000_000_000;
        let stages: Vec<(usize, usize, usize)> = (0..n - 1).map(|_| (w, w, 1)).collect();
        assert_eq!(resolve(n, &stages), (n - 1) * w);
    }
}
