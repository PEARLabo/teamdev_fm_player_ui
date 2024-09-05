import console_override from "./js/console.mjs";
import { open, warningDialog } from "./js/dialog.mjs";
import PeriodicTask from "./js/periodic.mjs";
import PianoRoll from "./js/pianoroll.mjs";
import PerformanceMonitor from "./js/performMonitor.mjs";
import SequenceMsg from "./js/seqMsgParser.mjs";
import { BackEnd } from "./js/backendProcess.mjs";
let piano_roll;
let performance_monitor;
let periodic_task_manager;

window.onload = () => {
    // バックエンドからのイベントに対する動作定義
    BackEnd.onseq_msg = (data) => {
        const parsed = new SequenceMsg(data.payload);
        if (!parsed.is_ignore_msg()) {
            if (performance_monitor) {
                performance_monitor.update(parsed);
            }
            if (piano_roll) {
                piano_roll.updatePianoRoll(parsed);
            }
        }
    };
    BackEnd.onerror = (msg) => {
        warningDialog(msg);
    };
    // 描画に関する初期化
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
    // コンソールの機能をオーバーライド
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
        if (!selected) return;
        // ファイルオープン/ファイル形式確認
        BackEnd.file_open(selected)
            .then(() => {
                // receipt
                const fname = selected.split(/\/|\\/).at(-1);
                // 表示の変更
                document.getElementById("fname-display").innerHTML = fname;
                document.getElementById(
                    "midi-file-open-container",
                ).dataset.tooltip = fname;
                enableSendButton();
            })
            .catch((err) => {
                // failed or reject
                warningDialog(err);
            });
    };
    document.getElementById("swichPlayerBtn").onclick = document.getElementById(
        "swichMainBtn",
    ).onclick = togglePlayer;
    // シリアルポート設定ボタンのクリックイベントリスナーを追加
    document.getElementById("setSerialPortButton").onclick = async () => {
        const serialPortInput =
            document.getElementById("serialPortInput").value;
        if (serialPortInput) {
            // Note: This function does not return success or failure.
            BackEnd.serialport.open(serialPortInput).catch((err) => {
                warningDialog(err);
            });
            // console.log(`Success Open Serial port : ${serialPortInput}`)
        } else {
            console.error("Serial Port is undefined.");
        }
    };
    // 利用可能なシリアルポートのサジェストを作成
    document.getElementById("serialPortInput").onfocus = async () => {
        BackEnd.serialport.get_available_ports().then((ports) => {
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
        // Note: This function does not return success or failure.
        BackEnd.serialport.close().catch((err) => {
            warningDialog(err);
        });
    };
    // 送信ボタンがクリックされたときのイベントリスナー
    document.getElementById("sendButton").onclick = async () => {
        BackEnd.send_file().catch((err) => {
            warningDialog(err);
        });
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
