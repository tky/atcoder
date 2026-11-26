// https://atcoder.jp/contests/abc204/tasks/abc204_c

// N都市・M本の道路。道路 i は都市 A_i から B_i への**一方通行**(有向!)。
// 「都市 s から0本以上の道路で都市 t に到達できる」ような組 (s, t) の個数を
// 数えよ。s = t も数える。(s,t) と (t,s) は別の組。
//
// 復習セット1問目(軽め): 全点からの到達可能性。考えどころ:
// - 「0本以上」= 自分自身 (s,s) は無条件で到達可能
// - 答え = Σ_s (s から到達できる頂点数)。つまり「s から探索して数える」を
//   全ての s についてやるだけ。計算量は? N ≤ 2000, M ≤ 2000 で間に合うか
//   見積もってから書くこと(1回の探索のコスト × 回数)
// - 有向なので build_graph は片方向だけ積む(ABC340 D 以来2回目の有向)。
//   うっかり両方向に積むと… handmade の one-way が捕まえる
// - 探索ごとに「印」の配列をリセットする必要がある。毎回 vec! で作り直すのと
//   使い回すのとどちらでも通るが、確保のコストも見積もりに入れておくこと
// 制約: 2 ≤ N ≤ 2000, 0 ≤ M ≤ min(2000, N(N-1)), A_i ≠ B_i, (A_i,B_i)は相異なる
//
// resolve: edges[i] = (a, b) は a→b の有向辺(1-indexed)。返り値 = 組の個数
fn resolve(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);

    let mut count = 0;

    for start in 0..n {
        let mut visited = vec![false; n];
        visited[start] = true;
        let mut stack = vec![start];
        // 道路が0本でもいいので
        count += 1;

        while let Some(v) = stack.pop() {
            for &to in g[v].iter() {
                if !visited[to] {
                    visited[to] = true;
                    stack.push(to);
                    count += 1;
                }
            }
        }
    }

    count
}

type Graph = Vec<Vec<usize>>;

fn build_graph(n: usize, edges: &[(usize, usize)]) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];

    for &(from, to) in edges.iter() {
        g[from - 1].push(to - 1);
    }
    g
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn sample_01() {
        // 2↔3 が互いに行き来できる。1へは誰も戻れない
        assert_eq!(resolve(3, &[(1, 2), (2, 3), (3, 2)]), 7);
    }

    #[test]
    fn sample_02() {
        // 道路なし → (s,s) の3組だけ
        assert_eq!(resolve(3, &[]), 3);
    }

    #[test]
    fn sample_03() {
        // 一周の閉路 → 全組到達可能 = 4×4
        assert_eq!(resolve(4, &[(1, 2), (2, 3), (3, 4), (4, 1)]), 16);
    }

    #[test]
    fn handmade() {
        // 一方通行の確認: 1→2 のみ。(1,1),(2,2),(1,2) の3組。
        // 無向に積んでしまうと (2,1) も数えて4になる
        assert_eq!(resolve(2, &[(1, 2)]), 3);
    }

    // ---- naive: Floyd-Warshall型の到達可能性閉包(定義そのまま、小さいn専用) ----
    fn naive(n: usize, edges: &[(usize, usize)]) -> usize {
        let mut reach = vec![vec![false; n + 1]; n + 1];
        for v in 1..=n {
            reach[v][v] = true;
        }
        for &(a, b) in edges {
            reach[a][b] = true;
        }
        for k in 1..=n {
            for i in 1..=n {
                for j in 1..=n {
                    if reach[i][k] && reach[k][j] {
                        reach[i][j] = true;
                    }
                }
            }
        }
        (1..=n)
            .map(|i| (1..=n).filter(|&j| reach[i][j]).count())
            .sum()
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261126u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..300 {
            let n = 2 + next(10);
            let m = next((16.min(n * (n - 1)) + 1) as u64);
            let mut es = HashSet::new();
            let mut tries = 0;
            while es.len() < m && tries < 100 {
                tries += 1;
                let a = 1 + next(n as u64);
                let b = 1 + next(n as u64);
                if a != b {
                    es.insert((a, b));
                }
            }
            let edges: Vec<(usize, usize)> = es.into_iter().collect();
            assert_eq!(
                resolve(n, &edges),
                naive(n, &edges),
                "n={n} edges={edges:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 有向の鎖 1→2→…→2000: 答え = n(n+1)/2
        let n = 2000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &edges), 2_001_000);
    }

    #[test]
    fn wide_star() {
        // 中心→葉のスター: 中心からは全部(n)、葉からは自分だけ(1)×(n-1)
        let n = 2000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        assert_eq!(resolve(n, &edges), 2 * n - 1);
    }
}
