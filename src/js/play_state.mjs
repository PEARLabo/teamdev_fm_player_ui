import SequenceMsg from "./sequencer_msg_parser.mjs";

const PLAYER_HEADER_ITEMS = ["channel", "instrument", "note", "pitch", "Expr"];
const [CHANNEL_INDEX, INSTRUMENT_INDEX, NOTE_INDEX, PITCH_INDEX, Expr_INDEX] = [
  ...Array(5),
].map((_, i) => i);
// TODO: Expressionのdefault値を確認する。
const DEFAULT_VALUE = ["", "unknown", "OFF", "0", "128"];
export function init_play_state_display(max_ch = 6) {
  const fragment = document.createDocumentFragment();
  const header_elm = document.createElement("tr");
  for (let i = 0; i < PLAYER_HEADER_ITEMS.length; ++i) {
    const item = document.createElement("th");
    item.textContent = PLAYER_HEADER_ITEMS[i];
    header_elm.appendChild(item);
  }
  fragment.appendChild(header_elm);
  for (let i = 0; i < max_ch; ++i) {
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
  }
  document.getElementById("currentPlayState").appendChild(fragment);
  update_play_state_display(SequenceMsg.nop(), true);
}

/**
 *
 * @param {SequenceMsg} msg
 */
export function update_play_state_display(msg, is_reset = false) {
  if (is_reset || msg.is_reset()) {
    const channels =
      document.getElementById("currentPlayState").children.length - 1;
    console.log(channels);
    for (let i = 0; i < channels; ++i) {
      for (let j = 1; j < PLAYER_HEADER_ITEMS.length; j++) {
        document.getElementById(
          `ch${i + 1}_${PLAYER_HEADER_ITEMS[j]}`,
        ).innerHTML = DEFAULT_VALUE[j];
      }
    }
  } else if (msg.is_tempo()) {
    document.getElementById("bpm").innerHTML = msg.get_tempo();
  } else if (msg.is_program_change()) {
    document.getElementById(
      `ch${msg.get_channel() + 1}_${PLAYER_HEADER_ITEMS[INSTRUMENT_INDEX]}`,
    ).innerHTML = msg.get_instrument();
  } else if (msg.is_key_event()) {
    const note = msg.get_note();
    document.getElementById(
      `ch${msg.get_channel() + 1}_${PLAYER_HEADER_ITEMS[NOTE_INDEX]}`,
    ).innerHTML =
      `${note.is_key_on() ? "ON" : "OFF"} ${note.note_name}(${note.note_number})`;
  } else if (msg.is_all_note_off()) {
    const channels =
      document.getElementById("currentPlayState").children.length - 1;
    for (let i = 0; i < channels; ++i) {
      document.getElementById(
        `ch${i + 1}_${PLAYER_HEADER_ITEMS[NOTE_INDEX]}`,
      ).innerHTML = "OFF";
    }
  }
}
