// https://atcoder.jp/contests/abc126/tasks/abc126_d

// N頂点の木。i番目の辺は u_i と v_i を結び、長さは w_i。
// 各頂点を白(0)か黒(1)に塗り、「**同じ色**に塗られた任意の2頂点の距離が偶数」
// を満たす塗り分けを1つ出力する(必ず存在することが証明できる)。複数解OK。
//
// Step 5 木の2問目(構成問題)。考えどころ:
// - 複数解OK → テストは検証器方式(abc252_eでやったやつ)。出力の一致ではなく
//   「条件を満たしているか」を定義どおりに検査する
// - 本題: 木の2頂点 u, v 間の距離の**偶奇**は何で決まるか。
//   適当に根を1つ決めて、u→v の唯一の経路を「いったん登って、降りる」形と
//   見てみる。根に向かって登る道のうち、u側とv側で**共有している部分**は
//   u→vの移動では通らない — では dist(u,v) を、根からの距離 dist(u), dist(v) と
//   共有部分の長さで式にすると? その式を mod 2 で見ると何が消える?
// - それが見えれば塗り方は一行で決まる。前問 abc070_d の resolve がほぼそのまま
//   使える(要るのは距離そのものではなく、その…)
// - 全辺の重みが偶数の木では何が起きる? 実は入力例2がそれ(公式出力は
//   32通りある正解のうちの1つにすぎない)。検証器方式が効く理由
// - N=1(辺ゼロ、頂点1個だけ)もありうる
// - 距離の和は最大 ~10^14 → usizeなら溢れない。偶奇しか要らないので
//   途中で %2 してしまう手もある(どちらでも正しい)
// 制約: 1 ≤ N ≤ 10^5, 1 ≤ u_i < v_i ≤ N, 1 ≤ w_i ≤ 10^9, 入力は木
//
// resolve: edges[i] = (u, v, w) は u-v を長さwで結ぶ無向辺(頂点は1-indexed)。
// 返り値 = colors[v](0-indexed)が頂点v+1の色(0か1)
fn resolve(n: usize, edges: &[(usize, usize, usize)]) -> Vec<usize> {
    let g = build_graph(n, edges);
    let mut distances = vec![usize::MAX; n];

    distances[0] = 0;
    let mut stack = vec![0];

    while let Some(v) = stack.pop() {
        for &(to, w) in g[v].iter() {
            if distances[to] != usize::MAX {
                continue;
            }
            stack.push(to);
            distances[to] = distances[v] + w;
        }
    }

    distances.iter().map(|d| d % 2).collect()
}

// (頂点、重み）
type Graph = Vec<Vec<(usize, usize)>>;

fn build_graph(n: usize, edges: &[(usize, usize, usize)]) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to, w) in edges.iter() {
        g[from - 1].push((to - 1, w));
        g[to - 1].push((from - 1, w));
    }
    g
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- 検証器: 定義そのまま。各頂点から緩和フィックスポイントで全頂点への
    //      距離を計算し(別原理)、同色ペアの距離が全て偶数かを確かめる。
    //      O(N^2)以上かかるので小さい入力専用 ----
    fn is_valid(n: usize, edges: &[(usize, usize, usize)], colors: &[usize]) -> bool {
        if colors.len() != n || colors.iter().any(|&c| c > 1) {
            return false;
        }
        let inf = usize::MAX / 2;
        for s in 0..n {
            let mut dist = vec![inf; n];
            dist[s] = 0;
            loop {
                let mut changed = false;
                for &(u, v, w) in edges {
                    let (u, v) = (u - 1, v - 1);
                    if dist[u] + w < dist[v] {
                        dist[v] = dist[u] + w;
                        changed = true;
                    }
                    if dist[v] + w < dist[u] {
                        dist[u] = dist[v] + w;
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }
            for v in 0..n {
                if colors[s] == colors[v] && dist[v] % 2 != 0 {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn sample_01() {
        // 公式出力は 0 0 1 だが、条件を満たせば何でもよい
        let edges = [(1, 2, 2), (2, 3, 1)];
        assert!(is_valid(3, &edges, &resolve(3, &edges)));
    }

    #[test]
    fn sample_02() {
        // 全辺が偶数 → 実はどんな塗り方でも正解になる入力
        let edges = [(2, 5, 2), (2, 3, 10), (1, 3, 8), (3, 4, 2)];
        assert!(is_valid(5, &edges, &resolve(5, &edges)));
    }

    #[test]
    fn handmade() {
        // 奇数辺の鎖 1-2(1), 2-3(1): 有効な塗り方は {1,3}|{2} の2通りだけ
        let edges = [(1, 2, 1), (2, 3, 1)];
        assert!(is_valid(3, &edges, &resolve(3, &edges)));
        // 検証器自身の健全性: 間違った塗り方はちゃんと落ちる
        assert!(!is_valid(3, &edges, &[0, 0, 0]));
        assert!(!is_valid(3, &edges, &[0, 0, 1]));
        // N=1: 辺ゼロ。どちらの色でも有効(長さ1のVecが返ること)
        assert!(is_valid(1, &[], &resolve(1, &[])));
        // 奇数と偶数の混在スター
        let edges = [(1, 2, 1), (1, 3, 2), (1, 4, 3), (1, 5, 1_000_000_000)];
        assert!(is_valid(5, &edges, &resolve(5, &edges)));
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
            let n = 1 + next(15);
            // 木の生成: 頂点v(2..=n)を 1..v のどれかにつなぐ(u < v の制約にも合致)
            let edges: Vec<(usize, usize, usize)> = (2..=n)
                .map(|v| (1 + next((v - 1) as u64), v, 1 + next(100)))
                .collect();
            let colors = resolve(n, &edges);
            assert!(
                is_valid(n, &edges, &colors),
                "n={n} edges={edges:?} colors={colors:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 深さ最悪: 全辺が奇数(しかも巨大)の直線。距離の総和は ~10^14。
        // 鎖で全辺奇数なら「隣どうしは必ず別色」が有効性と同値 → O(N)で検査できる
        let n = 100_000;
        let w = 999_999_999;
        let edges: Vec<(usize, usize, usize)> = (1..n).map(|i| (i, i + 1, w)).collect();
        let colors = resolve(n, &edges);
        assert_eq!(colors.len(), n);
        assert!(colors.iter().all(|&c| c <= 1));
        for i in 0..n - 1 {
            assert_ne!(colors[i], colors[i + 1], "i={i}");
        }
    }

    #[test]
    fn wide_star() {
        // 幅最悪: 中心1に葉がぶら下がるスター、重みは葉番号(偶奇が混在)。
        // 根からの距離の偶奇: 中心=0、葉v=v%2。
        // 有効性 ⟺ 同色のグループ内で偶奇が揃っている → O(N)で検査できる
        let n = 100_000;
        let edges: Vec<(usize, usize, usize)> = (2..=n).map(|v| (1, v, v)).collect();
        let colors = resolve(n, &edges);
        assert_eq!(colors.len(), n);
        let parity = |v: usize| if v == 0 { 0 } else { (v + 1) % 2 }; // 0-indexed頂点vの偶奇
        for c in 0..=1 {
            let mut parities = (0..n).filter(|&v| colors[v] == c).map(parity);
            if let Some(first) = parities.next() {
                assert!(parities.all(|p| p == first), "color={c} 内で偶奇が混在");
            }
        }
    }
}
