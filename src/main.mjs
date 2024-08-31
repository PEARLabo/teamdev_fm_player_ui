const { open, message } = window.__TAURI__.dialog;
const { invoke } = window.__TAURI__.tauri;
import SequenceMsg from "./sequencer_msg_parser.mjs";
import {
  init_play_state_display,
  update_play_state_display,
} from "./play_state.mjs";
// import { updatePianoRoll, drawPianoRoll } from "./player.mjs";
import PianoRoll from "./player_current.mjs";
let playbackListenerId = null;
let portName = "/dev/pts/4"; //デフォルトのシリアルポート名

// Tauri関数名を指定
const tauriFunctionName = "send_file_size"; // 本番用
//let tauriFunctionName = 'send_file_test'; // テスト用
let piano_roll;
window.__TAURI__.event.listen("sequencer-msg", (data) => {
  // console.log(data)
  const parsed = new SequenceMsg(data.payload);
  if (parsed.is_ignore_msg()) {
    update_play_state_display(parsed);
    if (piano_roll) {
      piano_roll.updatePianoRoll(parsed);
    }
  }
});

window.onload = () => {
  init_play_state_display();
  // Fileを開くイベント(ダイアログから取得したパスをバックエンドへ送る)
  document.getElementById("fileOpen").onclick = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Midi",
          extensions: ["mid"],
        },
        {
          name: "All",
          extensions: ["*"],
        },
      ],
    });
    if (selected) {
      const is_midi = await invoke("open_file", { path: selected });
      const fname = selected.split("/").at(-1);
      if (is_midi) {
        document.getElementById("fname-display").innerHTML = fname;
        displaySendButton();
      } else {
        // invalid file format
        message(
          `Invalid File Format. "${fname}" is not MIDI File Format.\nFullPath: ${selected}`,
          { title: "Error", type: "error" },
        );
      }
    }
  };
  document.getElementById("swichPlayerBtn").onclick = switchPlayer;
  document.getElementById("swichMainBtn").onclick = switchMain;
  // シリアルポート設定ボタンのクリックイベントリスナーを追加
  document.getElementById("setSerialPortButton").onclick = async () => {
    const serialPortInput = document.getElementById("serialPortInput").value;
    if (serialPortInput) {
      portName = serialPortInput; // ポート名を更新
      await invoke("set_serial_port", { portName: serialPortInput });
      console.log(`Serial port set to: ${portName}`); // デバッグ用ログ
    } else {
      console.error("Invalid serial port input.");
    }
  };

  // Disconnectボタンのクリックイベントリスナーを追加
  document.getElementById("disconnectButton").onclick = async () => {
    try {
      // Rust側のdisconnect関数を呼び出す
      await invoke("disconnect_serial_port");
      console.log("Serial port disconnected successfully");
      const consoleArea = document.getElementById("console");
      consoleArea.value += "Serial port disconnected successfully\n";
    } catch (error) {
      console.error(`Error during disconnect: ${error}`);
      const consoleArea = document.getElementById("console");
      consoleArea.value += `Error during disconnect: ${error}\n`;
    }
  };
  // 送信ボタンがクリックされたときのイベントリスナー
  document.getElementById("sendButton").onclick = async () => {
    try {
      await invoke(tauriFunctionName); // Rust側のsend_file_sizeコマンドを呼び出し
    } catch (error) {
      message(`Error sending file size: ${error}`, {
        title: "Error",
        type: "error",
      });
      console.error(`Error sending file size: ${error}`); // エラーログ

      // // #console textarea 内に送信失敗メッセージを表示
      // const consoleArea = document.getElementById('console');
      // // 自動で一番下までスクロール
      // consoleArea.scrollTop = consoleArea.scrollHeight;
      // // consoleArea 内にエラーメッセージを追加
      // consoleArea.value += 'Error sending file size\n';
    }
  };
};

// div.playerを表示し、div.mainを非表示にする関数
function switchPlayer() {
  const main = document.getElementById("main");
  const player = document.getElementById("player");
  main.style.display = "none";
  player.style.display = "block";
  // ピアノロールの描画
  piano_roll = new PianoRoll("pianoRoll");
  piano_roll.draw();
  // #console をクリア
  // document.getElementById("console").value = null;
  // document.getElementById("playerConsole").value = null;
}

// div.mainを表示し、div.playerを非表示にする関数
function switchMain() {
  const main = document.getElementById("main");
  const player = document.getElementById("player");
  main.style.display = "block";
  player.style.display = "none";
  // #console をクリア
  document.getElementById("console").value = null;

  // イベントリスナを解除
  if (playbackListenerId) {
    playbackListenerId();
    playbackListenerId = null;
  }
}

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
  const fileSizeElemnt = document.getElementById("fileSize");
  fileSizeElemnt.textContent = `ファイルサイズ: ${size} byte`;
}

// 送信ボタンを表示する関数
function displaySendButton() {
  const sendButton = document.getElementById("sendButton");
  // disabled を false にする
  sendButton.disabled = false;
  // cursor not-allowed を auto に
  sendButton.style.cursor = "default";
  // 背景色を青に
  sendButton.style.backgroundColor = "#333";
}

// 送信ボタンを無効にする関数
function disableSendButton() {
  const sendButton = document.getElementById("sendButton");
  sendButton.disabled = true;
  sendButton.style.cursor = "not-allowed";
  sendButton.style.backgroundColor = "#aaa";
}

// 警告メッセージを表示する関数
function showWarningMessage() {
  // html の #console textarea 内にエラーメッセージを表示
  const consoleArea = document.getElementById("console");
  // 自動で一番下までスクロール
  consoleArea.scrollTop = consoleArea.scrollHeight;
  // consoleArea 内にエラーメッセージを追加
  consoleArea.value += "Error: MIDI形式のファイルを選択してください\n";
}
