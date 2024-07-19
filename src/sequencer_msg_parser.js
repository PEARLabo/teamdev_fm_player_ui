/**
 * 
 * @param lo 
 * @param hi 
 * @param print_log_func print log function
 * @returns null | {ch: number, key: number, vel: number}
 */
function parse_event_msg(lo, hi,print_log_func) {
  let len = (data & 0xf0) >> 4;
  let flag_a = (data & 0xf00) >> 8;
  let ch = (lo & 0xf000) >> 12;
  let msg_data = lo >> 16;
  let dst = {
    ch: ch, //Channel
    key: 0, // Key
    vel: 0, // Velocity
  };
  if (!print_log_func) {
    print_log_func = () => {};
  }
  switch (flag_a) {
    case 0:
      // KeyEvent
      let key = msg_data & 0xff;
      let vel = (msg_data & 0xff) >> 8;
      print_log_func(`Ch: ${ch} Key${vel?"ON":"OFF"} Key: ${key}`);
    break;
    default:
      // console.log("hogehoge \(* ^  w  ^ *)/");
      print_log_func(`Ch: ${ch} Something Event...`);
      dst = null;
      break;
  }
  return dst;
}