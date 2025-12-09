# reprack

## 概要

競技プログラミング用ライブラリバンドルするためのライブラリ．
[reprol](https://github.com/knrew/reprol)での使用を想定．

ローカルでライブラリを用いて書かれたコードに対し，そのコード内でuseされているライブラリを貼り付けて提出用コードを生成する．

この際，ライブラリのコメントやテストは削除される．

## インストール

```sh
cargo install --git https://github.com/knrew/reprack
```

## 使い方

```sh
reprack --library <library_dir> --name <library_name> <file_name>
```
とすると，<file_name>でuseしているライブラリをファイルの一番下に追加したものを出力する．
`library_name`は省略可．

例
```sh
reprack --library ~/codes/reprol src/main.rs > submission.rs
```

## 注意

- ソースファイルの`use`を検出してバンドルを行う
    - useせずにライブラリを用いることや`*`で一括インポートすることは禁止
- バンドルの対象となるのはライブラリの`src`以下のファイル
- `lib.rs`および`mod.rs`のコードはバンドルされない
- ライブラリ，ソースともに整形(rustfmt)しておくことを推奨する
- ブロックコメント(`/* ~ */`)を含む行はその前後に意味のあるコードがあっても行ごと無視される
    - このためブロックコメントの使用は非推奨
- テスト関数またはテストモジュール以降のコードは無視される
    - 正確には`#[test]`または`#[cfg(test)]`が現れた時点でそれ以降の行は無視される
- ライブラリ内のモジュール参照において相対パス(`super::`)は未対応
    -  絶対パス(`crate::`)を使うこと
