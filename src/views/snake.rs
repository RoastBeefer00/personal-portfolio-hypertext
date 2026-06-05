use hypertext::Raw;
use hypertext::prelude::*;

const SNAKE_JS: Raw<&'static str> = Raw::dangerously_create(include_str!("snake.js"));

#[component]
pub fn snake() -> impl Renderable {
    rsx! {
        <script type="module">
            (SNAKE_JS)
        </script>
        <div class="bg-blue rounded-xl shadow-lg text-center mx-auto m-6 p-6 lg:w-1/2">
            <h1 class="text-4xl lg:text-6xl text-crust font-bold tracking-tight">
                "Snake"
            </h1>
            <p class="text-crust mt-3 opacity-90 leading-relaxed">
                "I made this game using the Rust game engine Bevy.  It was my first attempt at ever creating a game.
                I learned a lot about how Bevy's ECS works and how I could use it to create simple game rules."
            </p>
        </div>
        <div class="flex justify-center items-center text-center text-text mt-8" id="bevy-loading">
            <span class="text-lg opacity-75">"Loading..."</span>
        </div>
        <div class="mx-auto w-fit mt-6 mb-6 rounded-xl overflow-hidden shadow-lg">
            <canvas class="block w-[800px] h-[800px]" id="snake-canvas" width="800" height="800"></canvas>
        </div>
    }
}
