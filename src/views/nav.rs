use hypertext::prelude::*;
use strum::IntoEnumIterator;

use super::Page;

#[allow(unused_parens)]
#[component]
pub fn nav(selected: Page, oob: bool) -> impl Renderable {
    rsx! {
        <nav id="nav" class="bg-mantle text-text border-b-2 border-blue" hx-swap-oob=true[oob]>
            <ul class="flex flex-row items-center justify-center gap-1 px-2 pt-1">
                @for route in Page::iter() {
                    <li>
                        <a
                            href=(route.get_ref())
                            class={
                                "flex items-center justify-center px-4 py-2 rounded-t font-semibold transition-colors"
                                @if route == selected {
                                    " bg-blue text-crust"
                                } @else {
                                    " text-subtext1 hover:bg-surface0 hover:text-text"
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
