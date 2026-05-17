/**
 * 過去の足場 i から直線を作る
 * 現在の h[j] を x として、一番小さい直線の値を取る
 * dp[j] を計算する
 * 足場 j から新しい直線を追加する
 */
use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Line {
    a: i128,
    b: i128,
}

impl Line {
    // y = ax + b
    fn value(&self, x: i128) -> i128 {
        self.a * x + self.b
    }
}

// l1, l2, l3 の順に直線を追加するとき、l2 が不要かどうか

/**
 * l2が必要の場合
 * A: l1とl2交点
 * B: l2とl3の交点
 * A < Bの場合、区間[A, B]においてl2が最小になる
 * y
↑
|
|  l1
|    \
|     \
|      \
|       \        l2
|        \      /
|         \    /    /
|          \  /    /
|           A     /
|          / \   /
|         /   \ /
|        /     B
|       /     / \
|            /   \  l3
+--------------------------------→ x
             A < B
 */
fn is_bad(l1: Line, l2: Line, l3: Line) -> bool {
    // h[i]は単調増加なので、a=-2h[i]は単調減少する
    // l1.a > l2.a > l3.a
    // 交点(l1, l2) >= 交点(l2, l3) なら l2 は不要
    //
    // (b2 - b1) / (a1 - a2) >= (b3 - b2) / (a2 - a3)
    //
    // 今回は傾き a が単調減少する:
    // a1 > a2 > a3
    // なので分母は正
    (l2.b - l1.b) * (l2.a - l3.a) >= (l3.b - l2.b) * (l1.a - l2.a)
}

struct ConvexHullTrick {
    lines: VecDeque<Line>,
}

impl ConvexHullTrick {
    fn new() -> Self {
        Self {
            lines: VecDeque::new(),
        }
    }

    // 新しい直線を追加するときに、
    // 「将来どの x でも最小にならない直線」を末尾から消す処理
    fn add_line(&mut self, line: Line) {
        while self.lines.len() >= 2 {
            let l2 = self.lines.pop_back().unwrap();
            let l1 = *self.lines.back().unwrap();

            // l1, l2, line の順に並んでいるとき、
            // 真ん中の l2 は不要か？
            if is_bad(l1, l2, line) {
                // l2 は不要なので捨てる
                continue;
            } else {
                // l2 は必要なので戻す
                self.lines.push_back(l2);
                // これ以上、末尾を消す必要はない
                break;
            }
        }

        // 最後の直線は、今後の query で一度も使われない可能性はある。
        // しかし、deque の末尾に置かれているだけなら、query の答えを邪魔しない。
        // なので不要判定をしないで追加している
        self.lines.push_back(line);
    }

    fn query(&mut self, x: i128) -> i128 {
        // 前提：一番良い直線は先頭にある
        //
        // 今の x に対して、先頭の直線より 2 本目の直線の方が良いなら、
        // 先頭の直線はもう二度と使わないので捨てる
        while self.lines.len() >= 2 {
            let l1 = self.lines[0];
            let l2 = self.lines[1];

            // 先頭より2本目の方が良い場合、先頭はもう使わないので捨てる
            if l1.value(x) >= l2.value(x) {
                self.lines.pop_front();
            } else {
                break;
            }
        }

        self.lines[0].value(x)
    }
}

fn resolve(h: &[i128], c: i128) -> i128 {
    let n = h.len();
    let mut dp = vec![0_i128; n];

    let mut cht = ConvexHullTrick::new();

    // dp[0] = 0
    // line_0(x) = -2h[0]x + dp[0] + h[0]^2
    cht.add_line(Line {
        a: -2 * h[0],
        b: dp[0] + h[0] * h[0],
    });

    for j in 1..n {
        let x = h[j];

        let best = cht.query(x);

        dp[j] = x * x + c + best;

        cht.add_line(Line {
            a: -2 * h[j],
            b: dp[j] + h[j] * h[j],
        });
    }

    dp[n - 1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let c: i128 = iter.next().unwrap().parse().unwrap();

    let h: Vec<i128> = (0..n)
        .map(|_| iter.next().unwrap().parse::<i128>().unwrap())
        .collect();

    let ans = resolve(&h, c);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_like_01() {
        let h = vec![10, 20, 30, 40];
        let c = 100;

        // 最適:
        // 10 -> 20 -> 30 -> 40
        // 各ジャンプ: (10)^2 + 100 = 200
        // 合計: 600
        assert_eq!(resolve(&h, c), 600);
    }

    #[test]
    fn sample_like_02() {
        let h = vec![10, 20, 30, 40];
        let c = 1;

        // 最適:
        // 10 -> 20 -> 30 -> 40
        // 各ジャンプ: (10)^2 + 1 = 101
        // 合計: 303
        assert_eq!(resolve(&h, c), 303);
    }

    #[test]
    fn sample_like_03() {
        let h = vec![1, 2, 3, 4, 5];
        let c = 10;

        // 注意:
        // 隣に進むと 4 回ジャンプなので 11 * 4 = 44
        // しかし直接ジャンプ 1 -> 5 なら
        // (5 - 1)^2 + 10 = 16 + 10 = 26
        // なので最小は 26
        assert_eq!(resolve(&h, c), 26);
    }

    #[test]
    fn two_stones() {
        let h = vec![10, 25];
        let c = 7;

        // 10 -> 25
        // (25 - 10)^2 + 7 = 225 + 7 = 232
        assert_eq!(resolve(&h, c), 232);
    }

    #[test]
    fn via_is_best() {
        let h = vec![1, 10, 20];
        let c = 1;

        // 1 -> 20:
        // (20 - 1)^2 + 1 = 361 + 1 = 362
        //
        // 1 -> 10 -> 20:
        // (10 - 1)^2 + 1 + (20 - 10)^2 + 1
        // = 81 + 1 + 100 + 1 = 183
        assert_eq!(resolve(&h, c), 183);
    }

    #[test]
    fn large_c_makes_fewer_jumps_better() {
        let h = vec![1, 10, 20];
        let c = 1000;

        // 1 -> 20:
        // (20 - 1)^2 + 1000 = 361 + 1000 = 1361
        //
        // 1 -> 10 -> 20:
        // 81 + 1000 + 100 + 1000 = 2181
        assert_eq!(resolve(&h, c), 1361);
    }

    #[test]
    fn compare_with_naive_small_case() {
        let h = vec![3, 8, 13, 20, 31];
        let c = 17;

        assert_eq!(resolve(&h, c), resolve_naive(&h, c));
    }

    #[test]
    fn compare_with_naive_many_small_cases() {
        let cases = vec![
            (vec![1, 2], 1),
            (vec![1, 2, 3], 1),
            (vec![1, 3, 6], 10),
            (vec![2, 5, 9, 14], 3),
            (vec![1, 4, 10, 20, 35], 100),
            (vec![5, 6, 7, 8, 100], 50),
        ];

        for (h, c) in cases {
            assert_eq!(resolve(&h, c), resolve_naive(&h, c));
        }
    }

    fn resolve_naive(h: &[i128], c: i128) -> i128 {
        let n = h.len();
        let mut dp = vec![i128::MAX / 4; n];
        dp[0] = 0;

        for j in 1..n {
            for i in 0..j {
                let diff = h[j] - h[i];
                let cost = dp[i] + diff * diff + c;
                dp[j] = dp[j].min(cost);
            }
        }

        dp[n - 1]
    }
}
