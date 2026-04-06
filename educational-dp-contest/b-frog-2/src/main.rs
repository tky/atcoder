fn resolve(hs: &[usize], k: usize) -> usize {
    let len = hs.len();

    let k = k.min(len);

    let mut prevs = vec![0; k];

    for i in 0..(k - 1) {
        prevs[i] = hs[k - 1 - i].abs_diff(hs[0]);
    }

    for i in k..len {
        // let mut cur = prevs[0] + hs[i].abs_diff(hs[i - k]);
        let mut cur = prevs[0] + hs[i].abs_diff(hs[i - 1]);
        for k in 1..k {
            let tmp = prevs[k] + hs[i].abs_diff(hs[i - k - 1]);
            if cur > tmp {
                cur = tmp;
            }
        }

        for j in (1..k).rev() {
            prevs[j] = prevs[j - 1];
        }
        prevs[0] = cur;
    }

    prevs[0]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        assert_eq!(resolve(&[10, 30, 40, 20], 2), 30);
        assert_eq!(resolve(&[10, 10], 2), 0);
        assert_eq!(resolve(&[30, 10, 60, 10, 50], 2), 40);

        assert_eq!(resolve(&[10, 30, 40, 50, 20], 3), 30);
    }
    #[test]
    fn sample_02_k_is_greater_than_n() {
        assert_eq!(resolve(&[10, 30], 100), 20);
    }

    #[test]
    fn k_is_greater_than_n_another_case() {
        assert_eq!(resolve(&[10, 20, 10], 100), 0);
    }
}
