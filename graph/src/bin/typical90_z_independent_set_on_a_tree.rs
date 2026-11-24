// https://atcoder.jp/contests/typical90/tasks/typical90_z

// N頂点の木(Nは偶数)。どの2頂点も辺で直接つながらないように、
// 相異なる N/2 個の頂点を選んで出力せよ。
//
// Step 5 木の9問目(★4だが軽め): 構成問題(複数解可 → 検証器方式のテスト、
// ABC126 D と同じ型)。考えどころ:
// - 「どの2つも非隣接」な頂点集合 = 独立集合(EDPC P で最大独立集合を数えた)。
//   でも今回は最大は要らない — N/2 個「存在する保証」はどこから来る?
// - 木の辺は必ず深さを1変える(ABC126 D でやった)。すると深さの偶奇で
//   頂点を2つの組に分けたとき、同じ組の2頂点が辺で結ばれることはあるか?
// - 2つの組の人数を足すと N。片方が N/2 未満だったら、もう片方は?(鳩の巣)
// - 出力は「ちょうど N/2 個」。多い方の組をそのまま全部出すとどうなる?
// - 実装: 深さ埋め(いつもの距離埋めの重みなし版)→ 偶奇で振り分け → 多い方から N/2 個
// 制約: 2 ≤ N ≤ 10^5, 1 ≤ A_i < B_i ≤ N, N は偶数, 与えられるデータは木
//
// resolve: edges[i] = (a, b) は無向辺(1-indexed, a < b)。
// 返り値 = 選んだ N/2 個の頂点番号(1-indexed、順不同、複数解可)
fn resolve(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let g = build_graph(n, edges);
    let mut depth = vec![usize::MAX; n];
    let mut stack = vec![0];
    let mut even = Vec::new();
    let mut odd = Vec::new();
    even.push(0);
    depth[0] = 0;

    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if depth[to] == usize::MAX {
                stack.push(to);
                depth[to] = depth[v] + 1;
                if depth[to] % 2 == 0 {
                    even.push(to);
                } else {
                    odd.push(to);
                }
            }
        }
    }

    (if even.len() >= odd.len() {
        &even[0..(n / 2)]
    } else {
        &odd[0..(n / 2)]
    })
    .iter()
    .map(|&v| v + 1)
    .collect()
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
    use std::collections::HashSet;

    // ---- 検証器: 複数解可なので「答えが条件を満たすか」を検査する ----
    fn verify(n: usize, edges: &[(usize, usize)], ans: &[usize]) {
        assert_eq!(ans.len(), n / 2, "個数が N/2 ではない: {ans:?}");
        let chosen: HashSet<usize> = ans.iter().copied().collect();
        assert_eq!(chosen.len(), ans.len(), "重複あり: {ans:?}");
        assert!(
            ans.iter().all(|&v| 1 <= v && v <= n),
            "範囲外の頂点: {ans:?}"
        );
        for &(a, b) in edges {
            assert!(
                !(chosen.contains(&a) && chosen.contains(&b)),
                "隣接ペア ({a},{b}) を両方選んでいる: {ans:?}"
            );
        }
    }

    #[test]
    fn sample_01() {
        let edges = [(1, 2), (2, 3), (2, 4)];
        verify(4, &edges, &resolve(4, &edges));
    }

    #[test]
    fn sample_02() {
        // 模範出力は "1 2 6" だが複数解可(例えば {1,4,5} も正解)
        let edges = [(1, 3), (2, 4), (3, 5), (2, 5), (3, 6)];
        verify(6, &edges, &resolve(6, &edges));
    }

    #[test]
    fn handmade() {
        // 最小ケース n=2: 辺1本、どちらか片方だけを1個選ぶ
        verify(2, &[(1, 2)], &resolve(2, &[(1, 2)]));
        // 偏った木: スター4頂点。中心側は1個しかない → 葉側から2個選ぶしかない
        verify(
            4,
            &[(1, 2), (1, 3), (1, 4)],
            &resolve(4, &[(1, 2), (1, 3), (1, 4)]),
        );
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261124u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..300 {
            let n = 2 * (1 + next(10)); // 偶数 2..=20
            let mut label: Vec<usize> = (0..=n).collect();
            for v in (3..=n).rev() {
                let r = 2 + next((v - 1) as u64);
                label.swap(v, r);
            }
            let edges: Vec<(usize, usize)> = (2..=n)
                .map(|v| {
                    let p = 1 + next((v - 1) as u64);
                    let (a, b) = (label[p], label[v]);
                    // 入力形式 A_i < B_i に合わせる
                    (a.min(b), a.max(b))
                })
                .collect();
            verify(n, &edges, &resolve(n, &edges));
        }
    }

    #[test]
    fn deep_path() {
        // 鎖 10^5: 偶奇がちょうど半々 → どちらの組も全部出せばぴったり N/2
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        verify(n, &edges, &resolve(n, &edges));
    }

    #[test]
    fn wide_star() {
        // スター 10^5: 根側は1個だけ → 葉側(99999個)から N/2 個選ぶ。
        // 「多い方を全部出す」実装はここで落ちる
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        verify(n, &edges, &resolve(n, &edges));
    }
}
