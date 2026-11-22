// https://atcoder.jp/contests/abc067/tasks/arc078_b

// 木の陣取りゲーム。マス1は黒(フェネック)、マスNは白(すぬけ)。
// フェネック(先手)は黒に隣接する未着色マスを黒く、すぬけ(後手)は白に
// 隣接する未着色マスを白く塗る。交互に1マスずつ。自分の手番で塗れる
// マスがなければ負け。両者最適で勝者は?
//
// Step 5 木の8問目(軽め): ゲームの距離への言い換え。考えどころ:
// - ゲーム木の全探索(minimax)は状態数が爆発して不可能。でも「両者最適」を
//   マスごとの運命に言い換えられる: あるマス v を黒が取れる最速は何手目?
//   黒の塊は1から広がる連結領域なので、答えは dist(1, v)。白なら dist(N, v)。
//   **先に着ける方がそのマスを取る**。同着はどちらのものになる?(先手後手)
// - 境界が確定したあとは、お互い自分の領地を塗りつぶすだけの消化試合。
//   すると勝敗は**領地のマス数の比較**だけで決まる(なぜ数だけでいいのか、
//   どちらが先に塗れなくなるのか、は実装後に議論)
// - 実装は いつもの距離埋めを2回(1から、Nから)→ 数えて比較。O(N)。
//   直径の2回走査と同じ道具立て
// - 等号の向きの罠: handmade の鎖3頂点(真ん中が同距離)が検出する
// 制約: 2 ≤ N ≤ 10^5, 与えられるグラフは木
//
// resolve: edges[i] = (a, b) は無向辺(1-indexed)。
// 返り値 = "Fennec" か "Snuke"
fn resolve(n: usize, edges: &[(usize, usize)]) -> String {
    let g = build_graph(n, edges);
    let df = distances_from(n, 0, &g);
    let ds = distances_from(n, n - 1, &g);

    let mut fennec_count = 0;
    let mut snuke_count = 0;

    for i in 0..n {
        if df[i] <= ds[i] {
            fennec_count += 1;
        } else {
            snuke_count += 1;
        }
    }
    if fennec_count > snuke_count {
        "Fennec".to_string()
    } else {
        "Snuke".to_string()
    }
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

fn distances_from(n: usize, start: usize, g: &Graph) -> Vec<usize> {
    let mut stack = vec![start];
    let mut distances = vec![usize::MAX; n];
    distances[start] = 0;

    while let Some(v) = stack.pop() {
        for &to in g[v].iter() {
            if distances[to] == usize::MAX {
                stack.push(to);
                distances[to] = distances[v] + 1;
            }
        }
    }

    distances
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn sample_01() {
        let edges = [(3, 6), (1, 2), (3, 1), (7, 4), (5, 7), (1, 4)];
        assert_eq!(resolve(7, &edges), "Fennec");
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(4, &[(1, 4), (4, 2), (2, 3)]), "Snuke");
    }

    #[test]
    fn handmade() {
        // n=2: フェネックは一手も打てずに負け
        assert_eq!(resolve(2, &[(1, 2)]), "Snuke");
        // 鎖3頂点: 真ん中のマス2は両者から同距離。先手が取るので2対1でFennec
        assert_eq!(resolve(3, &[(1, 2), (2, 3)]), "Fennec");
    }

    // ---- naive: ゲーム木のminimax(定義そのまま、両者最適)。小さいn専用 ----
    // colors[v]: 0=未着色, 1=黒, 2=白。手番側が塗れなければ負け
    fn win(
        colors: &mut Vec<u8>,
        turn: u8,
        adj: &Vec<Vec<usize>>,
        memo: &mut HashMap<(Vec<u8>, u8), bool>,
    ) -> bool {
        if let Some(&w) = memo.get(&(colors.clone(), turn)) {
            return w;
        }
        let n = colors.len() - 1;
        let moves: Vec<usize> = (1..=n)
            .filter(|&v| colors[v] == 0 && adj[v].iter().any(|&to| colors[to] == turn))
            .collect();
        let mut result = false; // 打つ手なし → 負け
        for v in moves {
            colors[v] = turn;
            let opponent_wins = win(colors, 3 - turn, adj, memo);
            colors[v] = 0;
            if !opponent_wins {
                result = true; // 相手を負けにできる手がある
                break;
            }
        }
        memo.insert((colors.clone(), turn), result);
        result
    }

    fn naive(n: usize, edges: &[(usize, usize)]) -> String {
        let mut adj = vec![Vec::new(); n + 1];
        for &(a, b) in edges {
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut colors = vec![0u8; n + 1];
        colors[1] = 1;
        colors[n] = 2;
        let mut memo = HashMap::new();
        if win(&mut colors, 1, &adj, &mut memo) {
            "Fennec".to_string()
        } else {
            "Snuke".to_string()
        }
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
        for _ in 0..200 {
            let n = 2 + next(7); // 2..=8(minimaxが現実的な範囲)
            let mut label: Vec<usize> = (0..=n).collect();
            for v in (3..=n).rev() {
                let r = 2 + next((v - 1) as u64);
                label.swap(v, r);
            }
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
            assert_eq!(
                resolve(n, &edges),
                naive(n, &edges),
                "n={n} edges={edges:?}"
            );
        }
    }

    #[test]
    fn deep_path() {
        // 鎖(偶数長): 50000対50000の同数 → 後手勝ち。
        // 鎖(奇数長): 真ん中の同着マスを先手が取って51対49側 → 先手勝ち
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &edges), "Snuke");
        let n = 99_999;
        let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &edges), "Fennec");
    }

    #[test]
    fn wide_star() {
        // スター: 中心もほぼ全部の葉もフェネックが先着
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (2..=n).map(|v| (1, v)).collect();
        assert_eq!(resolve(n, &edges), "Fennec");
    }
}
