use hypertext::prelude::*;

#[component]
pub fn snake() -> impl Renderable {
    rsx! {
        <canvas class="mt-14 mx-auto w-[800px] h-[800px]" id="snake-canvas"></canvas>
    }
}
