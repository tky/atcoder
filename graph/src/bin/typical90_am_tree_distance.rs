// https://atcoder.jp/contests/typical90/tasks/typical90_am

// N頂点の木(辺の長さは全部1)。すべての頂点ペア (u < v) について
// dist(u, v) を足し合わせた総和を出力。
//
// Step 5 木の6問目: 部分木サイズ + 寄与の分解。考えどころ:
// - 愚直: 全頂点から距離埋めして総和 → O(N²) = 10^10 で間に合わない
// - 発想の転換: 「ペアごとに距離を足す」のをやめる。dist(u,v) は
//   u-v 経路上の辺の本数、つまり経路に含まれる辺1本につき 1 の寄せ集め。
//   だったら足す順番をひっくり返して、**辺ごとに「何組のペアに使われるか」**を
//   数えて足しても、総和は同じはず(なぜ同じと言い切れるか? は実装後に議論)
// - では辺 e に注目する。e を木から取り除くと何が起きる? e を経路に含むのは
//   どんなペア? その組数は、何が分かれば計算できる?
// - その「何か」を全辺ぶん一気に求める道具は、もう持っている: 0 を根にして
//   EDPC P と同じ2パス(行きがけ順 + parent 記録 → 逆読みで子から親へ納品)。
//   今回の納品物は dp 2本ではなく、もっと単純な1個の数。葉のときの値は?
//   親は子から受け取った値をどうする?
// - 答えの大きさに注意: 最大で 10^14 級(鎖のとき)。usize(64bit)なら
//   余裕だが、32bit型を混ぜると静かに壊れる規模
// 制約: 2 ≤ N ≤ 10^5, 1 ≤ a_i, b_i ≤ N, 与えられるグラフは木
//
// resolve: edges[i] = (a, b) は頂点 a-b を結ぶ長さ1の無向辺(1-indexed)。
// 返り値 = 全ペアの距離の総和
fn resolve(n: usize, edges: &[(usize, usize)]) -> usize {
    let g = build_graph(n, edges);
    let mut parent = vec![usize::MAX; n];
    // 0をrootにして親の頂点を求める
    parent[0] = 0;
    let mut stack = vec![0];
    let mut order = vec![0];

    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if parent[to] == usize::MAX {
                parent[to] = v;
                stack.push(to);
                order.push(to);
            }
        }
    }
    // 親からpushしている。子から処理したいのでreverse
    order.reverse();

    let mut size = vec![1; n];
    for v in order {
        if v == 0 {
            continue;
        }
        size[parent[v]] += size[v];
    }

    let mut ans = 0;
    for v in 1..n {
        ans += size[v] * (n - size[v]);
    }
    ans
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
        assert_eq!(resolve(2, &[(1, 2)]), 1);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(4, &[(1, 2), (1, 3), (1, 4)]), 9);
    }

    #[test]
    fn sample_03() {
        let edges = [
            (1, 2),
            (3, 1),
            (4, 2),
            (2, 5),
            (3, 6),
            (3, 7),
            (8, 4),
            (4, 9),
            (10, 5),
            (11, 7),
            (7, 12),
        ];
        assert_eq!(resolve(12, &edges), 211);
    }

    #[test]
    fn handmade() {
        // 鎖4頂点: 1+2+3 + 1+2 + 1 = 10
        assert_eq!(resolve(4, &[(1, 2), (2, 3), (3, 4)]), 10);
        // 鎖5頂点: 4·1 + 3·2 + 2·3 + 1·4 = 20
        assert_eq!(resolve(5, &[(1, 2), (2, 3), (3, 4), (4, 5)]), 20);
    }

    // ---- naive: 全頂点から距離を埋めて総和(定義そのまま、O(N²)) ----
    fn naive(n: usize, edges: &[(usize, usize)]) -> usize {
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in edges {
            adj[a - 1].push(b - 1);
            adj[b - 1].push(a - 1);
        }
        let mut total = 0;
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
            total += dist.iter().sum::<usize>();
        }
        total / 2 // 各ペアを両側から2回数えている
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
                    (label[p], label[v])
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
        // 深さ最悪: 鎖 10^5。Σ d·(n−d) = 166_666_666_650_000(Pythonで事前計算)。
        // 32bit型を混ぜた実装はここで静かに壊れる
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &edges), 166_666_666_650_000);
    }

    #[test]
    fn wide_star() {
        // 幅最悪: スター 10^5。中心-葉 (N−1)·1 + 葉-葉 C(N−1,2)·2(Pythonで事前計算)
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        assert_eq!(resolve(n, &edges), 9_999_800_001);
    }
}
