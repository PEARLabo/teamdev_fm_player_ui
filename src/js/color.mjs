/**
 *
 * @param {[number] | number} colors
 * @returns {number} mixed color
 */
export function color_mixer(colors) {
    const is_array = Array.isArray(colors);
    if (!is_array) {
        return colors;
    }
    if (colors.length === 1) {
        return colors[0];
    }
    const alpha = 1 / colors.length;
    let r = 0;
    let g = 0;
    let b = 0;
    for (const color of colors) {
        r += ((color & 0xff0000) >> 16) * alpha;
        g += ((color & 0x00ff00) >> 8) * alpha;
        b += (color & 0x0000ff) * alpha;
    }
    return (r << 16) | (g << 8) | b;
}
/**
 *
 * @param {number} color
 * @returns {String} color code
 */
export function into_color_code(color) {
    return `#${color.toString(16).padStart(6, "0")}`;
}
/**
 *
 * @param {number} color
 * @param {number} darkness
 * @returns {number} Darkened color
 */
export function darken(color, darkness) {
    if (darkness >= 1) return color;
    if (darkness <= 0) return 0;
    const r = (color >> 16) & 255;
    const g = (color >> 8) & 255;
    const b = color & 255;
    return ((r * darkness) << 16) | ((g * darkness) << 8) | (b * darkness);
}
