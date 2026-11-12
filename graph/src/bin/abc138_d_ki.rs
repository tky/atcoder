// https://atcoder.jp/contests/abc138/tasks/abc138_d

// 頂点1を根とするN頂点の根付き木(i番目の辺は a_i と b_i を結ぶ)。全頂点の
// カウンターは0から始まる。操作j: 「頂点 p_j を根とする**部分木**に含まれる
// すべての頂点のカウンターに x_j を足す」。Q回の操作後の全頂点の値を出力。
//
// Step 5 木の3問目: 根付き木を「親→子の順」で処理する(木DPの入口)。考えどころ:
// - 愚直にクエリごとに部分木を塗ると O(NQ) = 4*10^10 → 間に合わない
// - 観測: 答えを聞かれるのは**全操作が終わったあと1回だけ**(途中経過は不要)。
//   なら操作は貯めておいて、最後にまとめて精算できるのでは?
// - 「pの部分木全体にxを足す」は、pに1回メモするだけで済ませたい。
//   最後に根から1回だけ木を下りながら、counter[v] = memo[v] + counter[親v]
//   と引き継いでいくと全頂点に行き渡る — なぜこれで正しい?
//   (操作(p, x)がvに効く ⟺ pはvの祖先またはv自身 ⟺ 根→vの経路上にそのメモがある。
//    つまりcounter[v]は「根→v経路上のメモの総和」)
// - 処理順の制約: counter[親]が**確定してから**子が読む必要がある。
//   根からのDFS(スタック)とBFS(キュー)、どちらなら保証される? 両方? なぜ?
// - 辺は a_i < b_i で与えられるが、「aが親」とは限らない(handmadeに反例を
//   仕込んである)。親子の向きは根から辿って初めて決まる — abc070_dでやった話
// - 答えの最大は Q*x = 2*10^9。i32だと溢れる(u32はぎりぎり、usizeなら安心)
// 制約: 2 ≤ N ≤ 2*10^5, 1 ≤ Q ≤ 2*10^5, 1 ≤ a_i < b_i ≤ N,
//       1 ≤ p_j ≤ N, 1 ≤ x_j ≤ 10^4, 与えられるグラフは木
//
// resolve: edges[i] = (a, b) は a-b を結ぶ無向辺、queries[j] = (p, x)(1-indexed)。
// 返り値 = counters[v](0-indexed)が全操作後の頂点v+1の値
fn resolve(n: usize, edges: &[(usize, usize)], queries: &[(usize, usize)]) -> Vec<usize> {
    let g = build_graph(n, edges);
    let mut memo = vec![0; n];
    for &(p, c) in queries {
        memo[p - 1] += c;
    }
    let mut counters = vec![usize::MAX; n];

    let mut stack = vec![0];
    counters[0] = memo[0];

    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if counters[to] == usize::MAX {
                stack.push(to);
                counters[to] = counters[v] + memo[to];
            }
        }
    }
    counters
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
        let edges = [(1, 2), (2, 3), (2, 4)];
        let queries = [(2, 10), (1, 100), (3, 1)];
        assert_eq!(resolve(4, &edges, &queries), vec![100, 110, 111, 110]);
    }

    #[test]
    fn sample_02() {
        let edges = [(1, 2), (1, 3), (2, 4), (3, 6), (2, 5)];
        let queries = [(1, 10), (1, 10)];
        assert_eq!(resolve(6, &edges, &queries), vec![20; 6]);
    }

    #[test]
    fn handmade() {
        // a < b でも a が親とは限らない: 木は 1-3-2(辺 (1,3) と (2,3))。
        // 頂点3の部分木は {3, 2} → 「aを親」と決め打ちすると [0, 0, 5] になる
        let edges = [(1, 3), (2, 3)];
        assert_eq!(resolve(3, &edges, &[(3, 5)]), vec![0, 5, 5]);
        // 葉への操作は葉だけ
        let edges = [(1, 2)];
        assert_eq!(resolve(2, &edges, &[(2, 7)]), vec![0, 7]);
        // 同じ頂点への操作が複数回
        let edges = [(1, 2)];
        assert_eq!(resolve(2, &edges, &[(2, 1), (1, 2), (2, 4)]), vec![2, 7]);
    }

    // ---- naive: クエリごとに部分木を実際に塗る(定義そのまま、O(NQ)) ----
    fn naive(n: usize, edges: &[(usize, usize)], queries: &[(usize, usize)]) -> Vec<usize> {
        // 根から辿って子リストを作る
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in edges {
            adj[a - 1].push(b - 1);
            adj[b - 1].push(a - 1);
        }
        let mut children = vec![Vec::new(); n];
        let mut seen = vec![false; n];
        seen[0] = true;
        let mut stack = vec![0];
        while let Some(v) = stack.pop() {
            for &to in adj[v].iter() {
                if !seen[to] {
                    seen[to] = true;
                    children[v].push(to);
                    stack.push(to);
                }
            }
        }
        let mut counters = vec![0; n];
        for &(p, x) in queries {
            let mut stack = vec![p - 1];
            while let Some(v) = stack.pop() {
                counters[v] += x;
                stack.extend(children[v].iter());
            }
        }
        counters
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261112u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..300 {
            let n = 2 + next(15);
            // 木の形: 頂点v(2..=n)の親を 1..v から選ぶ。そのあとラベルを
            // シャッフルして「親の番号 < 子の番号」とは限らない木にする
            // (根のラベルだけは1に固定)
            let mut label: Vec<usize> = (0..=n).collect();
            for v in (3..=n).rev() {
                let r = 2 + next((v - 1) as u64); // 2..=v
                label.swap(v, r);
            }
            let edges: Vec<(usize, usize)> = (2..=n)
                .map(|v| {
                    let p = 1 + next((v - 1) as u64); // 1..v
                    (label[p].min(label[v]), label[p].max(label[v]))
                })
                .collect();
            let q = 1 + next(10);
            let queries: Vec<(usize, usize)> = (0..q)
                .map(|_| (1 + next(n as u64), 1 + next(100)))
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
        // 深さ最悪: 鎖 1-2-...-N。全頂点に (v, 1) を1回ずつ →
        // counter[v] = 根→vの経路上のメモの数 = v
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        let queries: Vec<(usize, usize)> = (1..=n).map(|v| (v, 1)).collect();
        let expected: Vec<usize> = (1..=n).collect();
        assert_eq!(resolve(n, &edges, &queries), expected);
    }

    #[test]
    fn wide_star() {
        // 幅最悪: 中心1に葉2..=N。全頂点に (v, 1) を1回ずつ →
        // 中心は1、各葉は 1(根の分) + 1(自分の分) = 2
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        let queries: Vec<(usize, usize)> = (1..=n).map(|v| (v, 1)).collect();
        let mut expected = vec![2; n];
        expected[0] = 1;
        assert_eq!(resolve(n, &edges, &queries), expected);
    }

    #[test]
    fn large_values() {
        // 値の最悪: (1, 10^4) を 2*10^5 回 → 全頂点 2*10^9(i32だと溢れる)
        let n = 2;
        let edges = [(1, 2)];
        let queries: Vec<(usize, usize)> = (0..200_000).map(|_| (1, 10_000)).collect();
        assert_eq!(resolve(n, &edges, &queries), vec![2_000_000_000; 2]);
    }
}
