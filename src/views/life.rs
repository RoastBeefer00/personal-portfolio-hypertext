use hypertext::Raw;
use hypertext::prelude::*;

const RUN_LIFE_JS: Raw<&'static str> = Raw::dangerously_create(include_str!("run_life_js.js"));
const RUN_LIFE_WASM: Raw<&'static str> = Raw::dangerously_create(include_str!("run_life_wasm.js"));
const RUN_BENCH_JS: Raw<&'static str> = Raw::dangerously_create(include_str!("run_bench.js"));

#[component]
pub fn life_page() -> impl Renderable {
    rsx! {
        <div "x-data"="{ tab: 'game' }">
            <div class="flex mb-4 justify-center items-center mx-auto">
                <button
                    "x-on:click"="tab = 'game'"
                    "x-bind:class"="tab === 'game' ? 'bg-blue' : 'bg-surface1'"
                    class="rounded-tl rounded-bl p-2 px-4 text-crust"
                >
                    "Game"
                </button>
                <button
                    "x-on:click"="tab = 'bench'"
                    "x-bind:class"="tab === 'bench' ? 'bg-blue' : 'bg-surface1'"
                    class="rounded-tr rounded-br p-2 px-4 text-crust"
                >
                    "Benchmark"
                </button>
            </div>
            <div "x-show"="tab === 'game'">
                (life())
            </div>
            <div "x-show"="tab === 'bench'">
                (bench())
            </div>
        </div>
    }
}

#[component]
pub fn life() -> impl Renderable {
    rsx! {
            <script type="module">
                (RUN_LIFE_JS)
            </script>
            <script type="module">
                (RUN_LIFE_WASM)
            </script>
            <div class="w-full h-full flex flex-col items-center p-4 box-border">
                <div id="controls" class="flex flex-1 w-full mx-auto justify-center items-center mb-4">
                    <input id="game-of-life-size" type="number" value="64" min="1" max="1000" step="1" class="bg-white p-2 text-crust rounded mx-2">
                    <div class="flex rounded overflow-hidden mx-2">
    <button id="impl-js" class="w-16 p-2 px-4 bg-green text-crust">
        "JS"
    </button>
    <button id="impl-wasm" class="w-16 p-2 px-2 bg-surface1 text-crust">
        "WASM"
    </button>
                    </div>
                    <button id="new-game" class="rounded p-2 bg-green text-crust">New Game</button>
                </div>
                <div id="canvas-container" class="overflow-auto w-full flex-1 flex justify-center items-center">
                    <canvas id="game-of-life-canvas"></canvas>
                </div>
            </div>
        }
}

#[component]
pub fn bench() -> impl Renderable {
    rsx! {
        <script type="module">
            (RUN_BENCH_JS)
        </script>
        <div class="w-full h-full flex flex-col items-center p-4 box-border">
            <div class="flex flex-wrap items-center justify-center gap-4 mb-6">
                <label class="text-text flex items-center gap-2">
                    "Size"
                    <input id="bench-size" type="number" value="64" min="1" max="1000" step="1"
                        class="bg-white p-2 text-crust rounded w-24">
                </label>
                <label class="text-text flex items-center gap-2">
                    "Cycles"
                    <input id="bench-cycles" type="number" value="500" min="1" max="100000" step="1"
                        class="bg-white p-2 text-crust rounded w-28">
                </label>
                <button id="bench-run" class="rounded p-2 px-6 bg-green text-crust">
                    "Run"
                </button>
            </div>
            <div id="bench-status" class="text-text mb-4 h-4"></div>
            <div class="w-full max-w-4xl bg-surface0 rounded p-4 lg:p-8 flex flex-col lg:flex-row divide-y lg:divide-y-0 lg:divide-x divide-surface2">
                <div class="flex-1 pb-6 lg:pb-0 lg:pr-8">
                    <h2 class="text-xl text-text font-bold mb-4">"JS"</h2>
                    <table class="w-full text-text">
                        <tbody>
                            <tr><td>"Total time"</td><td id="js-total" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Avg ms/tick"</td><td id="js-avg" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Ticks/sec"</td><td id="js-tps" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Min tick"</td><td id="js-min" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Max tick"</td><td id="js-max" class="text-right font-mono">"-"</td></tr>
                        </tbody>
                    </table>
                </div>
                <div class="flex-1 pt-6 lg:pt-0 lg:pl-8">
                    <h2 class="text-xl text-text font-bold mb-4">"WASM"</h2>
                    <table class="w-full text-text">
                        <tbody>
                            <tr><td>"Total time"</td><td id="wasm-total" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Avg ms/tick"</td><td id="wasm-avg" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Ticks/sec"</td><td id="wasm-tps" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Min tick"</td><td id="wasm-min" class="text-right font-mono">"-"</td></tr>
                            <tr><td>"Max tick"</td><td id="wasm-max" class="text-right font-mono">"-"</td></tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
