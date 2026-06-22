use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: u64 = it.next().unwrap().parse().unwrap();

    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let x: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        graph[x].push(y);
        graph[y].push(x);
    }

    let ans = resolve(&graph, m);

    for x in ans {
        println!("{}", x);
    }
}

fn resolve(graph: &[Vec<usize>], m: u64) -> Vec<u64> {
    let n = graph.len();

    // dp[v]:
    // v を黒に固定し、v を根とする部分木の中だけで
    // 黒い頂点集合が連結になる塗り方の数
    let mut dp = vec![1_u64; n];

    // nは存在しない値を適当に設定
    dfs_down(0, n, graph, m, &mut dp);

    // ans[v]:
    // 木全体で v を黒に固定したときの答え
    let mut ans = vec![0_u64; n];

    // from_parent[v]:
    // v から見た「親方向」を使う場合の通り数
    // 根には親方向がないので 0
    // 木は任意の頂点を root にできる
    // → その仮 root には親がない
    // → 親方向を使う通り数は 0
    // → 最初の from_parent は 0
    dfs_all(0, n, 0, graph, m, &dp, &mut ans);

    ans
}

fn dfs_down(v: usize, parent: usize, graph: &[Vec<usize>], m: u64, dp: &mut [u64]) {
    // 葉の場合、ループしないので初期値1がdp[v]の値になる
    let mut prod = 1_u64;

    for &to in &graph[v] {
        if to == parent {
            continue;
        }
        // 子 to の部分木に関する dp[to] を先に計算する。
        // dfs_down から戻ってきた時点で、dp[to] は完成している。
        dfs_down(to, v, graph, m, dp);

        // v を含む連結集合を作るとき、子 to 側については、
        //   使わない: 1 通り
        //   使う    : dp[to] 通り
        // のどちらかを選べる。
        prod = prod * (dp[to] + 1) % m;
    }

    dp[v] = prod;
}

// dfs_all:
//   v を黒に固定したときの木全体での答え ans[v] を計算し、
//   さらに子へ「親方向を使う場合の通り数」を渡す。
//
// from_parent:
//   v から見た親方向を使う場合の通り数。
//   つまり、親を黒にして、v 側以外で連結な黒集合を作る通り数。
fn dfs_all(
    v: usize,
    parent: usize,
    // from_parent:
    // vからみた「親方向」を使う場合の通り数
    // vの親側に黒を伸ばす時、親を黒にして、v側以外で連結な黒集合を作る通り数
    from_parent: u64,
    graph: &[Vec<usize>],
    m: u64,
    dp: &[u64],
    ans: &mut [u64],
) {
    let deg = graph[v].len();

    // values[i]:
    //   v から graph[v][i] 方向について、
    //   その方向を使わない 1 通り
    //   +
    //   その方向を使う通り数
    //
    // 隣接先が親なら、使う通り数は from_parent。
    // 隣接先が子なら、使う通り数は dp[to]。
    let mut values = vec![0_u64; deg];

    for i in 0..deg {
        let to = graph[v][i];
        let msg = if to == parent { from_parent } else { dp[to] };
        values[i] = (msg + 1) % m;
    }

    // ans[v] は全方向の values の積
    // ans[v] = Π(各方向を使わない or 使う通り数)
    let mut total = 1_u64;
    for &x in &values {
        total = total * x % m;
    }
    ans[v] = total;

    // prefix[i] = values[0] * ... * values[i - 1]
    // 例
    // values = [a, b, c, d, e]
    // prefix[0] = 1
    // prefix[1] = a
    // prefix[2] = a * b
    // prefix[3] = a * b * c
    // prefix[4] = a * b * c * d
    // prefix[5] = a * b * c * d * e
    let mut prefix = vec![1_u64; deg + 1];
    for i in 0..deg {
        prefix[i + 1] = prefix[i] * values[i] % m;
    }

    // suffix[i] = values[i] * ... * values[deg - 1]
    // suffix[0] = a * b * c * d * e
    // suffix[1] = b * c * d * e
    // suffix[2] = c * d * e
    // suffix[3] = d * e
    // suffix[4] = e
    // suffix[5] = 1
    let mut suffix = vec![1_u64; deg + 1];
    for i in (0..deg).rev() {
        suffix[i] = suffix[i + 1] * values[i] % m;
    }

    for i in 0..deg {
        let to = graph[v][i];

        if to == parent {
            continue;
        }

        // to に渡す親方向の値を作る。
        //
        // to から見ると、親方向は v 側。
        // そのため、v を黒にしたうえで、
        // to 方向以外のすべての方向を自由に使う/使わないできる。
        //
        // よって「to 方向を除いた values の積」が from_parent_for_child になる。
        // toを除いたnの積の計算
        // values = [a, b, c, d, e] で、c(i = 2) を覗きたいとする
        // prefix[2] : a * b
        // suffix[3]: d * e
        // よりprefix[i] * suffix[i + 1]
        /*
                v
              / | \
             /  |  \
           to   a   b

        to に渡す値は、

        v を黒にして、
        a 方向は使う/使わない
        b 方向は使う/使わない
        親方向があれば、それも使う/使わない
        ただし、to 方向だけは除く
        */
        let next_from_parent = prefix[i] * suffix[i + 1] % m;

        dfs_all(to, v, next_from_parent, graph, m, dp, ans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_graph(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut graph = vec![Vec::new(); n];
        for &(a, b) in edges {
            let a = a - 1;
            let b = b - 1;
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    }

    #[test]
    fn sample_01() {
        let n = 3;
        let m = 100;
        let edges = vec![(1, 2), (2, 3)];

        let graph = build_graph(n, &edges);

        assert_eq!(resolve(&graph, m), vec![3, 4, 3]);
    }

    #[test]
    fn sample_02() {
        let n = 4;
        let m = 100;
        let edges = vec![(1, 2), (1, 3), (1, 4)];

        let graph = build_graph(n, &edges);

        assert_eq!(resolve(&graph, m), vec![8, 5, 5, 5]);
    }

    #[test]
    fn sample_03() {
        let n = 1;
        let m = 100;
        let edges = vec![];

        let graph = build_graph(n, &edges);

        assert_eq!(resolve(&graph, m), vec![1]);
    }

    #[test]
    fn path_4() {
        let n = 4;
        let m = 1_000_000_007;
        let edges = vec![(1, 2), (2, 3), (3, 4)];

        let graph = build_graph(n, &edges);

        assert_eq!(resolve(&graph, m), vec![4, 6, 6, 4]);
    }

    #[test]
    fn star_5() {
        let n = 5;
        let m = 1_000_000_007;
        let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5)];

        let graph = build_graph(n, &edges);

        assert_eq!(resolve(&graph, m), vec![16, 9, 9, 9, 9]);
    }

    #[test]
    fn modulo() {
        let n = 5;
        let m = 7;
        let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5)];

        let graph = build_graph(n, &edges);

        assert_eq!(resolve(&graph, m), vec![2, 2, 2, 2, 2]);
    }
}
