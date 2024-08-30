function color_mixer(colors) {
  // console.log(colors)
  let is_array = Array.isArray(colors);
  if (!is_array) {
    return colors;
  }
  if (colors.length === 1) {
    return colors[0];
  }
  let alpha = 1 / colors.length;
  let r = 0,
    g = 0,
    b = 0;
  for (let i in colors) {
    r += colors[i][0] * alpha;
    g += colors[i][1] * alpha;
    b += colors[i][2] * alpha;
  }
  return (r << 16) | (g << 8) | b;
}
function into_color_code(color) {
  // console.log(`#${color.toString(16)}`)
  return `#${color.toString(16)}`;
}

function darken(color, darkness) {
  if (darkness >= 1) return color;
  if (darkness <= 0) return 0;
  let r = (color >> 16) & 255;
  let g = (color >> 8) & 255;
  let b = color & 255;
  return ((r * darkness) << 16) | ((g * darkness) << 8) | (b * darkness);
}
