const player_header_items = ["channel", "instrument", "note", "pitch", "Expr"];
const [CHANNEL_INDEX, INSTRUMENT_INDEX, NOTE_INDEX, PITCH_INDEX, Expr_INDEX] = [
  ...Array(5),
].map((_, i) => i);
export function init_play_state_display(max_ch = 6) {
  const fragment = document.createDocumentFragment();

  const header_elm = document.createElement("tr");
  for (let i = 0; i < player_header_items.length; ++i) {
    const item = document.createElement("th");
    item.textContent = player_header_items[i];
    header_elm.appendChild(item);
  }
  fragment.appendChild(header_elm);
  for (let i = 0; i < max_ch; ++i) {
    const tr = document.createElement("tr");
    const ch = document.createElement("th");
    ch.textContent = `Ch${i + 1}:`;
    tr.appendChild(ch);
    for (let j = 0; j < player_header_items.length - 1; ++j) {
      const td = document.createElement("td");
      td.id = `ch${i + 1}_${player_header_items[j + 1]}`;
      tr.appendChild(td);
    }
    fragment.appendChild(tr);
  }
  document.getElementById("currentPlayState").appendChild(fragment);
}

/**
 * 
 * @param {SequenceMsg} msg 
 */
export function update_play_state_display(msg) {
  if (msg.is_tempo()) {
    document.getElementById("bpm").innerHTML = msg.get_tempo();
  } else if (msg.is_program_change()) {
    document.getElementById(
      `ch${msg.get_channel() + 1}_${player_header_items[INSTRUMENT_INDEX]}`,
    ).innerHTML = msg.get_instrument();
  } else if (msg.is_key_event()) {
    const note = msg.get_note();
    document.getElementById(
      `ch${msg.get_channel() + 1}_${player_header_items[NOTE_INDEX]}`,
    ).innerHTML = `${note.is_key_on() ? "ON" : "OFF"} ${note.note_name}(${note.note_number})`;
  } else if (msg.is_end()) {
    // TODO: 表示のクリアの実装
  }
}
