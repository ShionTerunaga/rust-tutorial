GPT 等を用いてまとめたもの

# Rust における所有権とは

プログラムのメモリを安全かつ効率的に管理する仕組み。

# 所有権のルール

1. 値は必ず 1 つの所有者が持つ
   - 複数人の所有者は存在しない
   - 必要がなくなったら自動的に解放する
1. 所有者がスコープから外れたとき、データは解放される
   - スコープから抜けた時点で自動的にメモリを解放される
1. ムーブされる
   - 所有者が新しい変数に代入された場合は新しい変数にムーブされるので元の変数は使えなくなる
   - 元の変数はメモリを自動で解放されるためであると考察する
   - 別の変数に代入されることをムーブという

> [!NOTE]
> スコープとは変数や関数などの名前が有効で使える範囲のことを言う

GPT のサンプル例をそもままコピペ

```rs
{
    let s = String::from("hello"); // sが"hello"を所有
    let t = s; // 所有権がsからtにムーブ
    println!("{}", s); // エラー：sはもう使えない
} // tがスコープを抜けると、"hello"が解放される
```

# 所有権はなぜ必要なのか？

1. 二重解放のバグ
   - 手動でメモリを解放する言語(C や C++)では同じデータを 2 回解放するとエラーが発生する
   - Rust ではデータの所持者は 1 人か
1. メモリリーク
   - 使用が終わったメモリを解放しないとメモリは消費されてしまう
   - スコープを抜けると自動で解放するのでリークの心配はない
1. 不正なメモリアクセス
   - 解放されたデータにアクセスしようとするとクラッシュや不具合が起こってしまう
   - 不正なメモリアクセスをしようとするとコンパイル時に防止してくれる

# 「借用」「参照」の関係

## 借用

- 所有権を移さずに一時的にデータを借りる仕組み
- イミュータブル借用とミュータブル借用がある

```rs
let s = String::from("hello");

// イミュータブル借用
let len = calculate_length(&s); // sの所有権を渡さず、参照だけ渡す
println!("{}", s); // sはまだ使える

fn calculate_length(s: &String) -> usize {
    s.len() // 参照を利用してデータにアクセス
}

// ミュータブル借用
let mut s = String::from("hello");
change(&mut s); // ミュータブル参照で変更可能
println!("{}", s); // "hello world" と表示

fn change(s: &mut String) {
    s.push_str(" world");
}
```

## 参照

- データのアドレスを参照として渡す
- 安全性をコンパイル時にチェック