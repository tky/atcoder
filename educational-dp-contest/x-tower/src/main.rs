use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Block {
    w: usize,
    s: usize,
    v: u64,
}

fn resolve(blocks: &mut Vec<Block>) -> u64 {
    // ブロックは w + s の昇順に並べる。
    //
    // 理由:
    // この問題では、あるブロック i の上に乗っている重さが s_i 以下でなければならない。
    // つまり、ブロック i 自身を含めて、その位置以上に存在できる最大重量は
    //
    //     w_i + s_i
    //
    // と考えられる。
    //
    // w_i + s_i が小さいブロックは、自分を含めた上側の塔の許容量が小さいため、
    // 下に置くよりも上に置くべき。
    // 逆に w_i + s_i が大きいブロックは、より下に置ける候補になる。
    //
    // 2つのブロック A, B について、もし
    //
    //     w_A + s_A <= w_B + s_B
    //
    // なら、A を B より上に置く順番を考えればよい。
    // すべての最適解がこの順番とは限らないが、少なくとも1つの最適解は
    // w + s の昇順に並べた形にできる。
    //
    // そのため、DP ではこの順番でブロックを処理し、
    // 「今ある塔の下に現在のブロックを追加する」かどうかだけを考えればよい。
    // 使ったブロック集合を状態として持つ必要はない。
    blocks.sort_by_key(|b| b.w + b.s);

    // w_i, s_i <= 10000, N <= 1000
    // ただし、この問題では w+s の最大を使えば十分
    let max_weight = blocks.iter().map(|b| b.w + b.s).max().unwrap();

    // dp[j]: 合計重量 j の塔を作れるときの価値の最大値
    let mut dp = vec![0_u64; max_weight + 1];

    for block in blocks.iter() {
        let w = block.w;
        let s = block.s;
        let v = block.v;

        // 同じブロックを複数回使わないため、逆順に回す
        for j in (0..=s).rev() {
            if j + w <= max_weight {
                dp[j + w] = dp[j + w].max(dp[j] + v);
            }
        }
    }

    *dp.iter().max().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut blocks = Vec::with_capacity(n);
    for _ in 0..n {
        let w: usize = it.next().unwrap().parse().unwrap();
        let s: usize = it.next().unwrap().parse().unwrap();
        let v: u64 = it.next().unwrap().parse().unwrap();

        blocks.push(Block { w, s, v });
    }

    let ans = resolve(&mut blocks);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let mut blocks = vec![
            Block { w: 2, s: 2, v: 20 },
            Block { w: 2, s: 1, v: 30 },
            Block { w: 3, s: 1, v: 40 },
        ];

        assert_eq!(resolve(&mut blocks), 50);
    }

    #[test]
    fn sample_02() {
        let mut blocks = vec![
            Block {
                w: 1,
                s: 10000,
                v: 1000000000,
            },
            Block {
                w: 1,
                s: 10000,
                v: 1000000000,
            },
            Block {
                w: 1,
                s: 10000,
                v: 1000000000,
            },
            Block {
                w: 1,
                s: 10000,
                v: 1000000000,
            },
            Block {
                w: 1,
                s: 10000,
                v: 1000000000,
            },
        ];

        assert_eq!(resolve(&mut blocks), 5000000000);
    }

    #[test]
    fn sample_03() {
        let mut blocks = vec![
            Block { w: 1, s: 1, v: 1 },
            Block { w: 2, s: 1, v: 2 },
            Block { w: 3, s: 1, v: 4 },
            Block { w: 4, s: 1, v: 8 },
            Block { w: 5, s: 1, v: 16 },
        ];

        assert_eq!(resolve(&mut blocks), 17);
    }

    #[test]
    fn single_block() {
        let mut blocks = vec![Block {
            w: 10,
            s: 1,
            v: 100,
        }];

        assert_eq!(resolve(&mut blocks), 100);
    }

    #[test]
    fn cannot_stack_both_due_to_strength() {
        let mut blocks = vec![
            Block {
                w: 100,
                s: 1,
                v: 1000,
            },
            Block {
                w: 10,
                s: 90,
                v: 100,
            },
        ];

        assert_eq!(resolve(&mut blocks), 1000);
    }

    #[test]
    fn can_stack_two_blocks() {
        let mut blocks = vec![
            Block {
                w: 3,
                s: 10,
                v: 100,
            },
            Block { w: 5, s: 3, v: 200 },
        ];

        assert_eq!(resolve(&mut blocks), 300);
    }

    #[test]
    fn choose_better_subset() {
        let mut blocks = vec![
            Block { w: 4, s: 4, v: 10 },
            Block { w: 3, s: 10, v: 20 },
            Block { w: 2, s: 2, v: 100 },
        ];

        assert_eq!(resolve(&mut blocks), 130);
    }
}
