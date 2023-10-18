# leptos の簡単なサンプル。クライアントサイドで 足し算 を計算し描画する。

### 環境構築

参考： https://leptos-rs.github.io/leptos/02_getting_started.html

rustup は最新にアップデートしておく

```sh
rustup show
rustup update
```

コンパイルターゲットに wasm を追加しておくのを忘れない。

```sh
rustup target add wasm32-unknown-unknown
rustup target list --installed
```

プロジェクトの作成

```sh
cargo install trunk
cargo init leptos-tutorial
cd leptos-tutorial
cargo add leptos --features=csr,nightly
```

ビルド

コードを変更して保存すると自動でビルドされて、ブラウザが再レンダリングされる

```sh
trunk serve --open
```

### ビルド成果物

`<project-root>/dist/` 配下に生成される

```sh
$ tree ./dist
./dist
├── index.html
├── leptos-tutorial-772767697aef62e8.js
└── leptos-tutorial-772767697aef62e8_bg.wasm

1 directory, 3 files
```

### 生成された wasm バイナリを wat に変換する

```sh
wasm2wat ./dist/leptos-tutorial-772767697aef62e8_bg.wasm -o ./dist/leptos-tutorial.wat
```

`leptos-tutorial-772767697aef62e8_bg.wasm` のバイナリのファイル名の部分は生成されたバイナリごとに異なるので適宜修正してください

参考： https://github.com/WebAssembly/wabt
