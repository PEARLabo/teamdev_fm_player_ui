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
  EventText,
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
  let minuit_per_beat = usec_per_beat / 1000_000 / 60;
  return (1/minuit_per_beat + 0.5) | 0;
}
function parse_event_msg(lo, hi, print_log_func) {
  let len = (lo & 0xf0) >>> 4;
  if ((lo & 0x0f) != 1) {
    // Todo: メッセージに合わせてよきに計らって。
    console.log(`Other Message Received: Flag is ${lo & 0x0f}`);
    return null;
  }
  let flag_a = (lo & 0xf00) >>> 8;
  let ch = (lo & 0xf000) >>> 12;
  let msg_data = lo >>> 16;
  let dst = {
    ch: ch, //Channel
    key: 0, // Key
    vel: 0, // Velocity
  };
  if (!print_log_func) {
    print_log_func = () => { };
  }
  switch (flag_a) {
    case EventKeyEvent:
      {// KeyEvent
        let key = msg_data & 0xff;
        let vel = (msg_data & 0xff) >>> 8;
        print_log_func(`Ch${ch}: Key${vel ? "ON" : "OFF"} Key: ${key}`);
      }
      break;
    case EventTempo:
      print_log_func(`Tempo Change: ${convert_to_bpm(msg_data + (hi * 0x100))}`);
      dst = null;
      break;
    case EventEnd:
      print_log_func("End of playing");
      dst = null;
      break;
    case EventParam:
      {
        let param_target = (msg_data >> 16) & 0xf;
        let slot = (msg_data >> 20) & 0xf;
        let reg_value = (msg_data >>> 24) & 0xff;
        print_log_func(`Ch${ch}: Parameter Set @ slot${slot+1} - ${PARAM_SET_LUT[param_target]} := ${reg_value}`);
      }
      dst = null;
      break;
    case EventProgramChange:
      {
        let name = String.fromCharCode(msg_data & 0x7f); + String.fromCharCode((msg_data & 0x7f00) >> 8);
        for (let i = 0; i < 4; i++) {
          name += String.fromCharCode((hi >>> (i * 8)) & 0x7f);
        }
        print_log_func(`Ch${ch}: Preset Set "${name}"`);
        dst = null;
      }
      break;
    case EventExpression:
      print_log_func(`Ch${ch}: Expression Change ${msg_data}`);
      dst = null;
      break;
    default:
      // console.log("hogehoge \(* ^  w  ^ *)/");
      print_log_func(`Ch${ch}?: Something Event...`);
      dst = null;
      break;
  }
  return dst;
}