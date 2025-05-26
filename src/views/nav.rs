use hypertext::prelude::*;

#[component]
pub fn nav<'a>(selected: &'a str, oob: bool) -> impl Renderable {
    let routes = [
        ("Home", "/"),
        ("About", "/about"),
        ("Projects", "/list"),
        ("Snake", "/snake"),
    ];

    rsx! {
        <nav id="nav" class="text-text border-b border-b-blue" hx-swap-oob=(oob)>
            <ul class="flex flex-row items-center justify-center">
                @for (name, path) in routes {
                    <a
                        href=(path)
                        class={
                            "flex items-center justify-center w-20 p-2 font-bold hover:rounded-t"
                            @if path == selected {
                                " bg-blue rounded-t text-crust hover"
                            } @else {
                                " text-text hover:rounded-t hover:bg-lavender hover:text-crust"
                            }
                        }
                        hx-get=(path)
                        hx-target="#page"
                        hx-swap="innerHTML"
                        hx-push-url="true"
                    >
                        <li>
                            (name)
                        </li>
                    </a>
                }
            </ul>
        </nav>
    }
}
