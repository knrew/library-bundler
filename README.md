# library-bundler
## 概要
競技プログラミング用ライブラリバンドルするためのライブラリ．
[reprol](https://github.com/knrew/reprol)での使用を想定．

ローカルでライブラリを用いて書かれたコードに対し，そのコード内でuseされているライブラリを貼り付けて提出用コードを生成する．

この際，ライブラリのコメントやテストは削除される．

## インストール
```sh
cargo install --git https://github.com/knrew/library-bundler
```
または
```sh
git clone https://github.com/knrew/library-bundler
cd library-bundler
cargo install --path .
```

## 使い方
```sh
library-bundler -l <library_dir> -n <library_name> <file_name>
```
とすると，<file_name>でuseしているライブラリをファイルの一番下に追加したものを出力する．
`library_name`は省略可．


例
```sh
library-bundler -l ~/codes/reprol src/main.rs > submission.rs
```
などとすれば，`src/main.rs`で書いたコードにライブラリを貼り付けたものが`submission.rs`として出力される．

## 注意
仕様により，ライブラリ作成においては以下に注意する．
- 対象となるライブライファイルおよびソースファイルはフォーマット(rustfmt)しておく
    - 変なスペースとかあるとうまくできない可能性あり
- ライブラリルートの`/src`直下およびその1個下の階層のみ有効
    - `src/poyo.rs`，`src/poyo/piyo.rs`はOK．`src/poyo/piyo/puyo.rs`はNG
- `lib.rs`および`mod.rs`のコードはバンドルされない
- ブロックコメント(`/* ~ */`)は非推奨
    - ブロックコメントを含む行は行ごと無視される
        - `let x = 4; /* poyo */`のような行は行ごと無視されてしまう
- テスト関数またはテストモジュール以降のコードは無視される
    - 正確には`#[test]`または`#[cfg(test)]`が現れた時点でそれ以降の行は無視される
- ライブラリはuseして使用すること
    - `let dsu = reprol::dsu::Dsu::new(n);`のようにuseせずにライブラリを用いてはいけない
- `*`を用いてインポートしてはいけない
- ライブラリ内のモジュール参照は`super::`(相対パス)ではなく`crate::`(絶対パス)を使うこと
- バンドル対象のモジュールはすべてpubとしてバンドルされる
