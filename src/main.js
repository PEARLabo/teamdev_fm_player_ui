const { invoke } = window.__TAURI__.tauri;
let playbackListenerId = null;

// Tauri関数名を指定
//let tauriFunctionName = tauriFunctionName; // 本番用
let tauriFunctionName = 'send_file_test'; // テスト用

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

    // イベントリスナを設定
    if (!playbackListenerId) {
        window.__TAURI__.event.listen('playback_info', (event) => {
            const data = event.payload;
            console.log(data);
            // ピアノロールアップデート
            updatePianoRoll(data);
        }).then((unlisten) => {
            playbackListenerId = unlisten;
        });
    }
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

    // イベントリスナを解除
    if (playbackListenerId) {
        playbackListenerId();
        playbackListenerId = null;
    }
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
        const contents = await readFileAsArrayBuffer(file);


        try {
            //const contents = await readFileAsArrayBuffer(file); // ファイルの内容を読み込み
            const fileInfo = await invoke('read_file', { contents: Array.from(new Uint8Array(contents)) }); // Rust側のread_fileコマンドを呼び出し
            console.log(`File info: ${JSON.stringify(fileInfo)}`);  // デバッグ用ログ

            // ファイルサイズを表示
            displayFileSize(fileInfo.size);

            // ファイルの形式がMIDI形式か判定
            if (fileInfo.is_midi) {
                displaySendButton();
            } else {
                disableSendButton();
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
    // 背景色を青に
    sendButton.style.backgroundColor = '#333';
}

// 送信ボタンを無効にする関数
function disableSendButton() {
    const sendButton = document.getElementById('sendButton');
    sendButton.disabled = true;
    sendButton.style.cursor = 'not-allowed';
    sendButton.style.backgroundColor = '#aaa';
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
    const portName = "/dev/pts/2"; // シリアルポート名を指定（適宜変更）
    console.log('Data send clicked');
    try {
        await invoke(tauriFunctionName, { contents: Array.from(new Uint8Array(contents)), portName: portName }); // Rust側のsend_file_sizeコマンドを呼び出し
        console.log('Data sent successfully');  // デバッグ用ログ
        //div.mainを非表示にし、div.playerを表示
        switchPlayer();

        // #playerConsole textarea デモ
        const playerConsoleArea = document.getElementById('playerConsole');
        playerConsoleArea.scrollTop = playerConsoleArea.scrollHeight;
        playerConsoleArea.value += '==Switched Player==\n';
    } catch (error) {
        console.error(`Error sending file size: ${error}`);  // エラーログ
        // #console textarea 内に送信失敗メッセージを表示
        const consoleArea = document.getElementById('console');
        // 自動で一番下までスクロール
        consoleArea.scrollTop = consoleArea.scrollHeight;
        // consoleArea 内にエラーメッセージを追加
        consoleArea.value += 'Error sending file size\n';
    }
});
