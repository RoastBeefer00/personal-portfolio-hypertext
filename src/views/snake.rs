use hypertext::Raw;
use hypertext::prelude::*;

#[component]
pub fn snake() -> impl Renderable {
    rsx! {
        (Raw(
            // JS
            r#"<script type="module">
                import init from "/wasm/bevy_snake.js";
                init().catch((error) => {
                    if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
                        throw error;
                    }
                })
                const elementToRemove = document.getElementById('bevy-loading');
                elementToRemove.remove();
            </script>"#
        ))
        <div class="flex justify-center items-center text-center text-text mt-14" id="bevy-loading">
            <span>"Loading..."</span>
        </div>
        <canvas class="mt-14 mx-auto w-[800px] h-[800px]" id="snake-canvas"></canvas>
    }
}
