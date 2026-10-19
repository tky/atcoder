// https://atcoder.jp/contests/abc007/tasks/abc007_3 (古いABCなのでタスクIDは abc007_3)

// R行C列の迷路('.'=空きマス, '#'=壁)。スタートからゴールへの最小移動手数(上下左右)。
// 問題文自体がBFSの解説になっている教材問題。保証: 盤面の外周はすべて壁、
// スタート/ゴールは空きマス、ゴールには必ず到達できる。
//
// 学びどころ: 「グリッドをグラフとみなす」
// - 頂点 = マス (y, x)、辺 = 上下左右の隣接。build_graphは不要で、
//   隣接リストの代わりに「その場で4方向を計算する」(方向配列 dy/dx)
// - visited と origin の代わりに、距離配列 dist を1本持つのが定番:
//   INF(番兵)で初期化し、「dist[ny][nx] == INF」が「未訪問」を兼ねる。
//   積むときに dist[ny][nx] = dist[y][x] + 1 を書く(=積むときに印、の距離版)
// - y-1 の usize アンダーフローが気になるところだが、制約を読むと…
//   外周が全部壁 → 空きマスは必ず内側 → 空きマスからの4近傍は常に盤面内。
//   「どの保証が境界チェックを不要にするか」を考えるのもこの問題の価値
//
// start, goal は問題の入力どおり1-indexedの (y, x) で渡す(内部で0-indexedへ)
fn resolve(start: (usize, usize), goal: (usize, usize), grid: &[&str]) -> usize {
    let row = grid.len();
    let col = grid[0].len();
    let mut visited = vec![vec![usize::MAX; col]; row];

    let start = (start.0 - 1, start.1 - 1);
    let goal = (goal.0 - 1, goal.1 - 1);

    visited[start.0][start.1] = 0;

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        let next_step = visited[x][y] + 1;

        for (next_x, next_y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if grid[next_x].as_bytes()[next_y] == b'.' {
                if visited[next_x][next_y] == usize::MAX {
                    visited[next_x][next_y] = next_step;
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }
    visited[goal.0][goal.1]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let grid = [
            "########", "#......#", "#.######", "#..#...#", "#..##..#", "##.....#", "########",
        ];
        assert_eq!(resolve((2, 2), (4, 5), &grid), 11);
    }

    #[test]
    fn sample_02() {
        let grid = ["########", "#.#....#", "#.###..#", "#......#", "########"];
        assert_eq!(resolve((2, 2), (2, 4), &grid), 10);
    }

    #[test]
    fn sample_03() {
        // 50x50、内側に壁のない広間。最短 = (49-2) + (49-2) = 94
        let top = "#".repeat(50);
        let mid = format!("#{}#", ".".repeat(48));
        let mut rows = vec![top.clone()];
        rows.extend(std::iter::repeat_n(mid, 48));
        rows.push(top);
        let grid: Vec<&str> = rows.iter().map(|s| s.as_str()).collect();
        assert_eq!(resolve((2, 2), (49, 49), &grid), 94);
    }

    #[test]
    fn handmade() {
        // 最小: 隣どうし
        assert_eq!(resolve((2, 2), (2, 3), &["####", "#..#", "####"]), 1);
        // 壁を回り込む: 右隣は壁、下を迂回して4手
        let grid = [
            "#####", //
            "#.#.#", //
            "#...#", //
            "#####",
        ];
        assert_eq!(resolve((2, 2), (2, 4), &grid), 4);
    }

    // ---- ランダム迷路で naive(辺の緩和)と突き合わせ ----
    // naive: 「隣どうしの dist の差は高々1」が成り立つまで全マスを緩和し続ける。
    // BFSとは別原理なので答え合わせになる
    fn naive_dist(grid: &[String], start: (usize, usize), goal: (usize, usize)) -> usize {
        let r = grid.len();
        let c = grid[0].len();
        let inf = usize::MAX / 2;
        let g: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
        let mut dist = vec![vec![inf; c]; r];
        dist[start.0 - 1][start.1 - 1] = 0;
        loop {
            let mut changed = false;
            for y in 1..r - 1 {
                for x in 1..c - 1 {
                    if g[y][x] != b'.' {
                        continue;
                    }
                    for (ny, nx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                        if g[ny][nx] == b'.' && dist[ny][nx] + 1 < dist[y][x] {
                            dist[y][x] = dist[ny][nx] + 1;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        dist[goal.0 - 1][goal.1 - 1]
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261019u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        let inf = usize::MAX / 2;
        let mut tested = 0;
        for _ in 0..200 {
            let r = 3 + next(48); // 3..=50
            let c = 3 + next(48);
            // 外周は壁、内側は30%の確率で壁
            let grid: Vec<String> = (0..r)
                .map(|y| {
                    (0..c)
                        .map(|x| {
                            if y == 0 || y == r - 1 || x == 0 || x == c - 1 || next(10) < 3 {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .collect()
                })
                .collect();
            // 空きマスからstart/goalを選ぶ(2つ無ければ作り直し)
            let open: Vec<(usize, usize)> = (1..r - 1)
                .flat_map(|y| (1..c - 1).map(move |x| (y, x)))
                .filter(|&(y, x)| grid[y].as_bytes()[x] == b'.')
                .collect();
            if open.len() < 2 {
                continue;
            }
            let s = open[next(open.len() as u64)];
            let g = open[next(open.len() as u64)];
            if s == g {
                continue;
            }
            let start = (s.0 + 1, s.1 + 1);
            let goal = (g.0 + 1, g.1 + 1);
            let expected = naive_dist(&grid, start, goal);
            if expected >= inf {
                continue; // 到達不能な盤面は問題の制約外なのでスキップ
            }
            let grid_refs: Vec<&str> = grid.iter().map(|s| s.as_str()).collect();
            assert_eq!(
                resolve(start, goal, &grid_refs),
                expected,
                "start={start:?} goal={goal:?}\n{}",
                grid.join("\n")
            );
            tested += 1;
        }
        assert!(tested >= 50, "有効ケースが少なすぎる: {tested}");
    }
}
