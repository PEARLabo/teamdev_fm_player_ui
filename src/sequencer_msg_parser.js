/**
 *
 * @param lo
 * @param hi
 * @param print_log_func print log function
 * @returns null | {ch: number, key: number, vel: number}
 */
const [
  EventKeyEvent,
  EventTempo, // 4分音符１つ分のusec
  EventEnd,
  EventNop,
  EventParam, // | ParamType(8bit) |slot(8bit)|data(8bit)|
  EventProgramChange,
  EventExpression,
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
function convert_to_bpm(usec_per_beat) {
  let minute_per_beat = usec_per_beat / 1000_000;
  let beat_per_sec = 60 / minute_per_beat;
  return usec_per_beat;
}
class SequenceMsg {
  constructor(flag, ch) {
    this.sq_event = flag; // イベントタイプ
    this.ch = ch; // チャンネル番号
    this.key = 0; // キー番号
    this.vel = 0; // ベロシティ
    this.tempo = 0; // テンポ
    this.timbre = ""; // Timbre
  }
  is_key_event() {
    return this.sq_event == EventKeyEvent;
  }
}
function parse_event_msg(sequence_msg, print_log_func) {
  let flag_a = sequence_msg.sq_event;
  let ch = sequence_msg.channel;
  // Note: 未使用時0を記入
  let dst = new SequenceMsg(flag_a, ch);
  if (!print_log_func) {
    print_log_func = () => {};
  }
  switch (flag_a) {
    case EventKeyEvent:
      {
        dst.key = sequence_msg.data[0];
        dst.vel = sequence_msg.data[1];
      }
      break;
    case EventTempo:
      {
        dst.tempo =
          sequence_msg.data[0] |
          (sequence_msg.data[1] << 8) |
          (sequence_msg.data[2] << 16);
      }
      break;
    case EventEnd:
      dst = null;
      break;
    case EventProgramChange:
      {
        let name = "";
        for (let i = 0; i < 6; i++) {
          name += String.fromCharCode(sequence_msg.data[i]);
        }
        dst.timbre = name;
      }
      break;
    default:
      dst = null;
      break;
  }
  return dst;
}
