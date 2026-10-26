// https://atcoder.jp/contests/agc033/tasks/agc033_a

// H行W列の白黒盤面。毎分、黒マスに隣接(上下左右)する白マスが全部黒くなる。
// 全マスが黒くなるまでの操作回数を求める。黒マスは最初に1個以上ある保証。
//
// 複数始点BFS: 「各マスの答え = 最寄りの黒マスまでの距離」で、
// 操作回数 = その最大値。全黒マスを**始点として同時に**BFSすると一度に求まる。
//
// ABC007 C(迷路)との差分は初期化だけ:
// - 迷路: start 1マスに dist 0 を書いて積む
// - 今回: 黒マス全部に dist 0 を書いて積む(「最初のpushにも印」が複数になるだけ)
//   ループ本体は一切変わらない。なぜこれで「最寄りの黒マスまでの距離」になるのか、
//   キューの不変条件(距離の昇順に出てくる)で説明できるか考えてみてほしい
//
// 注意: 今回は「外周が壁」の保証が**ない**(前問と逆)。境界チェックが必要。
//   方法は前に話した3択 — 範囲チェック / 番兵壁を足す / wrapping_add
// 制約: H, W ≤ 1000 → 最大10^6マス。O(HW)のBFSなら余裕
fn resolve(grid: &[&str]) -> usize {
    let row = grid.len();
    let col = grid[0].len();

    let mut dist = vec![vec![usize::MAX; col]; row];

    let mut queue = std::collections::VecDeque::new();

    for i in 0..row {
        for j in 0..col {
            if grid[i].as_bytes()[j] == b'#' {
                queue.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let current = dist[x][y];
        for (next_x, next_y) in neighbors(x, y, row, col) {
            if dist[next_x][next_y] == usize::MAX {
                dist[next_x][next_y] = current + 1;
                queue.push_back((next_x, next_y));
            }
        }
    }
    *dist.iter().flatten().max().unwrap()
}

fn neighbors(x: usize, y: usize, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
        let nx = x.checked_add_signed(dx)?;
        let ny = y.checked_add_signed(dy)?;
        (nx < row && ny < col).then_some((nx, ny))
    })
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&["...", ".#.", "..."]), 2);
    }

    #[test]
    fn sample_02() {
        let grid = [
            "..#..#", //
            "......", //
            "#..#..", //
            "......", //
            ".#....", //
            "....#.",
        ];
        assert_eq!(resolve(&grid), 3);
    }

    #[test]
    fn handmade() {
        assert_eq!(resolve(&["#"]), 0); // 1マスで既に全黒
        assert_eq!(resolve(&["##", "##"]), 0); // 全黒なら操作0回
        assert_eq!(resolve(&["#...", "....", "...."]), 5); // 角から対角まで (3-1)+(4-1)
        assert_eq!(resolve(&["#...#"]), 2); // 両端が黒: 真ん中が最後
    }

    // ---- ランダム盤面で naive(緩和法)と突き合わせ ----
    fn naive(grid: &[String]) -> usize {
        let h = grid.len();
        let w = grid[0].len();
        let inf = usize::MAX / 2;
        let g: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
        let mut dist = vec![vec![inf; w]; h];
        for y in 0..h {
            for x in 0..w {
                if g[y][x] == b'#' {
                    dist[y][x] = 0;
                }
            }
        }
        loop {
            let mut changed = false;
            for y in 0..h {
                for x in 0..w {
                    let mut best = dist[y][x];
                    if y > 0 {
                        best = best.min(dist[y - 1][x] + 1);
                    }
                    if y + 1 < h {
                        best = best.min(dist[y + 1][x] + 1);
                    }
                    if x > 0 {
                        best = best.min(dist[y][x - 1] + 1);
                    }
                    if x + 1 < w {
                        best = best.min(dist[y][x + 1] + 1);
                    }
                    if best < dist[y][x] {
                        dist[y][x] = best;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        dist.iter().flatten().copied().max().unwrap()
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261026u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..100 {
            let h = 1 + next(30);
            let w = 1 + next(30);
            // 黒の密度をケースごとに変える(濃い盤面も薄い盤面も試す)
            let density = 1 + next(9); // 10%〜90%
            let mut grid: Vec<String> = (0..h)
                .map(|_| {
                    (0..w)
                        .map(|_| if next(10) < density { '#' } else { '.' })
                        .collect()
                })
                .collect();
            // 黒1個以上の保証を満たす(なければ1個置く)
            if !grid.iter().any(|row| row.contains('#')) {
                let y = next(h as u64);
                let x = next(w as u64);
                grid[y].replace_range(x..x + 1, "#");
            }
            let grid_refs: Vec<&str> = grid.iter().map(|s| s.as_str()).collect();
            assert_eq!(
                resolve(&grid_refs),
                naive(&grid),
                "盤面:\n{}",
                grid.join("\n")
            );
        }
    }

    #[test]
    fn large_corner() {
        // 制約最大 1000x1000、左上の角だけ黒 → (1000-1)+(1000-1) = 1998
        let h = 1000;
        let first = format!("#{}", ".".repeat(h - 1));
        let rest = ".".repeat(h);
        let mut rows = vec![first];
        rows.extend(std::iter::repeat_n(rest, h - 1));
        let grid: Vec<&str> = rows.iter().map(|s| s.as_str()).collect();
        assert_eq!(resolve(&grid), 1998);
    }
}
