# From ChatGPT

`process_event`関数のRustコードをHTMLページに情報を表示するようにJavaScriptを使用して統合する方法を説明します。Tauriを使用しているため、RustとJavaScript間でのデータ通信を行う必要があります。

以下は、RustとJavaScript間でデータをやり取りする一般的なステップです：

### ステップ 1: Rust側の準備

Tauriは`@tauri-apps/api` モジュールを使用して、フロントエンドとバックエンド間の通信を容易にします。まず、必要なデータをフロントエンドに送信するためのエンドポイントをRustで設定します。

`process_event` 関数を変更して、データを返すようにします。例えば、再生情報を受信したらそのデータをJavaScriptに送ることができます。

```rust
#[tauri::command]
async fn process_event(port_name: String) -> Result<String, String> {
    // 既存のコード
    // データを受信した際に、データをJSON形式の文字列に変換して返す
    Ok(format!("{{\"tempo\": {}, \"chanel\": {}, \"key\": {}, \"velocity\": {}}}", tempo, chanel, key, velocity))
}
```

### ステップ 2: JavaScript側の準備

フロントエンドでTauri APIを使用してRustコマンドを呼び出し、受信したデータをHTMLに表示します。ここで`invoke`メソッドを使用してRustの関数を呼び出し、結果を取得します。

```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Playback Information</title>
<script src="https://cdn.jsdelivr.net/npm/@tauri-apps/api/dist/bundle.js"></script>
</head>
<body>
    <div id="info">
        <p id="tempo">Tempo: </p>
        <p id="chanel">Chanel: </p>
        <p id="key">Key: </p>
        <p id="velocity">Velocity: </p>
    </div>
    <script>
        async function updatePlaybackInfo() {
            try {
                const response = await window.__TAURI__.invoke('process_event', { port_name: 'COM3' });
                const data = JSON.parse(response);
                document.getElementById('tempo').textContent = `Tempo: ${data.tempo}`;
                document.getElementById('chanel').textContent = `Chanel: ${data.chanel}`;
                document.getElementById('key').textContent = `Key: ${data.key}`;
                document.getElementById('velocity').textContent = `Velocity: ${data.velocity}`;
            } catch (error) {
                console.error('Error updating playback info:', error);
            }
        }

        updatePlaybackInfo(); // 初回ロード時に情報を取得
    </script>
</body>
</html>
```

このHTMLとJavaScriptのコードは、ページがロードされた時に自動的に`process_event`を呼び出し、結果をHTML要素に表示します。この例では、バックエンドから受け取った再生情報を表示しています。

これで、RustのバックエンドとJavaScriptのフロントエンド間で情報を受け渡し、HTMLページに動的に表示する基本的な流れが整いました。