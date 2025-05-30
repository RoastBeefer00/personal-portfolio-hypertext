use hypertext::Raw;
use hypertext::prelude::*;

const SNAKE_JS: Raw<&'static str> = Raw(include_str!("snake.js"));

#[component]
pub fn snake() -> impl Renderable {
    rsx! {
        <script type="module">
            (SNAKE_JS)
        </script>
        <div
            class="bg-blue rounded justify-center text-center flex mx-auto m-6 p-6 lg:w-1/2"
        >
            <h1 class="text-4xl lg:text-6xl text-crust flex-none">
                <b>Snake</b>
            </h1>
        </div>
        <div class="flex justify-center items-center text-center text-text mt-14" id="bevy-loading">
            <span>"Loading..."</span>
        </div>
        <canvas class="mt-14 mx-auto w-[800px] h-[800px]" id="snake-canvas"></canvas>
    }
}
