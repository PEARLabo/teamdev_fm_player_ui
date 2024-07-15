const { invoke } = window.__TAURI__.tauri;

// div.playerを表示し、div.mainを非表示にする関数
function switchPlayer() {
    const main = document.getElementById('main');
    const player = document.getElementById('player');
    main.style.display = 'none';
    player.style.display = 'block';
    // ピアノロールの描画
    drawPianoRoll();
    // #console をクリア
    document.getElementById('console').value = null;
    document.getElementById('playerConsole').value = null;
}

// div.mainを表示し、div.playerを非表示にする関数
function switchMain() {
    const main = document.getElementById('main');
    const player = document.getElementById('player');
    main.style.display = 'block';
    player.style.display = 'none';
    // #console をクリア
    document.getElementById('console').value = null;
    document.getElementById('playerConsole').value = null;
}

// ファイル選択ダイアログを開く関数
function openFile() {
    const fileInput = document.getElementById('fileInput');
    fileInput.click(); // ユーザーがファイルを選択するためのダイアログを開く
}

// ファイルが選択されたときのイベントリスナー
document.getElementById('fileInput').addEventListener('change', async (event) => {
    // #console をクリア
    const consoleArea = document.getElementById('console');
    consoleArea.value = null;

    const fileList = event.target.files; // 選択されたファイルリストを取得
    if (fileList.length > 0) {
        const file = fileList[0];
        const contents = await readFileAsArrayBuffer(file); // ファイルの内容を読み込み

        try {
            const fileInfo = await invoke('read_file', { contents: Array.from(new Uint8Array(contents)) }); // Rust側のread_fileコマンドを呼び出し
            console.log(`File info: ${JSON.stringify(fileInfo)}`);  // デバッグ用ログ

            // ファイルサイズを表示
            displayFileSize(fileInfo.size);

            // ファイルの形式がMIDI形式か判定
            if (fileInfo.is_midi) {
                displaySendButton();
            } else {
                showWarningMessage();
            }
        } catch (error) {
            console.error(`Error invoking read_file: ${error}`);  // エラーログ
            // html の #console textarea 内にエラーメッセージを表示
            const consoleArea = document.getElementById('console');
            // 自動で一番下までスクロール
            consoleArea.scrollTop = consoleArea.scrollHeight;
            // consoleArea 内にエラーメッセージを追加( ${error} も表示)
            consoleArea.value += `Error invoking read_file: ${error}\n`;
        }
    }
});

// ファイルの内容をArrayBufferとして読み込む関数
async function readFileAsArrayBuffer(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onloadend = () => resolve(reader.result);
        reader.onerror = reject;
        reader.readAsArrayBuffer(file);
    });
}

// ファイルサイズを表示する関数
function displayFileSize(size) {
    const fileSizeElemnt = document.getElementById('fileSize');
    fileSizeElemnt.textContent = `ファイルサイズ: ${size} byte`;
}

// 送信ボタンを表示する関数
function displaySendButton() {
    const sendButton = document.getElementById('sendButton');
    // disabled を false にする
    sendButton.disabled = false;
    // cursor not-allowed を auto に
    sendButton.style.cursor = 'default';
}

// 警告メッセージを表示する関数
function showWarningMessage() {
    // html の #console textarea 内にエラーメッセージを表示
    const consoleArea = document.getElementById('console');
    // 自動で一番下までスクロール
    consoleArea.scrollTop = consoleArea.scrollHeight;
    // consoleArea 内にエラーメッセージを追加
    consoleArea.value += 'Error: MIDI形式のファイルを選択してください\n';
}

// 送信ボタンがクリックされたときのイベントリスナー
document.getElementById('sendButton').addEventListener('click', async () => {
    const fileInput = document.getElementById('fileInput');
    const file = fileInput.files[0];
    const contents = await readFileAsArrayBuffer(file);
    const portName = "COM3"; // シリアルポート名を指定（適宜変更）
    try {
        await invoke('send_file_size', { contents: Array.from(new Uint8Array(contents)), port_name: portName }); // Rust側のsend_file_sizeコマンドを呼び出し
        console.log('Data sent successfully');  // デバッグ用ログ
        //div.mainを非表示にし、div.playerを表示
        switchPlayer();

        // #playerConsole textarea デモ
        const playerConsoleArea = document.getElementById('playerConsole');
        playerConsoleArea.scrollTop = playerConsoleArea.scrollHeight;
        playerConsoleArea.value += 'これはさすがにひょうじされるよね\n';

        /* player.html での挙動 */
        // Rust側のprocess_eventコマンド呼び出し
        //await invoke('process_event', { port_name: portName });
        // Tauriのイベントリスニングを設定
        window.__TAURI__.event.listen('playback_info', (event) => {
            const data = JSON.parse(event.payload);  // 受信したデータをパース
            const playerConsoleArea = document.getElementById('console');
            playerConsoleArea.value += `Playback Data: ${data.error}\n`;  // エラーメッセージを追加
            // 自動で一番下までスクロール
            playerConsoleArea.scrollTop = consoleArea.scrollHeight;
        });

    } catch (error) {
        console.error(`Error sending file size: ${error}`);  // エラーログ
        // #console textarea 内に送信失敗メッセージを表示
        const consoleArea = document.getElementById('console');
        // 自動で一番下までスクロール
        consoleArea.scrollTop = consoleArea.scrollHeight;
        // consoleArea 内にエラーメッセージを追加
        consoleArea.value += 'Error sending file size\n';

        /*
            以下開発用
        */
        //div.mainを非表示にし、div.playerを表示
        switchPlayer();

        // #console textarea 内に送信失敗メッセージを表示
        const playerConsoleArea = document.getElementById('playerConsole');
        // 自動で一番下までスクロール
        playerConsoleArea.scrollTop = playerConsoleArea.scrollHeight;
        // playerConsoleArea 内にエラーメッセージを追加
        playerConsoleArea.value += '[MIDI送信完了(仮)]これはさすがにひょうじされるよね\n';

        // Rust側のprocess_eventコマンド呼び出し
        //await invoke('process_event', { port_name: portName });
        // Tauriのイベントリスニングを設定
        window.__TAURI__.event.listen('playback_info', (event) => {
            const data = JSON.parse(event.payload);  // 受信したデータをパース
            const playerConsoleArea = document.getElementById('console');
            playerConsoleArea.value += `Playback Data: ${data.error}\n`;  // エラーメッセージを追加
            // 自動で一番下までスクロール
            playerConsoleArea.scrollTop = consoleArea.scrollHeight;
        });

        /*
            開発用ここまで
        */
    }
});
