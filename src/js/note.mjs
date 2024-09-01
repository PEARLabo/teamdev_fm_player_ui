const DISTANCE_LUT = [
  0,0, 1,1, 2, 3,2, 4,3, 5,4, 6,
];
const NATURAL_LUT = [
  true,false,true,false,true,true,false,true,false,true,false,true,
];
const NOTE_NAME = [
  "C","C#","D","D#","E","F","F#","G","G#","A","A#","B"
];
/**
 * 
 * @param {number} note 
 * @returns {boolean} true if note is natural
 */
export function is_natural(note) {
  if (note >= 12) note %= 12;
  else if (note < 0) note += 12;
  return NATURAL_LUT[note];
}
/**
 * 
 * @param {number} note 
 * @returns {number} distance from C stepped by  natural note
 */
export function distance_from_c(note) {
  if (note >= 12) note %= 12;
  else if (note < 0) note += 12;
  return DISTANCE_LUT[note];
}
/**
 * 
 * @param {number} note 
 * @returns {number} distance from C# stepped by accidental note
 */
export function distance_from_c_sharp(note) {
  if (note >= 12) note %= 12;
  else if (note < 0) note += 12;
  return DISTANCE_LUT[note];
}
/**
 * 
 * @param {number} note 
 * @returns {boolean} true if note is accidental
 */
export function is_accidental(note) {
  if (note >= 12) note %= 12;
  else if (note < 0) note += 12;
  return !NATURAL_LUT[note];
}

export default class Note {
  #note_number;
  #octave;
  #interval;
  #velocity;
  constructor(note,vel) {
    let div = note / 12;
    let mod = note % 12;
    this.#note_number = note;
    this.#octave = (div|0) - 1;
    this.#interval = mod;
    this.#velocity = vel;
  }
  /**
   * @returns {number}
   */
  get note_number() {
    return this.#note_number;
  }
  get octave() {
    return this.#octave;
  }
  get note_name() {
    return NOTE_NAME[this.#interval] + this.#octave;
  }
  /**
   * @returns {number} Intervals with C as the base
   */
  get interval() {
    return this.#interval;
  }
  is_key_on() {
    return this.#velocity !== 0;
  }
  is_natural() {
    return NATURAL_LUT[interval];
  }
}