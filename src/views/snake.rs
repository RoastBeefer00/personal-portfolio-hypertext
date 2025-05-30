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
            class="bg-blue rounded justify-center text-center flex flex-col mx-auto m-6 p-6 lg:w-1/2"
        >
            <h1 class="text-4xl lg:text-6xl text-crust flex-none">
                <b>Snake</b>
            </h1>
            <div>
                <p class="text-crust">
                    "I made this game using the Rust game engine Bevy.  It was my first attempt at ever creating a game.
                    I learned a lot about how Bevy's ECS works and how I could use it to create simple game rules."
                </p>
            </div>
        </div>
        <div class="flex justify-center items-center text-center text-text mt-14" id="bevy-loading">
            <span>"Loading..."</span>
        </div>
        <canvas class="mt-10 mx-auto w-[800px] h-[800px] mb-4" id="snake-canvas"></canvas>
    }
}
