// https://atcoder.jp/contests/abc212/tasks/abc212_c

// 数列Aと数列Bから1つずつ選んだときの |a - b| の最小値を求める
// N, M <= 2*10^5 なので総当たり O(NM) は不可
// ヒント: 片方をソートすると「aに一番近いbの候補」は2つに絞れる
fn resolve(a: &[usize], b: &[usize]) -> usize {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[1, 6], &[4, 9]), 2);
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&[10], &[10]), 0);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &[82, 76, 82, 82, 71, 70],
                &[17, 39, 67, 2, 45, 35, 22, 24]
            ),
            3
        );
    }
}
