import { Universe as JsUniverse } from "/game_of_life.js";
import init, { Universe as WasmUniverse } from "/game-of-life/conway_game_of_life_rs.js";

const wasm = await init();

const status = document.getElementById('bench-status');

const runBench = (newUniverse, cycles) => {
  const universe = newUniverse();
  const tickTimes = [];

  for (let i = 0; i < cycles; i++) {
    const t0 = performance.now();
    universe.tick();
    tickTimes.push(performance.now() - t0);
  }

  const total = tickTimes.reduce((a, b) => a + b, 0);
  const avg = total / cycles;
  const min = Math.min(...tickTimes);
  const max = Math.max(...tickTimes);
  const tps = 1000 / avg;

  return { total, avg, min, max, tps };
};

const fmt = (n, decimals = 2) => n.toFixed(decimals);
const fmtTime = (ms) => {
  if (ms >= 1000) return `${(ms / 1000).toFixed(2)} s`;
  return `${ms.toFixed(2)} ms`;
};

const displayResults = (prefix, { total, avg, min, max, tps }) => {
  document.getElementById(`${prefix}-total`).textContent = fmtTime(total);
  document.getElementById(`${prefix}-avg`).textContent = fmtTime(avg);
  document.getElementById(`${prefix}-tps`).textContent = fmt(tps);
  document.getElementById(`${prefix}-min`).textContent = fmtTime(min);
  document.getElementById(`${prefix}-max`).textContent = fmtTime(max);
};

document.getElementById('bench-run').addEventListener('click', async () => {
  const size = parseInt(document.getElementById('bench-size').value);
  const cycles = parseInt(document.getElementById('bench-cycles').value);
  const btn = document.getElementById('bench-run');

  btn.disabled = true;
  btn.classList.replace('bg-green', 'bg-surface1');

  // JS — runs synchronously but yield first so the browser can repaint
  status.textContent = 'Running JS...';
  await new Promise(r => setTimeout(r, 0));
  const jsResults = runBench(() => new JsUniverse(size * 2, size), cycles);
  displayResults('js', jsResults);

  // WASM
  status.textContent = 'Running WASM...';
  await new Promise(r => setTimeout(r, 0));
  const wasmResults = runBench(() => WasmUniverse.new(size * 2, size), cycles);
  displayResults('wasm', wasmResults);

  status.textContent = 'Done.';
  btn.disabled = false;
  btn.classList.replace('bg-surface1', 'bg-green');
});
