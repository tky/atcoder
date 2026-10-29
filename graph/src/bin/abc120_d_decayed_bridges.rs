// https://atcoder.jp/contests/abc120/tasks/abc120_d

// N島M橋(最初は全島が行き来可能=連結)。橋は1番から順に崩落していく。
// 不便さ = 行き来できなくなった島のペア(a,b) (a<b) の個数。
// 各 i について「橋iが崩落した直後」の不便さを出力(M行)。
//
// Union-Find 応用の定番。考えどころ:
// - Union-Find は辺を**足す**のは得意だが、**消す**操作がない(経路圧縮で元の形を
//   捨てているので、1本抜いた後の形を復元できない)。崩落は削除の列。さてどうする?
//   ヒント: クエリを全部読んでから答えてよい(オンラインで答える必要がない)なら、
//   時間をどちら向きに流すかは自由
// - 2つの成分(サイズ s1, s2)が1本の辺でつながるとき、不便さはいくつ変化する?
//   前問の size 配列がまた効く
// - 辺が1本もない状態の不便さは? (これが片端の初期値になる)
// - すでに同じ成分の2島をつなぐ辺では不便さは変化しない(unionのrx==ryの分岐)
// - 答えは32bitに収まらないと問題文に明記あり。usizeは64bitなのでそのままでよいが、
//   N*(N-1)/2 ≈ 5*10^9 という規模感は意識しておく
// 制約: 2 ≤ N ≤ 10^5, 1 ≤ M ≤ 10^5, A<B, 全ペア相異(多重辺・自己ループなし)、初期は連結
//
// resolve は崩落順の橋のリストを受け取り、ans[i] = 橋i+1(1-indexedでi+1番目)崩落直後の
// 不便さ、を返す(長さM)
fn resolve(n: usize, bridges: &[(usize, usize)]) -> Vec<usize> {
    // 全ての頂点が孤立している場合
    let mut disconnected_pairs = n * (n - 1) / 2;

    let mut ans = Vec::new();

    let mut bridges: Vec<(usize, usize)> = bridges[1..]
        .to_vec()
        .iter()
        .map(|&(x, y)| (x - 1, y - 1))
        .collect();
    bridges.reverse();

    ans.push(disconnected_pairs);

    let mut uf = UnionFind::new(n);
    for &(x, y) in bridges.iter() {
        if !uf.same(x, y) {
            disconnected_pairs -= uf.group_count(x) * uf.group_count(y);
        }
        ans.push(disconnected_pairs);
        uf.union(x, y);
    }

    ans.reverse();

    ans
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

    pub fn group_count(&mut self, x: usize) -> usize {
        let rx = self.find(x);
        self.size[rx]
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
        let bridges = [(1, 2), (3, 4), (1, 3), (2, 3), (1, 4)];
        assert_eq!(resolve(4, &bridges), vec![0, 0, 4, 5, 6]);
    }

    #[test]
    fn sample_02() {
        let bridges = [(2, 3), (1, 2), (5, 6), (3, 4), (4, 5)];
        assert_eq!(resolve(6, &bridges), vec![8, 9, 12, 14, 15]);
    }

    #[test]
    fn sample_03() {
        assert_eq!(resolve(2, &[(1, 2)]), vec![1]);
    }

    #[test]
    fn handmade() {
        // スター: 1-2, 1-3。橋1崩落後は {1,3} と {2} → 2ペア不通。橋2崩落後は全バラ → 3
        assert_eq!(resolve(3, &[(1, 2), (1, 3)]), vec![2, 3]);
        // 三角形: どの1本を失っても連結のまま → 0, 2本目で2頂点が孤立, 3本目で全バラ
        assert_eq!(resolve(3, &[(1, 2), (2, 3), (1, 3)]), vec![0, 2, 3]);
    }

    // ---- naive: 崩落iごとに残りの辺でBFSし、成分サイズから非連結ペアを直接数える ----
    fn naive(n: usize, bridges: &[(usize, usize)]) -> Vec<usize> {
        (0..bridges.len())
            .map(|i| {
                let mut adj = vec![Vec::new(); n];
                for &(a, b) in &bridges[i + 1..] {
                    adj[a - 1].push(b - 1);
                    adj[b - 1].push(a - 1);
                }
                let mut seen = vec![false; n];
                let mut connected_pairs = 0;
                for s in 0..n {
                    if seen[s] {
                        continue;
                    }
                    seen[s] = true;
                    let mut queue = std::collections::VecDeque::new();
                    queue.push_back(s);
                    let mut count: usize = 0;
                    while let Some(v) = queue.pop_front() {
                        count += 1;
                        for &to in &adj[v] {
                            if !seen[to] {
                                seen[to] = true;
                                queue.push_back(to);
                            }
                        }
                    }
                    connected_pairs += count * (count - 1) / 2;
                }
                n * (n - 1) / 2 - connected_pairs
            })
            .collect()
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
        for _ in 0..150 {
            let n = 2 + next(25);
            // 木で連結を保証し、余分な辺を足す(全ペア相異・A<Bの制約を守る)
            let mut set = std::collections::HashSet::new();
            for v in 2..=n {
                let u = 1 + next(v as u64 - 1);
                set.insert((u.min(v), u.max(v)));
            }
            for _ in 0..next(n as u64) {
                let a = 1 + next(n as u64);
                let b = 1 + next(n as u64);
                if a != b {
                    set.insert((a.min(b), a.max(b)));
                }
            }
            let mut bridges: Vec<(usize, usize)> = set.into_iter().collect();
            bridges.sort_unstable();
            // 崩落順をシャッフル(Fisher-Yates)
            for i in (1..bridges.len()).rev() {
                bridges.swap(i, next(i as u64 + 1));
            }
            assert_eq!(
                resolve(n, &bridges),
                naive(n, &bridges),
                "n={n} bridges={bridges:?}"
            );
        }
    }

    #[test]
    fn large_chain() {
        // 制約最大: 一直線 1-2-...-N が端から順に崩れる。
        // 橋i崩落後は {1},...,{i} と {i+1..N} → 不便さ = C(N,2) - C(N-i,2)
        let n = 100_000;
        let bridges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
        let total = n * (n - 1) / 2;
        let expected: Vec<usize> = (1..n).map(|i| total - (n - i) * (n - i - 1) / 2).collect();
        assert_eq!(resolve(n, &bridges), expected);
    }
}
