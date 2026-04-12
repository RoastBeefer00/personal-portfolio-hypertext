import { Universe } from "/game_of_life.js";

const SCALE = 4;
const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext('2d');

let universe, width, height;
let active = true; // JS is default
let animFrame = null;

const setup = () => {
  const size = parseInt(document.getElementById("game-of-life-size").value);
  universe = new Universe(size * 2, size);
  width = universe.width();
  height = universe.height();
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * SCALE}px`;
  canvas.style.height = `${height * SCALE}px`;
};

const drawCells = () => {
  universe.render();
  ctx.putImageData(new ImageData(universe.pixel_buffer(), width, height), 0, 0);
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

// Button wiring lives here since JS script loads first
document.addEventListener('DOMContentLoaded', () => {
  const jsBtn = document.getElementById('impl-js');
  const wasmBtn = document.getElementById('impl-wasm');

  function setActive(impl) {
    jsBtn.classList.toggle('bg-green', impl === 'js');
    jsBtn.classList.toggle('bg-surface1', impl !== 'js');
    wasmBtn.classList.toggle('bg-green', impl === 'wasm');
    wasmBtn.classList.toggle('bg-surface1', impl !== 'wasm');
    window.dispatchEvent(new CustomEvent('life-impl-change', { detail: impl }));
  }

  jsBtn.addEventListener('click', () => setActive('js'));
  wasmBtn.addEventListener('click', () => setActive('wasm'));
});

window.addEventListener('life-impl-change', (e) => {
  active = e.detail === 'js';
  if (active) start();
  else if (animFrame) cancelAnimationFrame(animFrame);
});

document.getElementById("new-game").addEventListener("click", () => {
  if (active) start();
});

// Auto-start as default impl
start();
