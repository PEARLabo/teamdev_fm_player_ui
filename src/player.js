let activeNotes = new Map();
let eventQueue = new Queue(512);
// const variables
const CANVAS_WIDTH = 900;
const CANVAS_HEIGHT = 400;
const TIME_DISPLAY_HEIGHT = 20; // 経過時間表示のための高さ
const PIANO_HEIGHT = (CANVAS_HEIGHT - TIME_DISPLAY_HEIGHT) / 2; // 鍵盤のための高さを2段に分ける
const WHITE_KEY_WIDTH = CANVAS_WIDTH / 21; // 3オクターブ分の白鍵を横に収める
const WHITE_KEY_HEIGHT = PIANO_HEIGHT; // 白鍵の高さを全体に合わせて調整
const BLACK_KEY_WIDTH = WHITE_KEY_WIDTH * 0.6;
const BLACK_KEY_HEIGHT = WHITE_KEY_HEIGHT * 0.6;
// Note: 適当な色を設定。いい感じに変更求ム
const COLOR_LUT = [0xf08080, 0xfffacd, 0xe6e6fa, 0xd6efff, 0xcfffe5, 0xffd1dc];
let canvas_is_update_frame = true;
let is_play_state_changed = true;
let draw_count = 0;
// ピアノロールの描画(switchPlayer関数内で呼び出し)
function drawPianoRoll() {
  const canvas = document.getElementById("pianoRoll");
  canvas.height = CANVAS_HEIGHT; // Canvasの高さを設定
  canvas.width = CANVAS_WIDTH; // Canvasの幅を設定

  const ctx = canvas.getContext("2d");

  const numOctaves = 6; // 6オクターブ表示に調整

  const whiteKeys = [0, 2, 4, 5, 7, 9, 11];
  const blackKeys = [1, 3, 6, 8, 10];
  const module_3_table = [0, 1, 2, 0];
  let startTime = null;

  // ピアノの描画
  function drawPiano() {
    // let start_time = performance.now();
    // 先に全ての白鍵を描画
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = module_3_table[octave % 3] * 7 * WHITE_KEY_WIDTH;
      const yOffset = Math.floor(octave / 3) * PIANO_HEIGHT;

      for (let i = 0, _end = whiteKeys.length; i < _end; i++) {
        let key = whiteKeys[i];
        const pitch = octave * 12 + key + 24; // C1から開始するように24を追加
        const x = octaveOffsetX + i * WHITE_KEY_WIDTH;
        const y = yOffset;

        ctx.fillStyle = activeNotes.has(pitch)
          ? into_color_code(get_key_color(activeNotes.get(pitch)))
          : "white"; // 薄い灰色でアクティブな白鍵を表示
        ctx.fillRect(x, y, WHITE_KEY_WIDTH, WHITE_KEY_HEIGHT);
        ctx.strokeStyle = "black";
        ctx.strokeRect(x, y, WHITE_KEY_WIDTH, WHITE_KEY_HEIGHT);

        // Cのラベルを表示
        if (key === 0) {
          ctx.fillStyle = "black";
          ctx.font = "14px Arial";
          ctx.fillText(
            `C${Math.floor(pitch / 12) - 1}`,
            x + 5,
            y + WHITE_KEY_HEIGHT - 5,
          );
        }
      }
    }

    // 黒鍵を描画し、その上にアクティブな色を適用
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = module_3_table[octave % 3] * 7 * WHITE_KEY_WIDTH;
      const yOffset = Math.floor(octave / 3) * PIANO_HEIGHT;

      for (let i = 0, _end = blackKeys.length; i < _end; i++) {
        let key = blackKeys[i];
        const pitch = octave * 12 + key + 24; // C#1から開始するように24を追加
        const x =
          octaveOffsetX +
          whiteKeys.indexOf(key - 1) * WHITE_KEY_WIDTH +
          WHITE_KEY_WIDTH -
          BLACK_KEY_WIDTH / 2;
        const y = yOffset;
        // 黒鍵の基本描画
        ctx.fillStyle = activeNotes.has(pitch)
          ? into_color_code(darken(get_key_color(activeNotes.get(pitch)), 0.9))
          : "black"; // 濃い灰色でアクティブな黒鍵を表示
        ctx.fillRect(x, y, BLACK_KEY_WIDTH, BLACK_KEY_HEIGHT);
        ctx.strokeStyle = "black";
        ctx.strokeRect(x, y, BLACK_KEY_WIDTH, BLACK_KEY_HEIGHT);
      }
    }
    is_play_state_changed = false;
    // let end = performance.now();
    // console.log("Render Elapsed Time: " + (end - start_time))
  }

  // // 経過時間の表示
  // function displayTime(elapsedTime) {
  //   ctx.fillStyle = "black";
  //   ctx.font = "16px Arial";
  //   // 秒数は小数点以下1桁まで表示
  //   ctx.fillText(
  //     `Time: ${Math.floor(elapsedTime / 1000)}s`,
  //     10,
  //     canvas.height - 5,
  //   );
  // }

  // アニメーションの開始
  function animate(time) {
    if (canvas_is_update_frame && is_play_state_changed) {
      // processEventQueue();
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      drawPiano();
      // draw_count++;
      // console.log(draw_count);
    }
    canvas_is_update_frame = !canvas_is_update_frame;

    requestAnimationFrame(animate);
  }

  requestAnimationFrame(animate);

  // 外部からノートオン、ノートオフをトリガーできるようにする
  window.noteOn = noteOn;
  window.noteOff = noteOff;
}

// イベントキューの処理
function processEventQueue() {
  while (eventQueue.length > 0) {
    const data = eventQueue.dequeue();
    handleEvent(data);
  }
}
// ノートオンイベントを処理
function noteOn(ch, pitch) {
  if (activeNotes.has(pitch)) {
    activeNotes.get(pitch).add(ch);
  }
  activeNotes.set(pitch, new Set([ch]));
  is_play_state_changed = true;
}

// ノートオフイベントを処理
function noteOff(ch, pitch) {
  if (!activeNotes.has(pitch)) return;
  let set = activeNotes.get(pitch);
  set.delete(ch);
  if (set.size == 0) {
    activeNotes.delete(pitch);
  }
  is_play_state_changed = true;
}
// イベントデータをキューに追加
function updatePianoRoll(data) {
  // eventQueue.enqueue(data);
  if (!data.is_key_event()) return;
  if (data.vel !== 0) {
    noteOn(data.ch, data.key);
  } else if (data.vel == 0) {
    noteOff(data.ch, data.key);
  }
}

// イベントデータの処理
function handleEvent(data) {
  if (data.vel !== 0) {
    noteOn(data.ch, data.key);
  } else if (data.vel == 0) {
    noteOff(data.ch, data.key);
  } else {
    // その他のイベントを処理
    console.warn("Unknown event type:", data);
  }
}

function updateTempo(data) {
  // テンポ情報をパースして更新
  // const tempoMatch = data.match(/tempo: (\d+)/);
  if (data.tempo) {
    // const tempo = parseInt(tempoMatch[1], 10);
    const tempoDisplay = document.getElementById("tempoDisplay");
    if (tempoDisplay) {
      tempoDisplay.textContent = `Tempo: ${data.tempo}`;
    }
  }
}

function handleEndEvent() {
  activeNotes.clear();
  console.log("End Event received.");
}
function get_key_color(current_on) {
  let color_array = [];
  current_on.forEach((ch) => color_array.push(COLOR_LUT[ch]));
  return color_mixer(color_array);
}
