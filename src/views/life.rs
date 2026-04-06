use hypertext::Raw;
use hypertext::prelude::*;

const LIFE_JS: Raw<&'static str> = Raw(include_str!("life.js"));

#[component]
pub fn life() -> impl Renderable {
    rsx! {
        <script type="module">
            (LIFE_JS)
        </script>
        <div
            class="bg-blue rounded justify-center text-center flex flex-col mx-auto m-6 p-6 lg:w-1/2"
        >
            <h1 class="text-4xl lg:text-6xl text-crust flex-none">
                <b>"Conway's Game of Life"</b>
            </h1>
        </div>
        <div class="w-full h-full flex flex-col items-center p-4 box-border">
            <div id="controls" class="flex flex-1 w-full mx-auto justify-center items-center mb-4">
              <input id="game-of-life-size" type="number" value="64" min="1" max="1000" step="1" class="bg-white p-2 text-crust rounded mx-2">
              <button id="new-game" class="rounded p-2 bg-green text-crust">New Game</button>
            </div>
            <div id="canvas-container" class="overflow-auto w-full flex-1 flex justify-center items-center">
              <canvas id="game-of-life-canvas"></canvas>
            </div>

            <canvas class="block" id="game-of-life-canvas"></canvas>
        </div>
    }
}
