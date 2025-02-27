# FMプレイヤー

## 依存ツール

Linux(Debianベース)向けの依存ライブラリの導入について記載する

### Rust

一応公式ページを参照すること。

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### tauri

```sh
cargo install tauri-cli --version 1.6.0 --locked
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 実行方法

1. リポジトリのクローン
2. クローンしたディレクトリへ移動
3. ビルドと実行:`cargo run -r`

物理的なシリアルポートが存在する場合は補完できる。
仮想シリアルポートの場合は、補完機能が動かないため、信じて直打ちする。
入力後、「接続」し、MIDIファイルを選択、送信する。
送信完了後、シーケンサが音声再生を開始する。

Player画面へ移ると鍵盤上の表示ができる。

## CLIモード

cliブランチにCLIモードが存在します。

個人的にはこちらの見た目のほうが好きなので、ぜひ。  
使用方法は、`cargo run -r -- --help`でヘルプを参照してほしい。
