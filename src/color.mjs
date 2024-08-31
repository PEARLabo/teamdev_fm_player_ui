export function color_mixer(colors) {
  // console.log(colors)
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
  for (const i in colors) {
    r += colors[i][0] * alpha;
    g += colors[i][1] * alpha;
    b += colors[i][2] * alpha;
  }
  return (r << 16) | (g << 8) | b;
}
export function into_color_code(color) {
  // console.log(`#${color.toString(16)}`)
  return `#${color.toString(16).padStart(6,'0')}`;
}

export function darken(color, darkness) {
  if (darkness >= 1) return color;
  if (darkness <= 0) return 0;
  const r = (color >> 16) & 255;
  const g = (color >> 8) & 255;
  const b = color & 255;
  return ((r * darkness) << 16) | ((g * darkness) << 8) | (b * darkness);
}
