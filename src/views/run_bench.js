import { Universe as JsUniverse } from "/game_of_life.js";
import init, { run_benchmark as runBenchMarkWasm } from "/game-of-life/conway_game_of_life_rs.js";

const wasm = await init();

const status = document.getElementById('bench-status');

const Implementation = Object.freeze({
  Js: 'js',
  Wasm: 'wasm',
});

class BenchmarkResult {
  constructor(total, avg, min, max, tps) {
    this.total = total;
    this.avg = avg;
    this.min = min;
    this.max = max;
    this.tps = tps;
  }
}

function runBenchMarkJs(size, cycles) {
  const universe = new JsUniverse(size * 2, size);
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

  return new BenchmarkResult(total, avg, min, max, tps);
}

function runBench(implementation, size, cycles) {
  if (implementation === Implementation.Js) {
    return runBenchMarkJs(size, cycles);
  } else {
    return runBenchMarkWasm(size, cycles);
  }
};

const fmt = (n, decimals = 2) => n.toFixed(decimals);
const fmtTime = (ms) => {
  if (ms >= 1000) return `${(ms / 1000).toFixed(2)} s`;
  return `${ms.toFixed(2)} ms`;
};

const displayResults = (prefix, results) => {
  document.getElementById(`${prefix}-total`).textContent = fmtTime(results.total);
  document.getElementById(`${prefix}-avg`).textContent = fmtTime(results.avg);
  document.getElementById(`${prefix}-tps`).textContent = fmt(results.tps);
  document.getElementById(`${prefix}-min`).textContent = fmtTime(results.min);
  document.getElementById(`${prefix}-max`).textContent = fmtTime(results.max);;
}

document.getElementById('bench-run').addEventListener('click', async () => {
  document.getElementById(`js-total`).textContent = '-';
  document.getElementById(`js-avg`).textContent = '-';
  document.getElementById(`js-tps`).textContent = '-';
  document.getElementById(`js-min`).textContent = '-';
  document.getElementById(`js-max`).textContent = '-';
  document.getElementById(`wasm-total`).textContent = '-';
  document.getElementById(`wasm-avg`).textContent = '-';
  document.getElementById(`wasm-tps`).textContent = '-';
  document.getElementById(`wasm-min`).textContent = '-';
  document.getElementById(`wasm-max`).textContent = '-';
  const size = parseInt(document.getElementById('bench-size').value);
  const cycles = parseInt(document.getElementById('bench-cycles').value);
  const btn = document.getElementById('bench-run');

  btn.disabled = true;
  btn.classList.replace('bg-green', 'bg-surface1');

  // JS — runs synchronously but yield first so the browser can repaint
  status.textContent = 'Running JS...';
  await new Promise(r => setTimeout(r, 0));
  const jsResults = runBench(Implementation.Js, size, cycles);
  displayResults('js', jsResults);

  // WASM
  status.textContent = 'Running WASM...';
  await new Promise(r => setTimeout(r, 0));
  const wasmResults = runBench(Implementation.Wasm, size, cycles);
  displayResults('wasm', wasmResults);

  status.textContent = 'Done.';
  btn.disabled = false;
  btn.classList.replace('bg-surface1', 'bg-green');
});
