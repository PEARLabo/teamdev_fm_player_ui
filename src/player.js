let activeNotes = new Set();

// ピアノロールの描画(switchPlayer関数内で呼び出し)
function drawPianoRoll() {
  const canvas = document.getElementById('pianoRoll');
  canvas.height = 400; // Canvasの高さを設定
  canvas.width = 900; // Canvasの幅を設定

  const ctx = canvas.getContext('2d');

  const timeDisplayHeight = 20; // 経過時間表示のための高さ
  const pianoHeight = (canvas.height - timeDisplayHeight) / 2; // 鍵盤のための高さを2段に分ける

  const whiteKeyWidth = canvas.width / 21; // 3オクターブ分の白鍵を横に収める
  const whiteKeyHeight = pianoHeight; // 白鍵の高さを全体に合わせて調整
  const blackKeyWidth = whiteKeyWidth * 0.6;
  const blackKeyHeight = whiteKeyHeight * 0.6;
  const numOctaves = 6; // 6オクターブ表示に調整

  const whiteKeys = [0, 2, 4, 5, 7, 9, 11];
  const blackKeys = [1, 3, 6, 8, 10];

  let startTime = null;

  // ピアノの描画
  function drawPiano() {
    // 先に全ての白鍵を描画
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = (octave % 3) * 7 * whiteKeyWidth;
      const yOffset = Math.floor(octave / 3) * pianoHeight;

      whiteKeys.forEach((key, i) => {
        const pitch = octave * 12 + key + 24; // C1から開始するように24を追加
        const x = octaveOffsetX + i * whiteKeyWidth;
        const y = yOffset;
        ctx.fillStyle = activeNotes.has(pitch) ? '#D3D3D3' : 'white';  // 薄い灰色でアクティブな白鍵を表示
        ctx.fillRect(x, y, whiteKeyWidth, whiteKeyHeight);
        ctx.strokeStyle = 'black';
        ctx.strokeRect(x, y, whiteKeyWidth, whiteKeyHeight);

        // Cのラベルを表示
        if (key === 0) {
          ctx.fillStyle = 'black';
          ctx.font = '14px Arial';
          ctx.fillText(`C${Math.floor(pitch / 12) - 1}`, x + 5, y + whiteKeyHeight - 5);
        }
      });
    }

    // 黒鍵を描画し、その上にアクティブな色を適用
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = (octave % 3) * 7 * whiteKeyWidth;
      const yOffset = Math.floor(octave / 3) * pianoHeight;

      blackKeys.forEach((key) => {
        const pitch = octave * 12 + key + 24; // C#1から開始するように24を追加
        const x = octaveOffsetX + whiteKeys.indexOf(key - 1) * whiteKeyWidth + whiteKeyWidth - blackKeyWidth / 2;
        const y = yOffset;

        // 黒鍵の基本描画
        ctx.fillStyle = activeNotes.has(pitch) ? '#A9A9A9' : 'black';  // 濃い灰色でアクティブな黒鍵を表示
        ctx.fillRect(x, y, blackKeyWidth, blackKeyHeight);
        ctx.strokeStyle = 'black';
        ctx.strokeRect(x, y, blackKeyWidth, blackKeyHeight);
      });
    }
  }

  // 経過時間の表示
  function displayTime(elapsedTime) {
    ctx.fillStyle = 'black';
    ctx.font = '16px Arial';
    ctx.fillText(`Time: ${Math.floor(elapsedTime / 1000)}s`, 10, canvas.height - 5);
  }

  // アニメーションの開始
  function animate(time) {
    if (!startTime) startTime = time;
    const elapsedTime = time - startTime;

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawPiano();
    displayTime(elapsedTime);

    requestAnimationFrame(animate);
  }

  requestAnimationFrame(animate);

  // ノートオンイベントを処理
  function noteOn(pitch) {
    // 存在しない場合追加する
    if (!activeNotes.has(pitch)) {
      activeNotes.add(pitch);
    }
  }

  // ノートオフイベントを処理
  function noteOff(pitch) {
    // 存在するすべてのノートを削除する
    if (activeNotes.has(pitch)) {
      activeNotes.delete(pitch);
      noteOff(pitch);
    }
  }

  // 外部からノートオン、ノートオフをトリガーできるようにする
  window.noteOn = noteOn;
  window.noteOff = noteOff;
}

function updatePianoRoll(data) {
  const playerConsoleArea = document.getElementById('playerConsole');
  if (playerConsoleArea) {
    playerConsoleArea.value += `Playback Data: ${data}\n`;
    playerConsoleArea.scrollTop = playerConsoleArea.scrollHeight;
  }

  // イベントタイプに応じて処理を分ける
  if (data.startsWith("tempo:")) {
    // テンポ情報を更新
    updateTempo(data);
  } else if (data.startsWith("chanel:")) {
    // ノートオン/オフイベントを処理
    const noteMatch = data.match(/chanel: \d+\(\s*\d+\), key: (\d+)\(\s*\d+\), velocity: (\d+)\(\s*\d+\)/);
    if (noteMatch) {
      const pitch = parseInt(noteMatch[1], 10);
      const velocity = parseInt(noteMatch[2], 10);
      //playerConsoleArea.value += `\nNote: ${pitch}, Velocity: ${velocity}\n`;
      if (velocity === 0) {
        window.noteOff(pitch);
      } else {
        window.noteOn(pitch);
      }
    }
  } else if (data.startsWith("End")) {
    // 終了イベントを処理
    handleEndEvent();
    playerConsoleArea.value += '==Playback Ended==\n';
  } else {
    // その他のイベントを処理
    console.warn("Unknown event type:", data);
  }
}

function updateTempo(data) {
  // テンポ情報をパースして更新
  const tempoMatch = data.match(/tempo: (\d+)/);
  if (tempoMatch) {
    const tempo = parseInt(tempoMatch[1], 10);
    const tempoDisplay = document.getElementById('tempoDisplay');
    if (tempoDisplay) {
      tempoDisplay.textContent = `Tempo: ${tempo}`;
    }
  }
}

function updateParameterChange(data) {
  // パラメータ変更イベントをピアノロールに反映
  // ここでは簡略化してログを出力する
  console.log("Parameter Change Event:", data);
}

function handleEndEvent() {
  // 終了イベントの処理
  activeNotes.clear();
  console.log("End Event received.");
}

function handleFlagAEvent(data) {
  // フラグA関連のイベントの処理
  console.log("FlagA Event:", data);
}
