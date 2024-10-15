# library-bundler
## 概要
自分用競技プログラミング用ライブラリバンドルするためのライブラリ．

ローカルでは普通のライブラリのようにインポートしてコードを書き，このバンドラによってuseされているライブラリを貼り付けて提出用のコードとする．

この際，コメントやテストは削除する．

## install
このリポジトリをクローンして，
```sh
cargo install --path .
```

## How to use
```sh
library-bundler -l <library_dir> -n <library_name> src/main.rs
```
とすると，`main.rs`でuseしているライブラリをファイルの一番下に追加したものを出力する．

`library_name`は省略可．

```sh
library-bundler -l <library_dir> src/main.rs > submission.rs
```
などとして利用する．

## 注意
ライブラリが不十分なので，ライブラリ側で以下に注意する
- `src`の直下にしかライブラリを置かないようにする
    - `src/poyo/mod.rs`みたいなの禁止
- コメントは`//`または`///`のみとする
    - `/* ~ */`はバグる可能性あり
- 行途中から始まるコメントは削除されない
    - `let x  = 5; // poyopoyo`みたいなやつ
- testは`#[cfg(test)]`からはじめ，test以降にコードを置かないようにする
    - `#[cfg(test)]`以降は無視する設定になっている
- `mylibrary/*`のように`*`でライブラリすべてをインポートしようとしないようにする
