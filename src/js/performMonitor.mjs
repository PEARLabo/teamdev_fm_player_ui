import SequenceMsg from "./seqMsgParser.mjs";

const PLAYER_HEADER_ITEMS = ["channel", "instrument", "note", "pitch", "Expr"];
const [CHANNEL_INDEX, INSTRUMENT_INDEX, NOTE_INDEX, PITCH_INDEX, Expr_INDEX] = [
    ...Array(5),
].map((_, i) => i);
// TODO: Expressionのdefault値を確認する。
const DEFAULT_VALUE = ["", "unknown", "OFF", "0", "128"];
export default class PerformanceMonitor {
    #dst;
    #max_ch;
    #state = [];
    #tempo;
    #tempo_is_change = false;
    constructor(id, max_ch = 6) {
        this.#dst = document.getElementById(id);
        this.#max_ch = max_ch;
        for (let i = 0; i < max_ch; i++) {
            this.#state[i] = {
                ch: i + 1,
                is_change: [],
                items: [],
            };
        }
    }
    init() {
        const fragment = document.createDocumentFragment();
        const header_elm = document.createElement("tr");
        for (let i = 0; i < PLAYER_HEADER_ITEMS.length; ++i) {
            const item = document.createElement("th");
            item.textContent = PLAYER_HEADER_ITEMS[i];
            header_elm.appendChild(item);
        }
        fragment.appendChild(header_elm);
        // 各種パラメータの表示場所の生成
        for (let i = 0; i < this.#max_ch; ++i) {
            const tr = document.createElement("tr");
            const ch = document.createElement("th");
            ch.textContent = `Ch${i + 1}:`;
            tr.appendChild(ch);
            for (let j = 0; j < PLAYER_HEADER_ITEMS.length - 1; ++j) {
                const td = document.createElement("td");
                td.id = `ch${i + 1}_${PLAYER_HEADER_ITEMS[j + 1]}`;
                tr.appendChild(td);
            }
            fragment.appendChild(tr);
            // パラメータの初期化
            this.#reset_ch(i);
        }
        this.#dst.appendChild(fragment);
    }
    /**
     *
     * @param {SequenceMsg} msg
     */
    update(msg) {
        if (msg.is_ignore_msg()) return;
        const ch = msg.get_channel();
        const state = this.#state[ch];
        if (msg.is_program_change()) {
            state.items[INSTRUMENT_INDEX] = msg.get_instrument();
            state.is_change[INSTRUMENT_INDEX] = true;
        } else if (msg.is_tempo()) {
            this.#tempo = msg.get_tempo();
            this.#tempo_is_change = true;
        } else if (msg.is_key_event()) {
            const note = msg.get_note();
            state.items[NOTE_INDEX] =
                `${note.is_key_on() ? "ON" : "OFF"} ${note.note_name}(${note.note_number})`;
            state.is_change[NOTE_INDEX] = true;
        } else if (msg.is_all_stop()) {
            state.items[NOTE_INDEX] = "Off";
        } else if (msg.is_reset_controller()) {
            state.items[PITCH_INDEX] = 0;
            state.items[Expr_INDEX] = 127;
        }
    }
    commit() {
        if (this.#tempo_is_change) {
            document.getElementById("bpm").innerHTML = this.#tempo;
        }
        const items = PLAYER_HEADER_ITEMS.length;
        for (const ch of this.#state) {
            for (let i = 1; i < items; ++i) {
                if (!ch.is_change[i]) continue;
                document.getElementById(
                    `ch${ch.ch}_${PLAYER_HEADER_ITEMS[i]}`,
                ).innerHTML = ch.items[i];
                ch.is_change[i] = false;
            }
        }
    }
    #reset_ch(channel) {
        for (let i = 1; i < PLAYER_HEADER_ITEMS.length; ++i) {
            this.#state[channel].items[i] = DEFAULT_VALUE[i];
            this.#state[channel].is_change[i] = true;
        }
    }
    reset_all() {
        for (let i = 0; i < this.#max_ch; ++i) this.#reset_ch(i);
    }
}
