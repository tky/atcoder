// https://atcoder.jp/contests/abc174/tasks/abc174_e

// N本の丸太(長さ A_i)を合計K回まで切れる。
// 切断後の「最長の丸太」を最小化し、その長さを切り上げ整数で出力する。
// (答えは整数の長さxとして探索してよい: 「最長をx以下にできるか?」を考える)
//
// Buy an Integerからのステップアップ:
// - 探索対象は「答えの候補 x = 1..=10^9」
// - 判定問題: 「全丸太をx以下に切り分けるのに必要な切断回数はK回以下か?」
//   長さaの丸太をx以下にするには ceil(a/x) 本に分ける = ceil(a/x) - 1 回切る
// - 単調性の向きに注意: xが大きいほど楽(必要回数が減る)。
//   つまり [不可能...不可能 | 可能...可能] と、これまでと帯の向きが逆!
//   ok/ng の左右がどうなるか考えてから書くこと
fn resolve(a: &[usize], k: usize) -> usize {
    let mut bottom = 0usize;
    let mut top = *a.iter().max().unwrap() as usize;

    while bottom + 1 < top {
        let l = (bottom + top) / 2;

        let mut count = 0;

        for &ai in a {
            count += ai.div_ceil(l) - 1;
        }

        if count <= k {
            top = l;
        } else {
            bottom = l;
        }
    }
    top
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[7, 9], 3), 4);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[3, 4, 5], 0), 5);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &[
                    158260522, 877914575, 602436426, 24979445, 861648772, 623690081, 433933447,
                    476190629, 262703497, 211047202
                ],
                10
            ),
            292638192
        );
    }
}
