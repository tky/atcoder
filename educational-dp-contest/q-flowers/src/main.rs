fn resolve(h: &[u64], aa: &[u64]) -> u64 {
    let len = h.len();
    // dp[i]: 花iを最後に選ぶ時の美しさの総和の最大値
    // dp[i] = a_i + max { dp[j] | j < i, h_j < h_i }
    let mut dp = vec![0u64; len];
    dp[0] = aa[0];

    for i in 1..len {
        let mut max = 0u64;
        for j in 0..i {
            if max < dp[j] && h[j] < h[i] {
                max = dp[j];
            }
        }
        dp[i] = aa[i] + max;
    }
    *dp.iter().max().unwrap()
}

fn resolve_by_segment_tree(h: &[usize], aa: &[u64]) -> u64 {
    let n = h.len();
    let mut seg = SegmentTree::new(n + 1);

    for i in 0..n {
        let best_before = seg.query(0, h[i]);
        let value = best_before + aa[i];
        seg.update(h[i], value);
    }
    seg.query(0, n + 1)
}
/**
 * RMQ（Range Maximum Query）用の基本形
 * 区間[i, j)の最大値を求める
 */
#[derive(Debug)]
struct SegmentTree {
    n: usize,       // 葉の数（2べきに丸めた後）
    data: Vec<u64>, // data[1] が根
}

/**
 * n=8の場合このようなイメージ
 * data[n+pos]が葉（元のデータ)
 */
/**
                  data[1]
           /               \
          data[2]             data[3]
     /      \            /      \
  data[4]   data[5]    data[6]   data[7]
 /   \      /   \      /   \      /   \
8     9   10   11    12   13    14   15
*/

/**
 * pos:3を更新したい場合
 * 葉の部分を更新data[8(3 + n)]に更新したい値を入れる
 * 親へ遡って最大値を更新していく
 * data[5] = max(data[8], data[9])
 * data[2] = max(data[4], data[5])
 * data[1] = max(data[2], data[3])
 */

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            n,
            data: vec![0; 2 * n],
        }
    }

    // a[pos] = max(a[pos], x) のように使うことを想定
    fn update(&mut self, pos: usize, x: u64) {
        let mut i = pos + self.n;
        self.data[i] = x;
        while i > 1 {
            i /= 2;
            self.data[i] = self.data[i * 2].max(self.data[i * 2 + 1]);
        }
    }

    // 区間 [l, r) の最大値
    fn query(&self, l: usize, r: usize) -> u64 {
        let mut left = l + self.n;
        let mut right = r + self.n;
        let mut res = 0;

        while left < right {
            // leftが右の子の場合、
            // 左のこの場合は親要素を比較すれば良い
            if left % 2 == 1 {
                res = res.max(self.data[left]);
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                res = res.max(self.data[right]);
            }
            left /= 2;
            right /= 2;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_01() {
        assert_eq!(resolve(&vec![3, 1, 4, 2], &vec![10, 20, 30, 40]), 60);
        assert_eq!(
            resolve_by_segment_tree(&vec![3, 1, 4, 2], &vec![10, 20, 30, 40]),
            60
        );
    }

    #[test]
    fn sample_02() {
        assert_eq!(resolve(&vec![1], &vec![10]), 10);
        assert_eq!(resolve_by_segment_tree(&vec![1], &vec![10]), 10);
    }

    #[test]
    fn sample_03() {
        assert_eq!(
            resolve(
                &vec![4, 2, 5, 8, 3, 6, 1, 7, 9],
                &vec![6, 8, 8, 4, 6, 3, 5, 7, 5]
            ),
            31
        );
        assert_eq!(
            resolve_by_segment_tree(
                &vec![4, 2, 5, 8, 3, 6, 1, 7, 9],
                &vec![6, 8, 8, 4, 6, 3, 5, 7, 5]
            ),
            31
        );
    }
}
#[cfg(test)]
mod segtree_query_test {
    use super::*;

    #[test]
    fn segtree_query_test() {
        let mut seg = SegmentTree::new(8);
        seg.update(2, 10);
        seg.update(4, 7);
        seg.update(6, 20);

        assert_eq!(seg.query(0, 8), 20);
        assert_eq!(seg.query(0, 4), 10);
        assert_eq!(seg.query(3, 7), 20);
        assert_eq!(seg.query(3, 5), 7);
        assert_eq!(seg.query(0, 2), 0);
    }

    #[test]
    fn dp_q_sample_1() {
        let h = vec![3, 1, 4, 2];
        let a = vec![10, 20, 30, 40];
        assert_eq!(resolve(&h, &a), 60);
    }
}
fn main() {
    println!("Hello, world!");
}
