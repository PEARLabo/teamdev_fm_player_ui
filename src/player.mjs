import { into_color_code,color_mixer,darken } from "./color.mjs";
import { is_natural, distance_from_c,distance_from_c_sharp, is_accidental } from "./musical_scale.mjs";
const WHITE_KEY_WIDTH = 45;
const WHITE_KEY_HEIGHT = 200;
const BLACK_KEY_WIDTH = WHITE_KEY_WIDTH * 0.6;
const BLACK_KEY_HEIGHT = WHITE_KEY_HEIGHT * 0.6;

// Note: 適当な色を設定。いい感じに変更求ム
const COLOR_LUT = [0xf08080, 0xfffacd, 0xe6e6fa, 0xd6efff, 0xcfffe5, 0xffd1dc];

function get_key_color(current_on) {
  const color_array = [];
  current_on.forEach((ch) => color_array.push(COLOR_LUT[ch]));
  return color_mixer(color_array);
}

export default class PianoRoll {
  #activeNotes = new Map(); // Active Note
  #change_keys = new Set(); // State Changed Note
  #canvas_is_update_frame = true; //
  #canvas;
  #num_octaves = 6;
  #row_octaves = 3;
  #animate_id;

  constructor(id) {
    this.#canvas = document.getElementById(id);
    let rows = Math.ceil(this.#num_octaves / this.#row_octaves);
    this.#canvas.width = WHITE_KEY_WIDTH * this.#row_octaves * 7;
    this.#canvas.height = WHITE_KEY_HEIGHT * rows;
  }

  #draw_key(note,is_rewrite = true) {
    let ctx = this.#canvas.getContext("2d");
    const octave = Math.floor((note - 24) / 12);
    const key = (note - 24) % 12;
    const octaveOffsetX = (octave % this.#row_octaves) * 7 * WHITE_KEY_WIDTH;
    const yOffset = Math.floor(octave / this.#row_octaves) * WHITE_KEY_HEIGHT;
    let x;
    let i;
    let key_width;
    let key_height;
    const y = yOffset;
    let isActive = this.#activeNotes.has(note);
    let color;
    if (is_natural(key)) {
      console.log(note);
      i = distance_from_c(key);
      x = octaveOffsetX + i * WHITE_KEY_WIDTH;
      key_width = WHITE_KEY_WIDTH;
      key_height = WHITE_KEY_HEIGHT;
      color = isActive ?  get_key_color(this.#activeNotes.get(note)): 0xffffff;
    } else {
      i = distance_from_c_sharp(key);
      x = octaveOffsetX + distance_from_c(key - 1) * WHITE_KEY_WIDTH + WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2;
      key_width = BLACK_KEY_WIDTH;
      key_height = BLACK_KEY_HEIGHT;
      color = isActive ?  darken(get_key_color(this.#activeNotes.get(note)),0.8): 0x000000;
    }
    if(is_rewrite) {
      ctx.clearRect(x,y,key_width, key_height)
    }
    let draw_canvas = () => {
      ctx.fillStyle = into_color_code(color);
      ctx.fillRect(x, y, key_width, key_height);
      ctx.strokeStyle = 'black';
      ctx.strokeRect(x, y, key_width, key_height);
      if (key === 0) {
        ctx.fillStyle = "black";
        ctx.font = "14px Arial";
        ctx.fillText(
          `C${octave+1}`,
          x + 5,
          y + WHITE_KEY_HEIGHT - 5,
        );
      }
    };
    if(is_natural(key)) {
      draw_canvas();
      // let next_is_changed = this.#change_keys.has(note + 1);
      // let prev_is_changed = this.#change_keys.has(note - 1);
      // if (!is_rewrite) return;
      // if(!next_is_changed && is_accidental((key + 1) % 12)) {
      //   this.#draw_key(note - 1);
      // }
      // if(!prev_is_changed && is_accidental((key - 1) % 12)) {
      //   this.#draw_key(note - 1);
      // }
    } else {
      // 描画の遅延実行
      queueMicrotask(draw_canvas);
    }
  }
  #init_draw() {
    for(let i = 24,_end = this.#num_octaves*12+i; i < _end; i++) {
      this.#draw_key(i,false);
    }
  }
  #animate() {
    if (this.#canvas_is_update_frame && this.#change_keys.size) {
      // 再描画
      this.#change_keys.forEach(this.#draw_key.bind(this));
    }
    this.#change_keys.clear();
    this.#canvas_is_update_frame = !this.#canvas_is_update_frame;
    requestAnimationFrame(this.#animate.bind(this));
  }
  updatePianoRoll(msg) {
    if(msg.is_end()) {
      this.reset();
      return;
    }
    if (!msg.is_key_event()) return;
    const ch = msg.get_channel();
    const { key, vel } = msg.get_key_vel();
    if (vel !== 0) {
      this.noteOn(ch, key);
    } else if (vel === 0) {
      this.noteOff(ch, key);
    }
  }
  noteOn(ch,note) {
    if (this.#activeNotes.has(note)) {
      this.#activeNotes.get(note).add(ch);
    }
    this.#activeNotes.set(note, new Set([ch]));
    this.#change_keys.add(note);
  }

  noteOff(ch,note) {
    if (!this.#activeNotes.has(note)) return;
    console.log(note,ch);
    const set = this.#activeNotes.get(note);
    set.delete(ch);
    if (set.size === 0) {
      this.#activeNotes.delete(note);
    }
    this.#change_keys.add(note);
  }
  reset() {

  }
  draw() {
    this.#init_draw();
    this.#animate_id = requestAnimationFrame(this.#animate.bind(this))
  }
  stop_draw() {
    // TODO: impl
  }
}