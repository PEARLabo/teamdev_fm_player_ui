# 開発の際に確認すること
必要に応じて本番用とテスト用を切り替える
テスト用はフロントエンド開発用。
## src-tauri/src/main.rs
```rust
637 | //send_file_size, // 本番用
638 | send_file_test  // テスト用
```

## src/main.js
```js
6 | //let tauriFunctionName = tauriFunctionName; // 本番用
7 | let tauriFunctionName = 'send_file_test'; // テスト用
```

# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

