import init, { Universe } from "/game-of-life/conway_game_of_life_rs.js";

const SCALE = 4;
const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext('2d');

let universe, width, height, ptr, memory;
let active = false;
let animFrame = null;

// Init WASM eagerly so it's ready when needed
const wasm = await init();
memory = wasm.memory;

const setup = () => {
  const size = parseInt(document.getElementById("game-of-life-size").value);
  universe = Universe.new(size * 2, size);
  width = universe.width();
  height = universe.height();
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * SCALE}px`;
  canvas.style.height = `${height * SCALE}px`;
  ptr = universe.pixel_buffer_ptr();
};

const drawCells = () => {
  universe.render();
  const pixels = new Uint8ClampedArray(memory.buffer, ptr, width * height * 4);
  ctx.putImageData(new ImageData(pixels, width, height), 0, 0);
};

const renderLoop = () => {
  if (!active) return;
  universe.tick();
  drawCells();
  animFrame = requestAnimationFrame(renderLoop);
};

const start = () => {
  if (animFrame) cancelAnimationFrame(animFrame);
  setup();
  drawCells();
  animFrame = requestAnimationFrame(renderLoop);
};

window.addEventListener('life-impl-change', (e) => {
  active = e.detail === 'wasm';
  if (active) start();
  else if (animFrame) cancelAnimationFrame(animFrame);
});

document.getElementById("new-game").addEventListener("click", () => {
  if (active) start();
});
