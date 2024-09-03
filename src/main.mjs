
const { invoke } = window.__TAURI__.tauri;
import {open,warningDialog} from "./js/dialog.mjs"
import PianoRoll from "./js/pianoroll.mjs";
import console_override from "./js/console.mjs";
import {
  init_play_state_display,
  update_play_state_display,
} from "./js/play_state.mjs";
import SequenceMsg from "./js/sequencer_msg_parser.mjs";
let playbackListenerId = null;
let portName = "/dev/pts/4"; //デフォルトのシリアルポート名

// Tauri関数名を指定
const tauriFunctionName = "send_file_size"; // 本番用
//let tauriFunctionName = 'send_file_test'; // テスト用
let piano_roll;

// TODO: いい感じに実装を移す
window.__TAURI__.event.listen("sequencer-msg", (data) => {
  const parsed = new SequenceMsg(data.payload);
  if (!parsed.is_ignore_msg()) {
    update_play_state_display(parsed);
    if (piano_roll) {
      piano_roll.updatePianoRoll(parsed);
    }
  }
});

window.onload = () => {
  init_play_state_display();
  console_override("console");
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
        enableSendButton();
      } else {
        // invalid file format
        warningDialog(
          `Invalid File Format. "${fname}" is not MIDI File Format.\nFullPath: ${selected}`,
        );
      }
    }
  };
  document.getElementById("swichPlayerBtn").onclick =
  document.getElementById("swichMainBtn").onclick = togglePlayer;
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
    } catch (error) {
      console.error(`Failed during disconnect: ${error}`);
    }
  };
  // 送信ボタンがクリックされたときのイベントリスナー
  document.getElementById("sendButton").onclick = async () => {
    try {
      await invoke("send_midi_file"); // Rust側のsend_file_sizeコマンドを呼び出し
    } catch (error) {
      warningDialog(`sending file size: ${error}`);
    }
  };
};

// playerの表示切替
function togglePlayer() {
  const main = document.getElementById("control-panel");
  const player = document.getElementById("player");
  const is_current_player = main.classList.contains("hide");
  main.classList.toggle("hide");
  player.classList.toggle("hide");
  // ピアノロールの描画
  if (is_current_player) {
    // 描画停止
    // TODO: Implement
  } else {
    if(!piano_roll) {
      // 初期化
      piano_roll = new PianoRoll("pianoRoll");
      piano_roll.draw();
    } else {
      // 描画再開
      // TODO: Implement
    }
  }
}


// 送信ボタンを表示する関数
function enableSendButton() {
  const sendButton = document.getElementById("sendButton");
  // disabled を false にする
  sendButton.disabled = false;
}

// 送信ボタンを無効にする関数
function disableSendButton() {
  const sendButton = document.getElementById("sendButton");
  sendButton.disabled = true;
}
