use std::io::{self, Read};

fn resolve(n: usize, intervals: &[(usize, usize, i64)]) -> i64 {
    // by_r[r]: 右端がrの区間たち
    let mut by_r = vec![vec![]; n + 1];

    for &(l, r, a) in intervals {
        by_r[r].push((l, a));
    }

    // dp[j]: 今みている位置までで、最後の1の位置がjの時の最大スコア
    let mut dp = vec![0_i64; n + 1];

    for r in 1..=n {
        // 位置 r を 1 にする
        // 以前の最後の 1 の位置がどこでも、r に 1 を置けば最後の 1 は r になる

        // best は r-1 までの最良の状態を取得
        // その best を使って「r に 1 を置いた状態」dp[r] を作る
        // 後半のループで、右端 r の区間について、
        // 条件を満たす状態 dp[j] にスコアを加算する

        // 現在、最後の 1 の位置が 0, 1, 2, ..., r-1 の状態の中で最大スコア
        // (右端rの区間を処理した後も最良とは限らない)
        let best_prev = dp[0..r].iter().copied().max().unwrap();
        dp[r] = dp[r].max(best_prev);

        // 右端が r の区間 [l, r] を処理する
        // 最後の 1 の位置 j が l 以上なら、この区間には 1 がある
        for &(l, a) in &by_r[r] {
            // 最後の 1 の位置が l, l+1, ..., r の状態は、
            // 区間 [l, r] の中に 1 があることが確定しているので、
            // それらすべてに a を足す
            for j in l..=r {
                dp[j] += a;
            }
        }
    }
    *dp.iter().max().unwrap()
}

const NEG_INF: i64 = -(1_i64 << 60);

struct LazySegTree {
    size: usize,
    data: Vec<i64>, // 区間最大値
    lazy: Vec<i64>, // 遅延加算
}

impl LazySegTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }

        Self {
            size,
            data: vec![0; size * 2],
            lazy: vec![0; size * 2],
        }
    }

    fn apply(&mut self, k: usize, x: i64) {
        self.data[k] += x;
        self.lazy[k] += x;
    }

    fn push(&mut self, k: usize) {
        if self.lazy[k] == 0 {
            return;
        }

        let x = self.lazy[k];

        self.apply(k * 2, x);
        self.apply(k * 2 + 1, x);

        self.lazy[k] = 0;
    }

    // [a, b) に x を加算する
    fn add(&mut self, a: usize, b: usize, x: i64) {
        self.add_inner(a, b, x, 1, 0, self.size);
    }

    fn add_inner(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        if r <= a || b <= l {
            return;
        }

        if a <= l && r <= b {
            self.apply(k, x);
            return;
        }

        self.push(k);

        let mid = (l + r) / 2;
        self.add_inner(a, b, x, k * 2, l, mid);
        self.add_inner(a, b, x, k * 2 + 1, mid, r);

        self.data[k] = self.data[k * 2].max(self.data[k * 2 + 1]);
    }

    // [a, b) の最大値を取得する
    fn query(&mut self, a: usize, b: usize) -> i64 {
        self.query_inner(a, b, 1, 0, self.size)
    }

    fn query_inner(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return NEG_INF;
        }

        if a <= l && r <= b {
            return self.data[k];
        }

        self.push(k);

        let mid = (l + r) / 2;
        let left = self.query_inner(a, b, k * 2, l, mid);
        let right = self.query_inner(a, b, k * 2 + 1, mid, r);

        left.max(right)
    }

    // idx の値を x にする
    fn set(&mut self, idx: usize, x: i64) {
        self.set_inner(idx, x, 1, 0, self.size);
    }

    fn set_inner(&mut self, idx: usize, x: i64, k: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[k] = x;
            self.lazy[k] = 0;
            return;
        }

        self.push(k);

        let mid = (l + r) / 2;
        if idx < mid {
            self.set_inner(idx, x, k * 2, l, mid);
        } else {
            self.set_inner(idx, x, k * 2 + 1, mid, r);
        }

        self.data[k] = self.data[k * 2].max(self.data[k * 2 + 1]);
    }
}

