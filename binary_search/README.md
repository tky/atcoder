# 二分探索の勉強

「1問 = `src/bin/` 内の1ファイル」で進める(パッケージはこのディレクトリ全体で1つ)。
`resolve()` を実装し、`#[cfg(test)]` でサンプルを検証するスタイル。

```sh
cargo test --bin abc231_c_counting_2   # 1問だけテスト
cargo test                             # 全問一括テスト
```

新しい問題を始めるときは `src/bin/<問題名>.rs` を追加するだけでよい。

## ロードマップ

### Step 0: テンプレートを身につける

- めぐる式二分探索(`ok` / `ng` の2ポインタ)を自分で書けるようにする
  - ループ不変条件: `ok` は常に条件を満たす / `ng` は常に満たさない
  - `while ng.abs_diff(ok) > 1` で回し、`mid = (ok + ng) / 2`
- 標準ライブラリの `slice::partition_point` / `binary_search` との対応を理解する

### Step 1: ソート列上の探索(lower_bound / upper_bound)

- [ABC231 C - Counting 2](https://atcoder.jp/contests/abc231/tasks/abc231_c)
  — lower_bound そのもの
- [ABC077 C - Snuke Festival](https://atcoder.jp/contests/abc077/tasks/arc084_a)
  — 中段を固定して上下を二分探索
- [ABC212 C - Min Difference](https://atcoder.jp/contests/abc212/tasks/abc212_c)
  — 一番近い値を探す(境界の両隣を見る)
- [ABC248 D - Range Count Query](https://atcoder.jp/contests/abc248/tasks/abc248_d)
  — 区間内の個数 = upper_bound - lower_bound
- [ABC143 D - Triangles](https://atcoder.jp/contests/abc143/tasks/abc143_d)
  — 2つ固定して3つ目の範囲を二分探索
- [ABC172 C - Tsundoku](https://atcoder.jp/contests/abc172/tasks/abc172_c)
  — 累積和の上で二分探索
- [ABC353 C - Sigma Problem](https://atcoder.jp/contests/abc353/tasks/abc353_c)
  — 条件を満たすペア数のカウント
- [ABC330 C - Minimize Abs 2](https://atcoder.jp/contests/abc330/tasks/abc330_c)
  — 1変数固定 + 最近値探索(境界の両隣)

### Step 1.5: 言い換えてから数える(ABC353 Cと同レベル帯)

- [ABC166 E - This Message Will Self-Destruct in 5s](https://atcoder.jp/contests/abc166/tasks/abc166_e)
  — 条件式をi側とj側に分離して一致カウント
- [ABC353 D - Another Sigma Problem](https://atcoder.jp/contests/abc353/tasks/abc353_d)
  — 寄与テクの続編 + mod計算
- [ABC229 D - Longest X](https://atcoder.jp/contests/abc229/tasks/abc229_d)
  — 累積和 + 単調性で二分探索(Step 2への橋渡し)

### Step 2: 答えで二分探索(判定問題への言い換え)

- [ABC146 C - Buy an Integer](https://atcoder.jp/contests/abc146/tasks/abc146_c)
  — 「xが買えるか?」の単調性
- [ABC174 E - Logs](https://atcoder.jp/contests/abc174/tasks/abc174_e)
  — 最小の最大値(答えを決め打ちして判定)

### Step 2.5: 答えで二分探索の練習

- [ABC063 D - Widespread](https://atcoder.jp/contests/abc063/tasks/arc075_b)
  — 判定問題の設計練習(全体攻撃分を先に差し引く)

### Step 3: 応用(二分探索 + α)

- [ABC149 E - Handshake](https://atcoder.jp/contests/abc149/tasks/abc149_e)
  — 答えの二分探索 + 累積和
- [ABC023 D - 射撃王](https://atcoder.jp/contests/abc023/tasks/abc023_d)
  — 判定問題 + 貪欲

### Step 4: 実数の二分探索

- [ABC034 D - 食塩水](https://atcoder.jp/contests/abc034/tasks/abc034_d)
  — 回数固定ループ(例: 100回)で収束させる

## メモ

- 「単調性があるか?」を最初に確認するのが全て
- 半開区間 `[ok, ng)` の向きは問題ごとに逆になる(最小化/最大化)
- オーバーフロー注意: `(ok + ng) / 2` は usize なら基本OK、i64の負数域では要注意
