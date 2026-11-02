// https://atcoder.jp/contests/abc362/tasks/abc362_d

use std::cmp::Reverse;

// N頂点M辺の単純連結無向グラフ。頂点iには重みA_i、辺jには重みB_jがある。
// パスの重み = パス上に現れる**頂点と辺の両方**の重みの合計。
// 各 i=2..N について、頂点1から頂点iへの最小重みパスを求める(答えをその順に返す)。
//
// ここから Step 4: Dijkstra。考えどころ:
// - BFSの正しさは「キューから距離の昇順に出てくる」不変条件に立っていたが、
//   あの証明は**全ての辺のコストが1**であることに依存していた。重みがバラバラだと
//   FIFOでは近い順に出てこない(反例を1つ作ってみると腹落ちする)
// - Dijkstra = キューを**優先度付きキュー**(常に暫定距離最小の頂点を取り出す)に
//   差し替えたBFS。これで「取り出した時点の距離は確定」が復活する。
//   ただし復活には条件がある: 辺の重みが**非負**であること(なぜ?)。制約でB≥0,A≥0 ✓
// - Rustの std::collections::BinaryHeap は**最大**ヒープ。最小で使うには
//   std::cmp::Reverse で包む: heap.push(Reverse((dist, v))) / Some(Reverse((d, u))) = heap.pop()
// - BinaryHeapには「すでに積んだ要素の優先度を下げる」操作がない。定石は**遅延削除**:
//   より良い距離を見つけたら気にせず重複push、popしたとき dist[u] より悪い値なら
//   「古い残骸」なので捨てて次へ(if d > dist[u] { continue; })。
//   「積むときに印」規約のDijkstra版がこの形になる
// - 頂点重みのひねりは、緩和の式に畳み込める: 1→…→u→v と進むとき
//   候補 = dist[u] + B(u,v) + A_v。始点の初期値はどうなる?
// - 答えは32bitに収まらない(問題文明記): 最悪 ~4*10^14。usize(64bit)でOK。
//   distの番兵usize::MAXに足し算しないよう、緩和は「popした確定値」からだけ行う
// 制約: 2 ≤ N ≤ 2*10^5, N-1 ≤ M ≤ 2*10^5, 0 ≤ A,B ≤ 10^9, 単純・連結、U<V
//
// resolve: a[i] = 頂点i+1の重み(0-indexed)、edges = (u, v, b) 1-indexed。
// 返り値 = i=2..N の答え(長さ N-1)
fn resolve(a: &[usize], edges: &[(usize, usize, usize)]) -> Vec<usize> {
    let mut heap = std::collections::BinaryHeap::new();
    // (距離,頂点)
    heap.push(Reverse((a[0], 0)));

    let graph = build_graph(a.len(), edges);

    let mut results = vec![usize::MAX; a.len()];
    results[0] = a[0];

    while let Some(Reverse((dist, v))) = heap.pop() {
        if dist > results[v] {
            continue;
        }
        for &(y, b) in graph[v].iter() {
            if dist + b + a[y] < results[y] {
                results[y] = dist + b + a[y];
                heap.push(Reverse((dist + b + a[y], y)));
            }
        }
    }

    results[1..].to_vec()
}

type Graph = Vec<Vec<(usize, usize)>>;

type Edge = (usize, usize, usize);

type Edges = [Edge];

fn build_graph(n: usize, edges: &Edges) -> Graph {
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

    #[test]
    fn sample_01() {
        let a = [1, 2, 3];
        let edges = [(1, 2, 1), (1, 3, 6), (2, 3, 2)];
        assert_eq!(resolve(&a, &edges), vec![4, 9]);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[0, 1], &[(1, 2, 3)]), vec![4]);
    }

    #[test]
    fn sample_03() {
        let a = [928448202, 994752369, 906965437, 942744902, 907560126];
        let edges = [
            (2, 5, 975090662),
            (1, 2, 908843627),
            (1, 5, 969061140),
            (3, 4, 964249326),
            (2, 3, 957690728),
            (2, 4, 942986477),
            (4, 5, 948404113),
            (1, 3, 988716403),
        ];
        assert_eq!(
            resolve(&a, &edges),
            vec![2832044198, 2824130042, 4696218483, 2805069468]
        );
    }

    #[test]
    fn handmade() {
        // 重み全ゼロ
        assert_eq!(resolve(&[0, 0], &[(1, 2, 0)]), vec![0]);
        // 一直線 1-2-3: 2経由でしか3に行けない
        assert_eq!(
            resolve(&[1, 10, 100], &[(1, 2, 2), (2, 3, 3)]),
            vec![13, 116]
        );
        // 頂点重みの罠: 辺だけ見ると直行(B=1)が最短だが、経由頂点2が軽い
        //   1→3 直行: 1 + 1 + 100 = 102
        //   1→2→3:   1 + 1 + 1 + 1 + 100 = 104 → 直行の勝ち
        //   ただしA_2が重い場合(=1000)に 1→2→3 を選んだら頂点重み忘れのバグ
        assert_eq!(
            resolve(&[1, 1000, 100], &[(1, 3, 1), (1, 2, 0), (2, 3, 0)]),
            vec![1001, 102]
        );
    }

    // ---- naive: 辺の緩和を変化がなくなるまで繰り返す(Bellman-Ford風、別原理) ----
    fn naive(a: &[usize], edges: &[(usize, usize, usize)]) -> Vec<usize> {
        let n = a.len();
        let inf = usize::MAX / 2;
        let mut dist = vec![inf; n];
        dist[0] = a[0];
        loop {
            let mut changed = false;
            for &(u, v, b) in edges {
                let (u, v) = (u - 1, v - 1);
                if dist[u] + b + a[v] < dist[v] {
                    dist[v] = dist[u] + b + a[v];
                    changed = true;
                }
                if dist[v] + b + a[u] < dist[u] {
                    dist[u] = dist[v] + b + a[u];
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        dist[1..].to_vec()
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261102u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..150 {
            let n = 2 + next(25);
            let a: Vec<usize> = (0..n).map(|_| next(20)).collect();
            // 木で連結を保証し、余分な辺を足す(単純グラフの制約: 全ペア相異)
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
                set.into_iter().map(|(u, v)| (u, v, next(20))).collect();
            assert_eq!(
                resolve(&a, &edges),
                naive(&a, &edges),
                "a={a:?} edges={edges:?}"
            );
        }
    }

    #[test]
    fn large_chain() {
        // 制約最大: 一直線、A_i=1, B=1 → dist[i] = 頂点i個 + 辺i-1本 = 2i-1
        let n = 200_000;
        let a = vec![1; n];
        let edges: Vec<(usize, usize, usize)> = (1..n).map(|i| (i, i + 1, 1)).collect();
        let expected: Vec<usize> = (2..=n).map(|i| 2 * i - 1).collect();
        assert_eq!(resolve(&a, &edges), expected);
    }

    #[test]
    fn large_weights() {
        // 重み最大値での溢れ確認: 一直線、A=B=10^9。
        // dist[N] = N*10^9 + (N-1)*10^9 ≈ 4*10^14 (u32なら即死、u64で余裕)
        let n = 200_000;
        let w = 1_000_000_000;
        let a = vec![w; n];
        let edges: Vec<(usize, usize, usize)> = (1..n).map(|i| (i, i + 1, w)).collect();
        let expected: Vec<usize> = (2..=n).map(|i| (2 * i - 1) * w).collect();
        assert_eq!(resolve(&a, &edges), expected);
    }
}
