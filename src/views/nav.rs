use hypertext::prelude::*;
use strum::IntoEnumIterator;

use super::Page;

#[allow(unused_parens)]
#[component]
pub fn nav(selected: Page, oob: bool) -> impl Renderable {
    rsx! {
        <nav id="nav" class="text-text border-b border-b-blue" hx-swap-oob=true[oob]>
            <ul class="flex flex-row items-center justify-center">
                @for route in Page::iter() {
                    <li
                    >
                        <a
                            href=(route.get_ref())
                            class={
                                "flex items-center justify-center w-20 p-2 font-bold hover:rounded-t"
                                @if route == selected {
                                    " bg-blue rounded-t text-crust hover"
                                } @else {
                                    " text-text hover:rounded-t hover:bg-lavender hover:text-crust"
                                }
                            }
                            hx-get=(route.get_ref())
                            hx-target="#page"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            (route.to_string())
                        </a>
                    </li>
                }
            </ul>
        </nav>
    }
}
