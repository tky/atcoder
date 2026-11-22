// https://atcoder.jp/contests/abc202/tasks/abc202_e

// 根付き木(根は頂点1、頂点i(2≤i≤N)の親はP_i、P_i < i が保証)。
// クエリ (U, D): 「uから根へのパス(端点含む)上に U がある」かつ
// 「uから根へのパスの辺数がちょうど D」を満たす頂点 u の個数を答える。
//
// Step 5 木の7問目: 部分木の区間化 + 二分探索(binary_search編との合流点)。考えどころ:
// - クエリの言い換え: 「根へのパス上に U がある」⇔「u は U の部分木の中(U自身も含む)」。
//   「辺数ちょうど D」⇔「u の深さ = D」。つまり各クエリは
//   **「U の部分木にいる、深さちょうど D の頂点の個数」**
// - 愚直: クエリごとに部分木を走査 → O(NQ) = 4×10^10 で間に合わない
// - 条件が2つ(部分木 ∧ 深さ)。深さ D の頂点だけを集めたリストから
//   「U の部分木にいるもの」を速く数えたい。もし部分木への所属が
//   「番号がある区間に入っているか」に変換できたら、ソート済みリスト×区間
//   = 二分探索2回で数えられる(partition_point、二分探索編の復習)
// - その変換が**行きがけ順**: 行きがけ順で頂点に番号を振り直すと、どの頂点の
//   部分木も連続区間になる(なぜ? スタック走査が部分木を描き切ってから
//   隣へ移ることを考えてみて)。区間の幅は前問の size そのもの:
//   v の部分木 = 行きがけ順の [in[v], in[v] + size[v])
// - 入力の形に注目: 親が最初から与えられている(しかも P_i < i)。
//   無向辺の build_graph と訪問印は今回も必要だろうか?
// - 深さ別リストを行きがけ順に積んでいくと、各リストのソートは必要?
// - D が木の最大深さを超えるクエリが来る(サンプル4個目 (5,5))。添字に注意
// - 0/1-indexed 変換は入口で一度だけ
// 制約: 2 ≤ N ≤ 2×10^5, 1 ≤ P_i < i, 1 ≤ Q ≤ 2×10^5,
//       1 ≤ U_i ≤ N, 0 ≤ D_i ≤ N−1
//
// resolve: parents[i] = 頂点 i+2 の親 P_{i+2}(1-indexed、長さ N−1)。
// queries[i] = (U_i, D_i)(U は 1-indexed、D は辺数=深さ)。
// 返り値 = 各クエリの答え
fn resolve(n: usize, parents: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let parents: Vec<usize> = parents.iter().map(|&x| x - 1).collect();
    let queries: Vec<(usize, usize)> = queries.iter().map(|&(v, d)| (v - 1, d)).collect();
    let g = build_graph(n, &parents);
    let mut idx = 0usize;
    let mut ins = vec![usize::MAX; n];
    let mut order = vec![usize::MAX; n];
    let mut depth = vec![0; n];

    let mut stack = vec![0];

    while let Some(v) = stack.pop() {
        // popした順番
        order[idx] = v;
        // 行きがけの番号
        ins[v] = idx;
        idx += 1;
        for &to in g[v].iter() {
            stack.push(to);
            depth[to] = depth[v] + 1;
        }
    }
    let mut sizes = vec![1; n];
    // i 番目に pop された頂点を取り出して、親に size を納品
    for i in (1..n).rev() {
        let v = order[i];
        sizes[parents[v - 1]] += sizes[v];
    }

    let num_depths = *depth.iter().max().unwrap() + 1;
    let mut by_depth: Vec<Vec<usize>> = vec![Vec::new(); num_depths];
    for &v in order.iter() {
        by_depth[depth[v]].push(ins[v]);
    }

    let mut ans = Vec::new();
    for (u, d) in queries {
        if d >= num_depths {
            ans.push(0);
            continue;
        }
        let lo = ins[u];
        let hi = lo + sizes[u];

        let count =
            by_depth[d].partition_point(|&x| x < hi) - by_depth[d].partition_point(|&x| x < lo);
        ans.push(count);
    }
    ans
}

