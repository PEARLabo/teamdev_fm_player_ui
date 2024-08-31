let activeNotes = new Set();
let previousActiveNotes = new Set();
let eventQueue = new Queue(512);

const CANVAS_WIDTH = 900 / 2;
const CANVAS_HEIGHT = 400 / 2;
const TIME_DISPLAY_HEIGHT = 20;
const PIANO_HEIGHT = (CANVAS_HEIGHT - TIME_DISPLAY_HEIGHT) / 2;
const WHITE_KEY_WIDTH = CANVAS_WIDTH / 21;
const WHITE_KEY_HEIGHT = PIANO_HEIGHT;
const BLACK_KEY_WIDTH = WHITE_KEY_WIDTH * 0.6;
const BLACK_KEY_HEIGHT = WHITE_KEY_HEIGHT * 0.6;

function drawPianoRoll() {
  const canvas = document.getElementById('pianoRoll');
  canvas.height = CANVAS_HEIGHT;
  canvas.width = CANVAS_WIDTH;
  const ctx = canvas.getContext('2d');
  
  const numOctaves = 6;
  const whiteKeys = [0, 2, 4, 5, 7, 9, 11];
  const blackKeys = [1, 3, 6, 8, 10];
  const module_3_table = [0,1,2,0];

  function drawKey(pitch, isActive) {
    const octave = Math.floor((pitch - 24) / 12);
    const key = (pitch - 24) % 12;

    const octaveOffsetX = module_3_table[(octave % 3)] * 7 * WHITE_KEY_WIDTH;
    const yOffset = Math.floor(octave / 3) * PIANO_HEIGHT;

    if (whiteKeys.includes(key)) {
      const i = whiteKeys.indexOf(key);
      const x = octaveOffsetX + i * WHITE_KEY_WIDTH;
      const y = yOffset;
      ctx.fillStyle = isActive ? '#D3D3D3' : 'white';
      ctx.fillRect(x, y, WHITE_KEY_WIDTH, WHITE_KEY_HEIGHT);
      ctx.strokeStyle = 'black';
      ctx.strokeRect(x, y, WHITE_KEY_WIDTH, WHITE_KEY_HEIGHT);

      if (key === 0) {
        ctx.fillStyle = 'black';
        ctx.font = '14px Arial';
        ctx.fillText(`C${Math.floor(pitch / 12) - 1}`, x + 5, y + WHITE_KEY_HEIGHT - 5);
      }
    } else if (blackKeys.includes(key)) {
      const i = blackKeys.indexOf(key);
      const x = octaveOffsetX + whiteKeys.indexOf(key - 1) * WHITE_KEY_WIDTH + WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2;
      const y = yOffset;
      ctx.fillStyle = isActive ? '#A9A9A9' : 'black';
      ctx.fillRect(x, y, BLACK_KEY_WIDTH, BLACK_KEY_HEIGHT);
      ctx.strokeStyle = 'black';
      ctx.strokeRect(x, y, BLACK_KEY_WIDTH, BLACK_KEY_HEIGHT);
    }
  }

  function drawInitialPiano() {
    // 最初に全ての鍵盤を描画する
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = module_3_table[(octave % 3)] * 7 * WHITE_KEY_WIDTH;
      const yOffset = Math.floor(octave / 3) * PIANO_HEIGHT;
      for (let key = 0; key < 12; key++) {
        drawKey(octave * 12 + key,false);
      }
      // whiteKeys.forEach((key, i) => {
      //   const pitch = octave * 12 + key + 24;
      //   const x = octaveOffsetX + i * WHITE_KEY_WIDTH;
      //   const y = yOffset;
      //   ctx.fillStyle = 'white';
      //   ctx.fillRect(x, y, WHITE_KEY_WIDTH, WHITE_KEY_HEIGHT);
      //   ctx.strokeStyle = 'black';
      //   ctx.strokeRect(x, y, WHITE_KEY_WIDTH, WHITE_KEY_HEIGHT);

      //   if (key === 0) {
      //     ctx.fillStyle = 'black';
      //     ctx.font = '14px Arial';
      //     ctx.fillText(`C${Math.floor(pitch / 12) - 1}`, x + 5, y + WHITE_KEY_HEIGHT - 5);
      //   }
      // });

      // blackKeys.forEach((key) => {
      //   const pitch = octave * 12 + key + 24;
      //   const x = octaveOffsetX + whiteKeys.indexOf(key - 1) * WHITE_KEY_WIDTH + WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2;
      //   const y = yOffset;
      //   ctx.fillStyle = 'black';
      //   ctx.fillRect(x, y, BLACK_KEY_WIDTH, BLACK_KEY_HEIGHT);
      //   ctx.strokeStyle = 'black';
      //   ctx.strokeRect(x, y, BLACK_KEY_WIDTH, BLACK_KEY_HEIGHT);
      // });
    }
  }

  function updateActiveKeys() {
    // リセットする必要がある鍵盤を先にリセット
    previousActiveNotes.forEach(pitch => {
      if (!activeNotes.has(pitch)) {
        drawKey(pitch, false);
      }
    });

    // 新しいアクティブ状態の鍵盤を描画
    activeNotes.forEach(pitch => {
      if (!previousActiveNotes.has(pitch)) {
        drawKey(pitch, true);
      }
    });

    // 前のアクティブノートを現在のものに更新
    previousActiveNotes = new Set(activeNotes);
  }

  function animate(time) {
    ctx.clearRect(0, CANVAS_HEIGHT - TIME_DISPLAY_HEIGHT, canvas.width, TIME_DISPLAY_HEIGHT);
    displayTime(time);
    updateActiveKeys();
    processEventQueue();
    requestAnimationFrame(animate);
  }

  // 最初にピアノ全体を描画する
  drawInitialPiano();

  requestAnimationFrame(animate);

  function noteOn(pitch) {
    if (!activeNotes.has(pitch)) {
      activeNotes.add(pitch);
    }
  }

  function noteOff(pitch) {
    activeNotes.delete(pitch);
  }

  window.noteOn = noteOn;
  window.noteOff = noteOff;
}

function processEventQueue() {
  while (eventQueue.length > 0) {
    const data = eventQueue.dequeue();
    handleEvent(data);
  }
}

function updatePianoRoll(data) {
  eventQueue.enqueue(data);
  const playerConsoleArea = document.getElementById('playerConsole');
  if (playerConsoleArea) {
    playerConsoleArea.value = `Playback Data: key: ${data.key}, vel: ${data.vel}\n`;
    playerConsoleArea.scrollTop = playerConsoleArea.scrollHeight;
  }
}

function handleEvent(data) {
  if(data.vel !== 0) {
    window.noteOn(data.key);
  } else {
    window.noteOff(data.key);
  }
}

function displayTime(elapsedTime) {
  const canvas = document.getElementById('pianoRoll');
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = 'black';
  ctx.font = '16px Arial';
  ctx.fillText(`Time: ${Math.floor(elapsedTime / 1000)}s`, 10, canvas.height - 5);
}

function updateTempo(data) {
  if (data.tempo) {
    const tempoDisplay = document.getElementById('tempoDisplay');
    if (tempoDisplay) {
      tempoDisplay.textContent = `Tempo: ${data.tempo}`;
    }
  }
}

function handleEndEvent() {
  activeNotes.clear();
  console.log("End Event received.");
}
