document.addEventListener('DOMContentLoaded', () => {
  const canvas = document.getElementById('pianoRoll');
  canvas.height = 400; // Canvasの高さを設定
  canvas.width = 900; // Canvasの幅を設定、適切に鍵盤が収まるように調整

  const ctx = canvas.getContext('2d');

  const timeDisplayHeight = 20; // 経過時間表示のための高さ
  const pianoHeight = canvas.height - timeDisplayHeight; // 鍵盤のための高さ

  const whiteKeyWidth = 50;
  const whiteKeyHeight = pianoHeight; // 白鍵の高さを全体に合わせて調整
  const blackKeyWidth = whiteKeyWidth * 0.6;
  const blackKeyHeight = whiteKeyHeight * 0.6;
  const numOctaves = 3; // 3オクターブ表示に調整

  // サンプルノート情報
  const notes = [
    { start: 0, end: 5000, pitch: 60 },  // C4
    { start: 0, end: 5000, pitch: 61 },  // C#4
    { start: 1000, end: 6000, pitch: 62 }, // D4
    { start: 2000, end: 7000, pitch: 63 }, // D#4
    { start: 3000, end: 8000, pitch: 64 }, // E4
  ];

  let startTime = null;

  // ピアノの描画
  function drawPiano(activeKeys) {
    // 先に全ての白鍵を描画
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = octave * 7 * whiteKeyWidth;

      for (let i = 0; i < 7; i++) {
        const pitch = octave * 12 + i + 48;
        const x = octaveOffsetX + i * whiteKeyWidth;
        const y = 0;
        ctx.fillStyle = activeKeys.has(pitch) ? '#D3D3D3' : 'white';  // 薄い灰色でアクティブな白鍵を表示
        ctx.fillRect(x, y, whiteKeyWidth, whiteKeyHeight);
        ctx.strokeStyle = 'black';
        ctx.strokeRect(x, y, whiteKeyWidth, whiteKeyHeight);
      }
    }

    // 黒鍵を描画し、その上にアクティブな色を適用
    for (let octave = 0; octave < numOctaves; octave++) {
      const octaveOffsetX = octave * 7 * whiteKeyWidth;

      const blackKeyPositions = [1, 2, 4, 5, 6];
      blackKeyPositions.forEach(pos => {
        const pitch = octave * 12 + Math.floor(pos) + 48;
        const x = octaveOffsetX + pos * whiteKeyWidth - blackKeyWidth / 2;
        const y = 0;

        // 黒鍵の基本描画
        ctx.fillStyle = activeKeys.has(pitch) ? '#A9A9A9' : 'black';  // 濃い灰色でアクティブな黒鍵を表示
        ctx.fillRect(x, y, blackKeyWidth, blackKeyHeight);
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

    const activeKeys = new Set();
    notes.forEach(note => {
      if (elapsedTime >= note.start && elapsedTime <= note.end) {
        activeKeys.add(note.pitch);
      }
    });

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawPiano(activeKeys);
    displayTime(elapsedTime);

    requestAnimationFrame(animate);
  }

  requestAnimationFrame(animate);
});