fn resolve2(n: usize, intervals: &[(usize, usize, i64)]) -> i64 {
    // by_r[r]: 右端が r の区間たち
    let mut by_r = vec![Vec::new(); n + 1];

    for &(l, r, a) in intervals {
        by_r[r].push((l, a));
    }

    // dp[0..=n] を Segment Tree で管理する
    // dp[j]: 今見ている位置までで、最後の 1 の位置が j のときの最大スコア
    // j = 0 は「まだ 1 を置いていない」
    let mut seg = LazySegTree::new(n + 1);

    for r in 1..=n {
        // 位置 r に 1 を置く候補を作る
        //
        // 以前の最後の 1 の位置が 0..r-1 のどれであっても、
        // 位置 r に 1 を置けば、最後の 1 は r になる。
        let best_prev = seg.query(0, r);
        seg.set(r, best_prev);

        // 右端が r の区間 [l, r] を処理する
        //
        // 最後の 1 の位置 j が l 以上なら、
        // 区間 [l, r] の中に 1 がある。
        //
        // そのため dp[l..=r] に a を足す。
        // Segment Tree は半開区間なので [l, r + 1) に加算する。
        for &(l, a) in &by_r[r] {
            seg.add(l, r + 1, a);
        }
    }

    seg.query(0, n + 1)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let intervals = vec![(1, 3, 10), (2, 4, -10), (3, 5, 10)];

        assert_eq!(resolve(5, &intervals), 20);
        assert_eq!(resolve2(5, &intervals), 20);
    }

    #[test]
    fn sample_02() {
        let intervals = vec![(1, 1, -100), (2, 2, -100), (3, 3, -100)];

        // 全部 0 にすれば、どの区間も満たさないので 0
        assert_eq!(resolve(3, &intervals), 0);
        assert_eq!(resolve2(3, &intervals), 0);
    }

    #[test]
    fn separated_positive_intervals() {
        let intervals = vec![(1, 1, 10), (5, 5, 10)];

        // 10001 にすれば両方満たせる
        assert_eq!(resolve(5, &intervals), 20);
        assert_eq!(resolve2(5, &intervals), 20);
    }

    #[test]
    fn one_interval_positive() {
        let intervals = vec![(2, 5, 10)];

        // 2..=5 のどこかに 1 を置けば +10
        assert_eq!(resolve(5, &intervals), 10);
        assert_eq!(resolve2(5, &intervals), 10);
    }

    #[test]
    fn one_interval_negative() {
        let intervals = vec![(2, 5, -10)];

        // 2..=5 に 1 を置かなければよいので 0
        assert_eq!(resolve(5, &intervals), 0);
        assert_eq!(resolve2(5, &intervals), 0);
    }

    #[test]
    fn positive_and_negative_overlap() {
        let intervals = vec![(1, 3, 10), (2, 4, -100), (4, 5, 10)];

        // 10001 なら [1,3] と [4,5] を満たし、[2,4] は満たさない
        // score = 10 + 10 = 20
        assert_eq!(resolve(5, &intervals), 20);
        assert_eq!(resolve2(5, &intervals), 20);
    }

    #[test]
    fn all_positive_can_be_satisfied() {
        let intervals = vec![(1, 3, 5), (2, 4, 7), (3, 5, 11)];

        // 例えば 00100 なら全部満たす
        assert_eq!(resolve(5, &intervals), 23);
        assert_eq!(resolve2(5, &intervals), 23);
    }

    #[test]
    fn choose_not_to_take_negative_even_with_positive() {
        let intervals = vec![(1, 5, 100), (3, 3, -1000)];

        // 位置3以外に1を置けば +100 だけ取れる
        assert_eq!(resolve(5, &intervals), 100);
        assert_eq!(resolve2(5, &intervals), 100);
    }
}