type Graph = Vec<Vec<usize>>;
fn build_graph(n: usize, parents: &[usize]) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];

    for (to, &parent) in parents.iter().enumerate() {
        g[parent].push(to + 1);
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
        assert_eq!(
            resolve(7, &[1, 1, 2, 2, 4, 2], &[(1, 2), (7, 2), (4, 1), (5, 5)]),
            vec![3, 1, 0, 0]
        );
    }

    #[test]
    fn handmade() {
        // 鎖 1-2-3-4。(2,3)→{4}、(2,1)→{2}(U自身が数えられる)、
        // (2,0)→なし(D < 深さ(U))、(3,3)→{4}、(1,0)→{1}(根自身)
        assert_eq!(
            resolve(4, &[1, 2, 3], &[(2, 3), (2, 1), (2, 0), (3, 3), (1, 0)]),
            vec![1, 1, 0, 1, 1]
        );
        // スター(中心1)。(1,1)→葉全部、(3,1)→{3}、(3,0)→なし、(1,0)→{1}
        assert_eq!(
            resolve(5, &[1, 1, 1, 1], &[(1, 1), (3, 1), (3, 0), (1, 0)]),
            vec![4, 1, 0, 1]
        );
    }

    // ---- naive: クエリごとに全頂点を見て、根まで登って判定(定義そのまま) ----
    fn naive(n: usize, parents: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
        // par[v], depth[v]: P_i < i なので番号順の1パスで確定する
        let mut par = vec![0; n + 1];
        let mut depth = vec![0; n + 1];
        for v in 2..=n {
            par[v] = parents[v - 2];
            depth[v] = depth[par[v]] + 1;
        }
        queries
            .iter()
            .map(|&(u, d)| {
                (1..=n)
                    .filter(|&v| depth[v] == d)
                    .filter(|&v| {
                        let mut w = v;
                        loop {
                            if w == u {
                                return true;
                            }
                            if w == 1 {
                                return false;
                            }
                            w = par[w];
                        }
                    })
                    .count()
            })
            .collect()
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261117u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..300 {
            let n = 2 + next(20);
            // P_v は 1..v から選ぶ(制約 P_i < i の形そのまま)
            let parents: Vec<usize> = (2..=n).map(|v| 1 + next((v - 1) as u64)).collect();
            // D は最大深さ超えも混ぜる
            let queries: Vec<(usize, usize)> = (0..8)
                .map(|_| (1 + next(n as u64), next((n + 2) as u64)))
                .collect();
            assert_eq!(
                resolve(n, &parents, &queries),
                naive(n, &parents, &queries),
                "n={n} parents={parents:?} queries={queries:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 深さ最悪: 鎖 2×10^5。クエリ (v, n-1) は「最深頂点が v の部分木にいるか」
        // → 全部 1。再帰で書くとここで死ぬ(EDPC Pで実体験済み)
        let n = 200_000;
        let parents: Vec<usize> = (1..n).collect();
        let queries: Vec<(usize, usize)> = (1..=n).map(|v| (v, n - 1)).collect();
        assert_eq!(resolve(n, &parents, &queries), vec![1; n]);
    }

    #[test]
    fn wide_star() {
        // 幅最悪: スター 2×10^5。深さ1のリストが巨大になり、二分探索の出番。
        // (1,1) → n-1、(葉,1) → 自分だけ
        let n = 200_000;
        let parents = vec![1; n - 1];
        let queries: Vec<(usize, usize)> = (1..=n).map(|v| (v, 1)).collect();
        let mut expected = vec![1; n];
        expected[0] = n - 1;
        assert_eq!(resolve(n, &parents, &queries), expected);
    }
}
