# library-bundler
## 概要
自分用競技プログラミング用ライブラリバンドルするためのライブラリ．

ローカルでは普通のライブラリのようにインポートしてコードを書き，このバンドラによってuseされているライブラリを貼り付けて提出用のコードとする．

この際，コメントやテストは削除する．

## install
```sh
cargo install --git https://github.com/knrew/library-bundler
```
または
```sh
git clone https://github.com/knrew/library-bundler
cd library-bundler
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
- `src`の直下のライブラリのみ有効
    - `src/poyo/mod.rs`みたいなやつは非対応
- ブロックコメント(`/* ~ */`)を使う場合はその行にコメント出ないコードを書いてはいけない
    - `let x = 4; /* poyo */`みたいなやつは非対応
- テストは`#[cfg(test)]`からはじめる．テストコード以降に通常のコードを置かないようにする
    - `#[cfg(test)]`以降のコードはすべて無視する設定になっている
- ライブラリはuseをつかってインポートすること
- `mylibrary/*`のように`*`でライブラリすべてをインポートするコードには非対応
