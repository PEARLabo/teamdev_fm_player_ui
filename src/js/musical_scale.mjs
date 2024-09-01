const DISTANCE_LUT = [
  0,0, 1,1, 2, 3,2, 4,3, 5,4, 6,
];
const NATURAL_LUT = [
  true,false,true,false,true,true,false,true,false,true,false,true,
];
/**
 * 
 * @param {number} note 
 * @returns {boolean} true if note is natural
 */
export function is_natural(note) {
  return NATURAL_LUT[note];
}
/**
 * 
 * @param {number} note 
 * @returns {number} distance from C stepped by  natural note
 */
export function distance_from_c(note) {
  return DISTANCE_LUT[note];
}
/**
 * 
 * @param {number} note 
 * @returns {number} distance from C# stepped by accidental note
 */
export function distance_from_c_sharp(note) {
  return DISTANCE_LUT[note];
}
/**
 * 
 * @param {number} note 
 * @returns {boolean} true if note is accidental
 */
export function is_accidental(note) {
  return !NATURAL_LUT[note];
}
