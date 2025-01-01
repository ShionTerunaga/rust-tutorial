# 借用

借用は主に 2 つある

- 変数を変更せずに利用する不変借用
- 変数を変更できる可変借用がある

## 不変借用

```rs
fn main() {
    let s = String::from("hello"); // 所有権を持つ
    let len = calculate_length(&s); // sを借用する（不変借用）
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // 借用した値を利用
}
```

## 可変借用

```rs
fn main() {
    let mut s = String::from("hello"); // 可変な所有権
    change(&mut s); // sを可変借用
    println!("{}", s); // 変更が反映される
}

fn change(s: &mut String) {
    s.push_str(", world"); // 借用した値を変更
}
```

# 借用のルール

## 同時に複数の可変借用はできない

- 1 つの可変借用のみが許される
- 別の参照があるときに可変借用するとエラーになる

```rs
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s; // 可変借用
    let r2 = &mut s; // 2つ目の可変借用（エラー）
    println!("{}, {}", r1, r2);
}
```

## 同時に不変借用と可変借用は同時に存在しない

```rs
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // 不変借用
    let r2 = &mut s; // 可変借用（エラー）
    println!("{}, {}", r1, r2);
}
```

基本、1 変数の借用は 一度まで

## 借用は有効なスコープ内でのみ使える

```rs
fn main() {
    let mut s = String::from("hello");
    {
        let r = &mut s; // 可変借用のスコープ開始
        r.push_str(", world");
    } // 可変借用のスコープ終了
    println!("{}", s); // 借用が終わったので利用可能
}
```

# 借用を使う理由

1. 所有権を移動させずに値を利用できる
2. メモリの**ダングリング参照**やデータ競合を防ぐ
3. 効率的にプログラムを記述できる

> [!NOTE]
> 「タングリング参照」とは、メモリ上で解放されたデータを参照しようとするバグのこと

## 2 のダングリングについて

Rust ではダングリング参照が発生しないようにコンパイル時にチェックされる

```c
#include <stdio.h>
#include <stdlib.h>

int* create_number() {
    int* num = malloc(sizeof(int));
    *num = 42;
    free(num); // メモリを解放
    return num; // 解放済みメモリへの参照を返す
}

int main() {
    int* ptr = create_number();
    printf("%d\n", *ptr); // ダングリング参照: 未定義動作
    return 0;
}
```

上記コードでは解放した変数を返そうとしている

```rs
fn create_number() -> &i32 {
    let num = 42; // スタック上に値を格納
    &num // エラー: 借用がスコープを超える
}

fn main() {
    // let ptr = create_number(); // コンパイルエラー
}
```

rust では借用がスコープ外になるとコンパイルエラーで警告する

# 2 のデータの競合について

データ競合は複数のスレッドが同じデータを同時に読み書きすることで予期せぬ動作やバグが発生する問題

## 借用ルールとデータ競合の防止

1. 不変借用は何個でも可能
1. 可変借用は同時に 1 つだけ可能
1. 不変借用と可変借用は同時に存在できない
