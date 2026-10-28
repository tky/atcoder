// https://atcoder.jp/contests/atc001/tasks/unionfind_a (ATC001 B。タスクIDは unionfind_a)

// N頂点・辺ゼロの無向グラフから始めて、Q個のクエリを順に処理する:
//   P=0: 頂点AとBを辺でつなぐ(連結クエリ)
//   P=1: AとBが連結か答える(判定クエリ)。A=Bは常に連結。多重辺・自己ループあり
// 頂点は最初から0-indexed(0 ≤ A,B ≤ N-1)なので今回は変換不要。
//
// ここから Step 3: Union-Find。二分探索と同じく**毎回手書き**する。考えどころ:
// - 判定のたびにBFS/DFSすると1回O(N+辺数)。Q=2*10^5, N=10^5 では間に合わない
//   (テストの naive がまさにそれ。小さい入力の答え合わせ専用)
// - Union-Find の芯は「各頂点が親を1つだけ覚える。親をたどり着いた先の根が成分の代表」
//   - find(v): vの根を返す。「根である」をどう表すか?
//     (parent[v] == v 方式か、番兵方式か — 番兵にするなら規約を思い出すこと)
//   - union(a, b): 両方の根を求め、片方の根をもう片方の根の**子**にする
//   - same(a, b): find(a) == find(b)
// - まず素朴に書いて動かしてよい。ただし chain_performance テストが
//   「一直線に10^5個つないでから端と端を10^5回聞く」を投げてくる。
//   素朴版が遅すぎたら、高速化を2つ調べる:
//   経路圧縮(path compression)と、サイズ(またはランク)による合併(union by size/rank)
//
// 制約: 1 ≤ N ≤ 10^5, 1 ≤ Q ≤ 2*10^5
//
// resolve はクエリ列を受け取り、判定クエリ(P=1)の答えだけを順に Vec<bool> で返す
fn resolve(n: usize, queries: &[(usize, usize, usize)]) -> Vec<bool> {
    let mut ans = Vec::new();

    let mut uf = UnionFind::new(n);

    for &(q, x, y) in queries {
        if q == 0 {
            uf.union(x, y);
        } else {
            ans.push(uf.same(x, y));
        }
    }
    ans
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<isize>,
    // その頂点を根とする木の頂点数。根のスロットの値だけが正しい
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
            // 小さい木の根を大きい木の根の下にぶら下げる(union by size)。
            // サイズの大小は深さの大小を保証しないが、それでよい:
            // 頂点の深さが1増えるのは所属する木のサイズが2倍以上になるときだけなので、
            // どの頂点も深さ ≤ log₂n に抑えられ、findは根まで log₂n 歩以内で着く
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
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let queries = [
            (0, 1, 2),
            (0, 3, 2),
            (1, 1, 3), // Yes
            (1, 1, 4), // No
            (0, 2, 4),
            (1, 4, 1), // Yes
            (0, 4, 2), // 多重辺
            (0, 0, 0), // 自己ループ
            (1, 0, 0), // Yes
        ];
        assert_eq!(resolve(8, &queries), vec![true, false, true, true]);
    }

    #[test]
    fn handmade() {
        // 辺ゼロ: 自分自身とは連結、他とは非連結
        assert_eq!(resolve(2, &[(1, 0, 0), (1, 0, 1)]), vec![true, false]);
        // 推移律: 0-1, 1-2 とつなぐと 0-2 も連結
        assert_eq!(resolve(3, &[(0, 0, 1), (0, 1, 2), (1, 0, 2)]), vec![true]);
        // つなぐ向きは無関係(無向): B側から聞いても同じ
        assert_eq!(
            resolve(3, &[(0, 2, 0), (1, 0, 2), (1, 2, 0)]),
            vec![true, true]
        );
        // すでに連結な2頂点を再度つないでも壊れない
        assert_eq!(
            resolve(4, &[(0, 0, 1), (0, 1, 2), (0, 2, 0), (1, 0, 2), (1, 3, 0)]),
            vec![true, false]
        );
        // N=1
        assert_eq!(resolve(1, &[(1, 0, 0)]), vec![true]);
    }

    // ---- naive: クエリごとにBFSで連結判定(Union-Findとは別原理の答え合わせ) ----
    fn naive(n: usize, queries: &[(usize, usize, usize)]) -> Vec<bool> {
        let mut adj = vec![Vec::new(); n];
        let mut out = Vec::new();
        for &(p, a, b) in queries {
            if p == 0 {
                adj[a].push(b);
                adj[b].push(a);
            } else {
                let mut seen = vec![false; n];
                seen[a] = true;
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(a);
                while let Some(v) = queue.pop_front() {
                    for &to in &adj[v] {
                        if !seen[to] {
                            seen[to] = true;
                            queue.push_back(to);
                        }
                    }
                }
                out.push(seen[b]);
            }
        }
        out
    }

    #[test]
    fn random_cross_check() {
        let mut seed = 20261028u64;
        let mut next = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as usize
        };
        for _ in 0..200 {
            let n = 1 + next(30);
            let q = 1 + next(60);
            let queries: Vec<(usize, usize, usize)> = (0..q)
                .map(|_| (next(2), next(n as u64), next(n as u64)))
                .collect();
            assert_eq!(
                resolve(n, &queries),
                naive(n, &queries),
                "n={n} queries={queries:?}"
            );
        }
    }

    #[test]
    fn chain_performance_reversed() {
        // chain_performance と同じ鎖を、**AとBを入れ替えて**つなぐ: (1,0), (2,1), ..., (n-1,n-2)。
        // 「合併の向きが実質固定」の実装は、元の向きか入れ替えた向きのどちらかで必ず深い木を作る
        let n = 100_000;
        let mut queries: Vec<(usize, usize, usize)> = (1..n).map(|i| (0, i, i - 1)).collect();
        let ask = 100_000;
        queries.extend(std::iter::repeat_n((1, 0, n - 1), ask));
        assert_eq!(resolve(n, &queries), vec![true; ask]);
    }

    #[test]
    fn chain_performance() {
        // 制約最大級: 0-1-2-...-(n-1) と一直線につないでから、
        // 端と端 (0, n-1) の判定を大量に投げる。
        // 親をたどる距離が最悪 n になる形なので、素朴なUnion-Findだとここで詰まる
        let n = 100_000;
        let mut queries: Vec<(usize, usize, usize)> = (1..n).map(|i| (0, i - 1, i)).collect();
        let ask = 100_000;
        queries.extend(std::iter::repeat_n((1, 0, n - 1), ask));
        assert_eq!(resolve(n, &queries), vec![true; ask]);
    }
}
