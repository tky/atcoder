/**
 * 1. 全マス DP は H,W が大きすぎて無理
 * 2. 壁とゴールだけを points に入れる
 * 3. ways(a,b) は combination で 2点間の経路数を数える
 * 4. dp[i] は points[i] まで、以前の壁を通らずに来る経路数
 * 5. 全経路数から、壁を通る経路数を引く
 */
use std::io::{self, Read};

const MOD: usize = 1_000_000_007;

// base^exp mod MOD を高速に計算する。
//
// 普通に base^exp を作ると値が巨大になるので、
// 掛け算のたびに MOD を取る。
// 計算量は O(log exp)。
fn mod_pow(mut base: usize, mut exp: usize) -> usize {
    let mut result = 1usize;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % MOD;
        }
        // baseを２乗してexpを半分にする
        base = base * base % MOD;
        exp /= 2;
    }

    result
}

/*
  nCr = n 個から r 個を選ぶ通り数を計算する。

  数学的にはnCr = n! / (r! (n-r)!)で計算できるが
  n!が巨大になるためMODを取る必要があるため、割り算をそのまま使うことができない

  グリッド上で (r1, c1) から (r2, c2) まで右・下だけで移動する場合、
  必要な移動回数は次の通り。

    down  = r2 - r1
    right = c2 - c1

  合計で down + right 回移動する。
  そのうち down 回を「下に進むタイミング」として選べば経路が決まるので、

    C(down + right, down)

  で経路数を求められる。

  例: (1,1) -> (3,4)

    行: 1 -> 3 なので下に 2 回
    列: 1 -> 4 なので右に 3 回

  合計 5 回の移動のうち、どの 2 回を「下」にするかを選ぶ。
  たとえば、

    D R R D R
    R D R R D
    R R R D D

  のように、D の位置を選べば、Rを考慮することなく経路が一意に決まる。
*/
struct Comb {
    // fact[i] = i! mod MOD
    fact: Vec<usize>,

    // inv_fact[i] = (i!)^{-1} mod MOD
    inv_fact: Vec<usize>,
}

impl Comb {
    fn new(n: usize) -> Self {
        let mut fact = vec![1usize; n + 1];
        let mut inv_fact = vec![1usize; n + 1];

        for i in 1..=n {
            fact[i] = fact[i - 1] * i % MOD;
        }

        inv_fact[n] = mod_pow(fact[n], MOD - 2);

        for i in (1..=n).rev() {
            inv_fact[i - 1] = inv_fact[i] * i % MOD;
        }

        Self { fact, inv_fact }
    }

    fn ncr(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }

        self.fact[n] * self.inv_fact[r] % MOD * self.inv_fact[n - r] % MOD
    }
}

// from から to まで右・下だけで行く経路数
fn ways(from: (usize, usize), to: (usize, usize), comb: &Comb) -> usize {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if r1 > r2 || c1 > c2 {
        return 0;
    }

    let down = r2 - r1;
    let right = c2 - c1;

    comb.ncr(down + right, down)
}

fn resolve(h: usize, w: usize, walls: &[(usize, usize)]) -> usize {
    let mut points = walls.to_vec();

    // ゴールも「重要な点」として追加する
    points.push((h, w));

    // 左上から右下の順に処理する
    points.sort();

    let comb = Comb::new(h + w);

    // dp[i]:
    // (1,1) から points[i] まで行く経路数
    // ただし、途中でそれ以前の壁を通らないもの
    let mut dp = vec![0usize; points.len()];

    for i in 0..points.len() {
        let p = points[i];

        // 壁を無視した場合の (1,1) -> p の全経路数
        let mut count = ways((1, 1), p, &comb);

        // 以前の壁を通って p に来る経路を引く
        for j in 0..i {
            let q = points[j];

            // dp[j]:
            //   (1, 1) から q まで、以前の壁を通らずに行く経路数
            //
            // ways(q, p):
            //   q から p まで右・下だけで行く経路数
            //
            // through:
            //   q を通って p に到達する経路数
            //   前半 dp[j] 通りと後半 ways(q, p) 通りを組み合わせるので掛け算
            let through = dp[j] * ways(q, p, &comb) % MOD;

            count = (count + MOD - through) % MOD;
        }

        dp[i] = count;
    }

    // ゴール (h,w) を追加して sort しているので、最後がゴール
    *dp.last().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut walls = Vec::new();

    for _ in 0..n {
        let r: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        walls.push((r, c));
    }

    println!("{}", resolve(h, w, &walls));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let h = 3;
        let w = 4;
        let walls = vec![(2, 2), (1, 4)];
        assert_eq!(resolve(h, w, &walls), 3);
    }

    #[test]
    fn sample_02() {
        let h = 5;
        let w = 2;
        let walls = vec![(2, 1), (4, 2)];
        assert_eq!(resolve(h, w, &walls), 0);
    }

    #[test]
    fn sample_03() {
        let h = 5;
        let w = 5;
        let walls = vec![(2, 2), (2, 3), (3, 2), (3, 3)];
        assert_eq!(resolve(h, w, &walls), 10);
    }
}
