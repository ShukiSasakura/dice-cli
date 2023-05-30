# dice-cli

コマンドラインで実行できるダイスロールシミュレータ

## 環境構築(整備中)

Cargo と WebAssembly ランタイムがインストールされている必要がある．
また，Rust ツールチェインの wasm32-wasi ターゲットをインストールする必要がある．

## 使い方

まず，このリポジトリをクローンする．

```sh
git clone https://github.com/ShukiSasakura/dice-cli.git
```

次に，ディレクトリに入り，ビルドを行う．

```sh
cd dice-cli
cargo build --target wasm32-wasi
```

WebAssembly ランタイムでシミュレータを実行する．

以下ではWasmEdgeで使用する場合を示す．

```sh
wasmedge target/wasm32-wasi/debug/dice.wasm
```

起動した後，ダイスの個数と面の数をダイスコードで入力する（例：2d6）．
指定した個数と同じ回数ランダムな数が表示され，最後に合計値が表示される．

### 実行例

```
please input by dice notation  e.g. 2d6
4d6
random number 0 = 2
random number 1 = 2
random number 2 = 6
random number 3 = 3
sum = 13
```
