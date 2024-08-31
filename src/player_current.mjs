import { into_color_code } from "./color.mjs";
import { is_natural,is_accidental, distance_from_c,distance_from_c_sharp } from "./musical_scale.mjs";
const CANVAS_WIDTH = 900;
const CANVAS_HEIGHT = 400;

const TIME_DISPLAY_HEIGHT = 20;
const PIANO_HEIGHT = (CANVAS_HEIGHT - TIME_DISPLAY_HEIGHT) / 2;
const WHITE_KEY_WIDTH = CANVAS_WIDTH / 21;
const WHITE_KEY_HEIGHT = PIANO_HEIGHT;
const BLACK_KEY_WIDTH = WHITE_KEY_WIDTH * 0.6;
const BLACK_KEY_HEIGHT = WHITE_KEY_HEIGHT * 0.6;

const COLOR_LUT = [0xf08080, 0xfffacd, 0xe6e6fa, 0xd6efff, 0xcfffe5, 0xffd1dc];
// NO EDIT!
const MOD3_LUT = [0, 1, 2, 0];
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
  #animate_id;
  constructor(id) {
    this.#canvas = document.getElementById(id);
    this.#canvas.height = CANVAS_HEIGHT;
    this.#canvas.width = CANVAS_WIDTH;
  }

  #draw_key(note,is_rewrite = true) {
    let ctx = this.#canvas.getContext("2d");
    const octave = Math.floor((note - 24) / 12);
    const key = (note - 24) % 12;
    const octaveOffsetX = MOD3_LUT[octave & 3] * 7 * WHITE_KEY_WIDTH;
    const yOffset = Math.floor(octave / 3) * PIANO_HEIGHT;
    let x;
    let i;
    let key_width;
let key_height;
    const y = yOffset;
    let isActive = this.#activeNotes.has(note);
    let color;
    if (is_natural(key)) {
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
    if(true) {
      ctx.clearRect(0, CANVAS_HEIGHT - TIME_DISPLAY_HEIGHT,x,y)
    }
    let draw_canvas = () => {
      if (!is_natural(key)) {
        console.log(into_color_code(color));
      }
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
        console.log(octave,octave & 3,x,y)
      }
    };
    if(is_natural(key)) {
      draw_canvas();
    } else {
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
      this.#change_keys.forEach(this.#draw_key)
    }
    this.#change_keys.clear();
    canvas_is_update_frame = !canvas_is_update_frame;
    requestAnimationFrame(this.#animate);
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
    this.#change_keys.set(note);
  }

  noteOff(ch,note) {
    if (!this.#activeNotes.has(note)) return;
    const set = this.#activeNotes.get(note);
    set.delete(ch);
    if (set.size === 0) {
      activeNotes.delete(note);
    }
    this.#change_keys.set(note);
  }
  reset() {

  }
  draw() {
    this.#init_draw();
    this.#animate_id = requestAnimationFrame(this.#animate)
  }
  stop_draw() {
    // TODO: impl
  }
}