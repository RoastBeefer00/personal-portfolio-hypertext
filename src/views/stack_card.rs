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
        <div class={ "rounded p-2 shadow-md shadow-crust m-2 " (bg_color) }>
            <div class="flex border-b-2 border-gray-900">
                <h2 class="flex-auto text-gray-900 font-bold text-3xl m-2">
                    <a href=(link)>(title)</a>
                </h2>
                <img
                    src=(img_url)
                    alt="img"
                    class="max-h-12 max-w-12 right-0 flex-none"
                />
            </div>
            <div class="p-2 text-gray-900">
                (children)
            </div>
        </div>
    }
}
