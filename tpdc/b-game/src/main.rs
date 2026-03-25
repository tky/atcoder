// https://atcoder.jp/contests/tdpc/tasks/tdpc_game

fn resolve(a: &[usize], b: &[usize]) -> usize {
    // dp[i][j]: aからi、bからjとった状態で手番のプレイヤーがどれだけ有利か
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

    for i in (0..=a.len()).rev() {
        for j in (0..=b.len()).rev() {
            if i == a.len() && j == b.len() {
                dp[i][j] = 0;
            } else if i == a.len() {
                dp[i][j] = b[j] as isize - dp[i][j + 1];
            } else if j == b.len() {
                dp[i][j] = a[i] as isize - dp[i + 1][j];
            } else {
                dp[i][j] = (a[i] as isize - dp[i + 1][j]).max(b[j] as isize - dp[i][j + 1]);
            }
        }
    }

    let diff = dp[0][0];
    let sum: isize =
        a.iter().map(|&x| x as isize).sum::<isize>() + b.iter().map(|&x| x as isize).sum::<isize>();

    ((sum + diff) / 2) as usize
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::resolve;

    #[test]
    fn both_empty() {
        let a = vec![];
        let b = vec![];
        assert_eq!(resolve(&a, &b), 0);
    }

    #[test]
    fn only_a_one_card() {
        let a = vec![5];
        let b = vec![];
        assert_eq!(resolve(&a, &b), 5);
    }

    #[test]
    fn only_b_one_card() {
        let a = vec![];
        let b = vec![7];
        assert_eq!(resolve(&a, &b), 7);
    }

    #[test]
    fn one_each() {
        let a = vec![2];
        let b = vec![3];
        // 先手は3を取る。後手は2を取る。先手合計は3。
        assert_eq!(resolve(&a, &b), 3);
    }

    #[test]
    fn sample_like_small_case() {
        let a = vec![2, 7];
        let b = vec![4, 1];
        // 会話中で追った例。先手は最終的に6点。
        assert_eq!(resolve(&a, &b), 6);
    }

    #[test]
    fn only_a_multiple() {
        let a = vec![1, 2, 3];
        let b = vec![];
        // 交互に取るので先手は 1 + 3 = 4
        assert_eq!(resolve(&a, &b), 4);
    }

    #[test]
    fn only_b_multiple() {
        let a = vec![];
        let b = vec![4, 5, 6];
        // 交互に取るので先手は 4 + 6 = 10
        assert_eq!(resolve(&a, &b), 10);
    }

    #[test]
    fn same_values() {
        let a = vec![10, 10];
        let b = vec![10, 10];
        // 全部同じなら先手後手で合計は同じ
        assert_eq!(resolve(&a, &b), 20);
    }

    #[test]
    fn brute_force_small_1() {
        let a = vec![1, 3];
        let b = vec![2, 4];
        assert_eq!(resolve(&a, &b), brute_force(&a, &b));
    }

    #[test]
    fn brute_force_small_2() {
        let a = vec![5, 1, 9];
        let b = vec![2, 6];
        assert_eq!(resolve(&a, &b), brute_force(&a, &b));
    }

    #[test]
    fn brute_force_small_3() {
        let a = vec![8, 3];
        let b = vec![7, 4, 2];
        assert_eq!(resolve(&a, &b), brute_force(&a, &b));
    }

    fn brute_force(a: &[usize], b: &[usize]) -> usize {
        fn dfs(a: &[usize], b: &[usize], i: usize, j: usize, turn: bool) -> (usize, usize) {
            if i == a.len() && j == b.len() {
                return (0, 0);
            }

            let mut candidates = Vec::new();

            if i < a.len() {
                let (s1, s2) = dfs(a, b, i + 1, j, !turn);
                if turn {
                    candidates.push((s1 + a[i], s2));
                } else {
                    candidates.push((s1, s2 + a[i]));
                }
            }

            if j < b.len() {
                let (s1, s2) = dfs(a, b, i, j + 1, !turn);
                if turn {
                    candidates.push((s1 + b[j], s2));
                } else {
                    candidates.push((s1, s2 + b[j]));
                }
            }

            if turn {
                candidates
                    .into_iter()
                    .max_by_key(|&(s1, s2)| s1 as isize - s2 as isize)
                    .unwrap()
            } else {
                candidates
                    .into_iter()
                    .max_by_key(|&(s1, s2)| s2 as isize - s1 as isize)
                    .unwrap()
            }
        }

        let (first, _) = dfs(a, b, 0, 0, true);
        first
    }
}
