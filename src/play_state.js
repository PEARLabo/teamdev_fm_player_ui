const player_header_items = ["channel", "timbre", "note", "pitch", "Expr"];
const [CHANNEL_INDEX, TIMBRE_INDEX, NOTE_INDEX, PITCH_INDEX, Expr_INDEX] = [
  ...Array(5),
].map((_, i) => i);
function init_play_state_display(max_ch = 6) {
  let fragment = document.createDocumentFragment();

  let header_elm = document.createElement("tr");
  for (let i = 0; i < player_header_items.length; ++i) {
    let item = document.createElement("th");
    item.textContent = player_header_items[i];
    header_elm.appendChild(item);
  }
  fragment.appendChild(header_elm);
  for (let i = 0; i < max_ch; ++i) {
    let tr = document.createElement("tr");
    let ch = document.createElement("th");
    ch.textContent = `Ch${i + 1}:`;
    tr.appendChild(ch);
    for (let j = 0; j < player_header_items.length - 1; ++j) {
      let td = document.createElement("td");
      td.id = `ch${i + 1}_${player_header_items[j + 1]}`;
      tr.appendChild(td);
    }
    fragment.appendChild(tr);
  }
  document.getElementById("currentPlayState").appendChild(fragment);
}
function update_play_state_display(msg) {
  if (msg.tempo) {
    document.getElementById("bpm").innerHTML = msg.tempo;
  } else if (msg.timbre.length) {
    document.getElementById(
      `ch${msg.ch + 1}_${player_header_items[TIMBRE_INDEX]}`,
    ).innerHTML = msg.timbre;
  } else if (!msg.vel) {
    document.getElementById(
      `ch${msg.ch + 1}_${player_header_items[NOTE_INDEX]}`,
    ).innerHTML = `OFF ${msg.key}`;
  } else {
    document.getElementById(
      `ch${msg.ch + 1}_${player_header_items[NOTE_INDEX]}`,
    ).innerHTML = `ON ${msg.key}`;
  }
}
