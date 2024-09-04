const { invoke } = window.__TAURI__.tauri;
import console_override from "./js/console.mjs";
import { open, warningDialog } from "./js/dialog.mjs";
import PeriodicTask from "./js/periodic.mjs";
import PianoRoll from "./js/pianoroll.mjs";
import PerformanceMonitor from "./js/performMonitor.mjs";
import SequenceMsg from "./js/seqMsgParser.mjs";

let piano_roll;
let performance_monitor;
let periodic_task_manager;
// TODO: いい感じに実装を移す
window.__TAURI__.event.listen("sequencer-msg", (data) => {
    const parsed = new SequenceMsg(data.payload);
    if (!parsed.is_ignore_msg()) {
        if (performance_monitor) {
            performance_monitor.update(parsed);
        }
        if (piano_roll) {
            piano_roll.updatePianoRoll(parsed);
        }
    }
});

window.onload = () => {
    piano_roll = new PianoRoll("pianoRoll");
    performance_monitor = new PerformanceMonitor("currentPlayState");
    performance_monitor.init();
    // 描画を定期タスクで管理
    periodic_task_manager = new PeriodicTask(
        [
            piano_roll.draw.bind(piano_roll),
            performance_monitor.commit.bind(performance_monitor),
        ],
        piano_roll.init_draw.bind(piano_roll),
    );
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
            try {
                await invoke("open_file", { path: selected });
                // ファイル名(と拡張子)のみを抽出
                const fname = selected.split(/\/|\\/).at(-1);
                // 表示の変更
                document.getElementById("fname-display").innerHTML = fname;
                document.getElementById("midi-file-open-container").dataset.tooltip = fname;
                enableSendButton();
            } catch (err) {
                warningDialog(err);
            }
        }
    };
    document.getElementById("swichPlayerBtn").onclick = document.getElementById(
        "swichMainBtn",
    ).onclick = togglePlayer;
    // シリアルポート設定ボタンのクリックイベントリスナーを追加
    document.getElementById("setSerialPortButton").onclick = async () => {
        const serialPortInput =
            document.getElementById("serialPortInput").value;
        if (serialPortInput) {
            await invoke("set_serial_port", { portName: serialPortInput });
            console.log(`Serial port set to: ${serialPortInput}`); // デバッグ用ログ
        } else {
            console.error("Invalid serial port input.");
        }
    };
    // 利用可能なシリアルポートのサジェストを作成
    document.getElementById("serialPortInput").onfocus = async () => {
        invoke("get_available_serial_ports", {}).then((ports) => {
            if (ports) {
                const datalist = document.getElementById("active-serialports");
                const fragment = document.createDocumentFragment();
                for (const port of ports) {
                    const item = document.createElement("option");
                    item.value = port;
                    fragment.appendChild(item);
                }
                datalist.innerHTML = null;
                datalist.appendChild(fragment);
            }
        });
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
            warningDialog(error);
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
        periodic_task_manager.stop();
        console.log("Move to Control Panel");
    } else {
        periodic_task_manager.start();
        console.log("Move to Visualizer Panel");
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
