use hypertext::prelude::*;

#[component]
pub fn stack_card<'a, R: Renderable>(
    title: &'a str,
    link: &'a str,
    img_url: &'a str,
    bg_color: &'a str,
    children: &R,
) -> impl Renderable {
    rsx! {
        <div class={ "rounded-xl p-3 shadow-lg shadow-crust m-0 transition-transform hover:scale-[1.02] " (bg_color) }>
            <div class="flex items-center border-b-2 border-crust pb-2 mb-2">
                <h2 class="flex-auto text-crust font-bold text-3xl tracking-tight">
                    <a
                        href=(link)
                        class="underline-offset-2 hover:underline-offset-4 transition-all"
                    >
                        (title)
                    </a>
                </h2>
                <img
                    src=(img_url)
                    alt="img"
                    class="max-h-12 max-w-12 flex-none"
                />
            </div>
            <div class="text-crust text-sm leading-relaxed space-y-2">
                (children)
            </div>
        </div>
    }
}
