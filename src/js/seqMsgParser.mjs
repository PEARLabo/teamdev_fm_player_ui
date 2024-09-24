import Note from "./note.mjs";
const [
    EventKeyEvent,
    EventTempo, // 4分音符１つ分のusec
    EventEnd,
    EventNop,
    EventParam, // | ParamType(8bit) |slot(8bit)|data(8bit)|
    EventProgramChange,
    EventExpression,
    EventPitchBend,
    EventPitchBendSensitivity,
    EventControllerReset,
    EventAllSoundOff,
    EventAllNoteOff,
    EventOther,
] = [...Array(8)].map((_, i) => i);
const PARAM_SET_LUT = [
    "SlotMask",
    "DetuneMultiple",
    "TotalLevel",
    "KeyScale AttackRate",
    "DecayRate",
    "SustainRate",
    "SustainLevel ReleaseRate",
    "FeedbackLevel Connection",
];
export default class SequenceMsg {
    #note;
    #tempo = 0;
    #instrument = "";
    #sq_event = EventOther;
    #ch = 0;
    /**
     *
     * @param {} sequence_msg
     */
    constructor(sequence_msg) {
        let flag = sequence_msg.sq_event;
        const ch = sequence_msg.channel;
        switch (flag) {
            case EventKeyEvent:
                this.#note = new Note(
                    sequence_msg.data[0],
                    sequence_msg.data[1],
                );
                break;
            case EventTempo:
                this.#tempo =
                    sequence_msg.data[0] |
                    (sequence_msg.data[1] << 8) |
                    (sequence_msg.data[2] << 16);
                break;
            case EventProgramChange:
                {
                    let name = "";
                    for (let i = 0; i < 6; i++) {
                        name += String.fromCharCode(sequence_msg.data[i]);
                    }
                    this.#instrument = name;
                }
                break;
            case EventEnd:
            case EventReset:
            case EventAllSoundOff:
            case EventAllNoteOff:
                // 値を持たないメッセージ
                break;
            default:
                flag = EventOther;
                // 未対応イベント
                // フロント未実装の物はすべてこことなる。
                break;
        }
        this.#sq_event = flag; // イベントタイプ
        this.#ch = ch; // チャンネル番号
    }
    static nop() {
        return new SequenceMsg({ sq_event: EventNop, channel: 0 });
    }
    is_key_event() {
        return this.#sq_event === EventKeyEvent;
    }
    is_nop() {
        return this.#sq_event === EventNop;
    }
    is_tempo() {
        return this.#sq_event === EventTempo;
    }
    is_program_change() {
        return this.#sq_event === EventProgramChange;
    }
    is_other() {
        return this.#sq_event === EventOther;
    }
    is_ignore_msg() {
        return this.#sq_event === EventOther || this.#sq_event === EventNop;
    }
    is_end() {
        return this.#sq_event === EventEnd;
    }
    is_reset_controller() {
        return this.#sq_event === EventControllerReset;
    }
    is_all_stop() {
        return (
            this.#sq_event === EventAllNoteOff ||
            this.#sq_event === EventAllSoundOff
        );
    }
    get_channel() {
        return this.#ch;
    }
    get_tempo() {
        return this.#tempo;
    }
    /**
     *
     * @returns {Note}
     */
    get_note() {
        return this.#note;
    }
    get_instrument() {
        return this.#instrument;
    }
}
