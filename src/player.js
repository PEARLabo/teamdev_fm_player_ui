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
  let activeNotes = new Set();

  // ピアノの描画
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
        playerConsoleArea.value += `\nNote: ${pitch}, Velocity: ${velocity}\n`;
        if (velocity === 0) {
          window.noteOff(pitch);
        } else {
          window.noteOn(pitch);
        }
      }
    } else if (data === "End") {
      // 終了イベントを処理
      handleEndEvent();
      playerConsoleArea.value += '==Playback Ended==\n';
    } else {
      // その他のイベントを処理
      console.warn("Unknown event type:", data);
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
    activeNotes.add(pitch);
  }

  // ノートオフイベントを処理
  function noteOff(pitch) {
    activeNotes.delete(pitch);
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
      playerConsoleArea.value += `\nNote: ${pitch}, Velocity: ${velocity}\n`;
      if (velocity === 0) {
        window.noteOff(pitch);
      } else {
        window.noteOn(pitch);
      }
    }
  } else if (data === "End") {
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
  console.log("End Event received.");
}

function handleFlagAEvent(data) {
  // フラグA関連のイベントの処理
  console.log("FlagA Event:", data);
}
