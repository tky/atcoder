// https://atcoder.jp/contests/abc177/tasks/abc177_d

// N人の人間とM個の事実「AとBは友達」。友達の友達は友達(推移的)で、
// 与えられた事実から導けない友達関係はない。
// 「どのグループにも自分の友達がいない」ようにN人をグループ分けするとき、
// 必要なグループ数の最小値を答える。
//
// Union-Find 2問目。考えどころ:
// - 「友達関係(推移閉包込み)のかたまり」= グラフの連結成分。前問で作った size が
//   そのまま効く問題として選んだ
// - 本題はここ: なぜ「最小グループ数」が、Union-Findで管理しているある量の
//   **最大値**になるのか? 「同じ成分の2人は(推移閉包で)必ず友達」という事実から、
//   (a) それ未満では必ず失敗する、(b) その数あれば必ず分けられる、の両方を言えるか
// - 入力は1-indexed(1 ≤ A,B ≤ N)。変換は入口で一度だけ、の規約どおり
// - 同じ事実が複数回来る(多重辺)。A=Bはない。M=0もある(そのとき答えは?)
// 制約: 2 ≤ N ≤ 2*10^5, 0 ≤ M ≤ 2*10^5
fn resolve(n: usize, friendships: &[(usize, usize)]) -> usize {
    let mut uf = UnionFind::new(n);
    for &(x, y) in friendships {
        uf.union(x - 1, y - 1);
    }
    uf.max_group_size()
}

pub struct UnionFind {
    parent: Vec<isize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == -1 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx != ry {
            if self.size[rx] < self.size[ry] {
                self.parent[rx] = ry as isize;
                self.size[ry] += self.size[rx];
            } else {
                self.parent[ry] = rx as isize;
                self.size[rx] += self.size[ry];
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn max_group_size(&mut self) -> usize {
        let mut max = 0;

        for n in 0..self.parent.len() {
            if self.parent[n] == -1 {
                max = max.max(self.size[n]);
            }
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        // 成分 {1,2,5} {3,4} → 3
        assert_eq!(resolve(5, &[(1, 2), (3, 4), (5, 1)]), 3);
    }

    #[test]
    fn sample_02() {
        // 全員友達(多重辺あり) → 4
        let friendships = [
            (1, 2),
            (2, 1),
            (1, 2),
            (2, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (3, 4),
        ];
        assert_eq!(resolve(4, &friendships), 4);
    }

    #[test]
    fn sample_03() {
        // 成分 {1,3,4} {5,9} {2,6} {7} {8} {10} → 3
        assert_eq!(resolve(10, &[(3, 1), (4, 1), (5, 9), (2, 6)]), 3);
    }

    #[test]
    fn handmade() {
        // M=0: 誰も友達がいない → 全員同じグループでいい
        assert_eq!(resolve(2, &[]), 1);
        assert_eq!(resolve(5, &[]), 1);
        // 全員一列につながる → 全員同じ成分
        assert_eq!(resolve(4, &[(1, 2), (2, 3), (3, 4)]), 4);
        // 同サイズの成分が複数
        assert_eq!(resolve(6, &[(1, 2), (3, 4), (5, 6)]), 2);
        // 間接的な友達(1-2, 2-3: 1と3も友達)
        assert_eq!(resolve(3, &[(1, 2), (2, 3)]), 3);
    }

    // ---- naive: BFSで連結成分を列挙して最大サイズ(Union-Findとは別原理) ----
    fn naive(n: usize, friendships: &[(usize, usize)]) -> usize {
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in friendships {
            adj[a - 1].push(b - 1);
            adj[b - 1].push(a - 1);
        }
        let mut seen = vec![false; n];
        let mut best = 0;
        for s in 0..n {
            if seen[s] {
                continue;
            }
            seen[s] = true;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(s);
            let mut count = 0;
            while let Some(v) = queue.pop_front() {
                count += 1;
                for &to in &adj[v] {
                    if !seen[to] {
                        seen[to] = true;
                        queue.push_back(to);
                    }
                }
            }
            best = best.max(count);
        }
        best
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261029u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..200 {
            let n = 2 + next(40);
            let m = next(80);
            let friendships: Vec<(usize, usize)> = (0..m)
                .filter_map(|_| {
                    let a = 1 + next(n as u64);
                    let b = 1 + next(n as u64);
                    (a != b).then_some((a, b))
                })
                .collect();
            assert_eq!(
                resolve(n, &friendships),
                naive(n, &friendships),
                "n={n} friendships={friendships:?}"
            );
        }
    }

    #[test]
    fn large_chain() {
        // 制約最大: 全員が一列 → 答えはN。深い木を作る形でもあるので性能も見る
        let n = 200_000;
        let friendships: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        assert_eq!(resolve(n, &friendships), n);
    }

    #[test]
    fn large_pairs() {
        // 制約最大のもう一方の極: 2人組が10^5個 → 答えは2
        let n = 200_000;
        let friendships: Vec<(usize, usize)> = (0..n / 2).map(|i| (2 * i + 1, 2 * i + 2)).collect();
        assert_eq!(resolve(n, &friendships), 2);
    }
}
