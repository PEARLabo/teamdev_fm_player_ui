const { invoke } = window.__TAURI__.tauri;

// ファイル選択ダイアログを開く関数
function openFile() {
    const fileInput = document.getElementById('fileInput');
    fileInput.click(); // ユーザーがファイルを選択するためのダイアログを開く
}

// ファイルが選択されたときのイベントリスナー
document.getElementById('fileInput').addEventListener('change', async (event) => {
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
                hideWarningMessage();
            } else {
                hideSendButton();
                displayWarningMessage();
            }
        } catch (error) {
            console.error(`Error invoking read_file: ${error}`);  // エラーログ
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
    sendButton.style.display = 'block';
}

// 送信ボタンを非表示にする関数
function hideSendButton() {
    const sendButton = document.getElementById('sendButton');
    sendButton.style.display = 'none';
}

// 警告メッセージを表示する関数
function displayWarningMessage() {
    const warningMessage = document.getElementById('warningMessage');
    warningMessage.style.display = 'block';
}

// 警告メッセージを非表示にする関数
function hideWarningMessage() {
    const warningMessage = document.getElementById('warningMessage');
    warningMessage.style.display = 'none';
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
        //player.htmlに遷移
        window.location.href = "player.html";
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
