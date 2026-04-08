import init, { Universe } from "/game-of-life/conway_game_of_life_rs.js";
const wasm = await init();
const memory = wasm.memory;
const SCALE = 4;

const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext('2d');

let universe, width, height, ptr;

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
  // pixels = new Uint8ClampedArray(memory.buffer, ptr, width * height * 4);
  // imageData = new ImageData(pixels, width, height);
};

const renderLoop = () => {
  universe.tick();

  drawCells();

  requestAnimationFrame(renderLoop);
};



const drawCells = () => {
  universe.render();
  const pixels = new Uint8ClampedArray(memory.buffer, ptr, width * height * 4);
  const imageData = new ImageData(pixels, width, height);
  ctx.putImageData(imageData, 0, 0);
};

document.getElementById("new-game").addEventListener("click", () => {
  setup();
  drawCells();
});

setup();
drawCells();
requestAnimationFrame(renderLoop);

