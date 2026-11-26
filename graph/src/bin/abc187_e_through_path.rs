// https://atcoder.jp/contests/abc187/tasks/abc187_e

// N頂点の木。辺 i は a_i と b_i を結ぶ。全頂点の値 c は 0 スタート。
// クエリ (t, e, x):
//   t=1: 辺 e の a_e から、b_e を「通らずに」たどり着ける全頂点に +x
//   t=2: 辺 e の b_e から、a_e を「通らずに」たどり着ける全頂点に +x
// 全クエリ処理後の c_1..c_N を出力。
//
// Step 5 木の8問目: 貯めて精算(Ki) × 部分木の見方(039)の合流。考えどころ:
// - クエリの言い換え: 「b を通らずに a から届く範囲」とは、辺 e を取り除いて
//   できる2つの連結成分のうちの a 側(039 でやった見方)。start 自身も含む
// - 途中経過は聞かれず、最後に1回だけ出力 → Ki の「貯めて精算」の合図。
//   memo[v] += x は「v の部分木全体に +x」の意味だった(精算は
//   counters[v] = memo[v] + counters[親] を上から)
// - 根付き化すると、どの辺も (親, 子) になる(辺↔子の1対1、039でやった)。
//   加算したい側が「子の側」なら、それはまさに子の部分木。
//   では「親の側」だったら? そのままでは部分木ではないが、
//   全体と子の部分木の**組み合わせ**で表せないか(集合の引き算)
// - 入力の (a, b) はどちらが親か分からない(入力例2に a が子の辺がある)。
//   どう判定する?
// - 引き算が出てくるということは、memo の途中の値は負になりうる
//   (handmade の star3 がまさにそれ)。どの型で計算して、最後にどう返す?
//   値の上限: x ≤ 10^9, Q ≤ 2×10^5 → 最大 2×10^14 — 何bitあれば足りる?
// 制約: 2 ≤ N ≤ 2×10^5, 1 ≤ Q ≤ 2×10^5, t ∈ {1,2}, 1 ≤ e ≤ N-1, 1 ≤ x ≤ 10^9
//
// resolve: edges[i] = (a, b) は辺 i+1(1-indexed)。queries[i] = (t, e, x)。
// 返り値 = 処理後の c_1..c_N(非負)
fn resolve(n: usize, edges: &[(usize, usize)], queries: &[(usize, usize, usize)]) -> Vec<usize> {
    let g = build_graph(n, edges);

    let mut parent = vec![usize::MAX; n];
    let mut stack = vec![0];
    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if to == 0 {
                continue;
            }
            if parent[to] == usize::MAX {
                stack.push(to);
                parent[to] = v;
            }
        }
    }

    let mut memo = vec![0isize; n];

    for &(t, e, x) in queries {
        let (a, b) = if t == 1 {
            let (a, b) = edges[e - 1];
            (a - 1, b - 1)
        } else {
            let (b, a) = edges[e - 1];
            (a - 1, b - 1)
        };
        // bが親
        if parent[a] == b {
            memo[a] += x as isize;
        } else {
            memo[0] += x as isize;
            memo[b] -= x as isize;
        }
    }

    let mut stack = vec![0];

    let mut counters = vec![isize::MAX; n];
    counters[0] = memo[0];
    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if counters[to] == isize::MAX {
                stack.push(to);
                counters[to] = counters[v] + memo[to];
            }
        }
    }

    counters.iter().map(|&v| v as usize).collect()
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
        let edges = [(1, 2), (2, 3), (2, 4), (4, 5)];
        let queries = [(1, 1, 1), (1, 4, 10), (2, 1, 100), (2, 2, 1000)];
        assert_eq!(resolve(5, &edges, &queries), vec![11, 110, 1110, 110, 100]);
    }

    #[test]
    fn sample_02() {
        let edges = [(2, 1), (2, 3), (4, 2), (4, 5), (6, 1), (3, 7)];
        let queries = [
            (2, 2, 1),
            (1, 3, 2),
            (2, 2, 4),
            (1, 6, 8),
            (1, 3, 16),
            (2, 4, 32),
            (2, 1, 64),
        ];
        assert_eq!(resolve(7, &edges, &queries), vec![72, 8, 13, 26, 58, 72, 5]);
    }

    #[test]
    fn sample_03() {
        let edges = [
            (2, 1),
            (1, 3),
            (3, 4),
            (5, 2),
            (1, 6),
            (1, 7),
            (5, 8),
            (3, 9),
            (3, 10),
            (11, 4),
        ];
        let queries = [
            (2, 6, 688),
            (1, 10, 856),
            (1, 8, 680),
            (1, 8, 182),
            (2, 2, 452),
            (2, 4, 183),
            (2, 6, 518),
            (1, 3, 612),
            (2, 6, 339),
            (2, 3, 206),
        ];
        assert_eq!(
            resolve(11, &edges, &queries),
            vec![
                1657, 1657, 2109, 1703, 1474, 1657, 3202, 1474, 1247, 2109, 2559
            ]
        );
    }

    #[test]
    fn handmade() {
        // スター3頂点、t=1 e=1 → {1,3} に +5。
        // 「全体 − 2の部分木」の形になり、メモの途中の値が負になる
        assert_eq!(resolve(3, &[(1, 2), (1, 3)], &[(1, 1, 5)]), vec![5, 0, 5]);
        // a が子になっている辺: 辺 (2,1) の t=1 は 2 の部分木side
        assert_eq!(resolve(3, &[(2, 1), (1, 3)], &[(1, 1, 7)]), vec![0, 7, 0]);
    }

    // ---- naive: クエリごとに禁止頂点を避けて探索し、届いた頂点に加算(定義そのまま) ----
    fn naive(n: usize, edges: &[(usize, usize)], queries: &[(usize, usize, usize)]) -> Vec<usize> {
        let mut adj = vec![Vec::new(); n + 1];
        for &(a, b) in edges {
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut c = vec![0usize; n + 1];
        for &(t, e, x) in queries {
            let (a, b) = edges[e - 1];
            let (start, forbidden) = if t == 1 { (a, b) } else { (b, a) };
            let mut seen = vec![false; n + 1];
            seen[forbidden] = true;
            seen[start] = true;
            let mut stack = vec![start];
            while let Some(v) = stack.pop() {
                c[v] += x;
                for &to in adj[v].iter() {
                    if !seen[to] {
                        seen[to] = true;
                        stack.push(to);
                    }
                }
            }
        }
        c[1..].to_vec()
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261122u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..300 {
            let n = 2 + next(20);
            let mut label: Vec<usize> = (0..=n).collect();
            for v in (3..=n).rev() {
                let r = 2 + next((v - 1) as u64);
                label.swap(v, r);
            }
            // 辺の向きもランダム(a が親とは限らない)
            let edges: Vec<(usize, usize)> = (2..=n)
                .map(|v| {
                    let p = 1 + next((v - 1) as u64);
                    if next(2) == 0 {
                        (label[p], label[v])
                    } else {
                        (label[v], label[p])
                    }
                })
                .collect();
            let queries: Vec<(usize, usize, usize)> = (0..10)
                .map(|_| (1 + next(2), 1 + next((n - 1) as u64), 1 + next(100)))
                .collect();
            assert_eq!(
                resolve(n, &edges, &queries),
                naive(n, &edges, &queries),
                "n={n} edges={edges:?} queries={queries:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 鎖 2×10^5。全辺に t=2(深い側)x=10^9 → c_v = (v−1)×10^9。
        // 最大 2×10^14 — 型の選択ミスはここで露見する
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        let queries: Vec<(usize, usize, usize)> = (1..n).map(|i| (2, i, 1_000_000_000)).collect();
        let expected: Vec<usize> = (0..n).map(|v| v * 1_000_000_000).collect();
        assert_eq!(resolve(n, &edges, &queries), expected);
    }

    #[test]
    fn wide_star() {
        // スター 2×10^5。全辺に t=1(中心側 = 全体 − 葉)x=1
        // → c_1 = n−1、各葉 = n−2。「全体+引き算」経路の最悪ケース
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        let queries: Vec<(usize, usize, usize)> = (1..n).map(|i| (1, i, 1)).collect();
        let mut expected = vec![n - 2; n];
        expected[0] = n - 1;
        assert_eq!(resolve(n, &edges, &queries), expected);
    }
}
