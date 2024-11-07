# library-bundler
## 概要
自分用競技プログラミング用ライブラリバンドルするためのライブラリ．

[reprol](https://github.com/knrew/reprol)を用いることを想定している．

ローカルでは普通のライブラリのようにインポートしてコードを書き，このバンドラによってuseされているライブラリを貼り付けて提出用のコードとする．

この際，コメントやテストは削除されるようにする．

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
library-bundlerはまだ不完全なので，バンドルするライブラリ作成において以下に注意する
- 対象となるライブライファイルおよびソースファイルはフォーマット(rustfmt)しておく
    - 変なスペースとかあるとうまくできない可能性あり
- ライブラリルートの`/src`直下およびその1個下の階層のみ有効
    - `src/poyo.rs`，`src/poyo/piyo.rs`はOK．`src/poyo/piyo/puyo.rs`はNG
- `lib.rs`および`mod.rs`のコードはバンドルされない
- ブロックコメント(`/* ~ */`)は非推奨
    - ブロックコメントを含む行は行ごと無視される
        - `let x = 4; /* poyo */`のような行はバンドルされない
- テスト関数またはテストモジュール以降のコードは無視される
    - 正確には`#[test]`または`#[cfg(test)]`が現れた時点でそれ以降の行は無視される
- ライブラリはuseして使用すること
    - `let dsu = reprol::dsu::Dsu::new(n);`のようにuseせずにライブラリを用いてはいけない
- `*`を用いてインポートしてはいけない
- `super::`(相対パス)ではなく`crate::`(絶対パス)を使う
- モジュールはすべてpubとしてバンドルされることに注意
