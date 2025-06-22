use hypertext::prelude::*;

use crate::views::nav::Nav;

use super::Page;

#[component]
pub fn document<R: Renderable>(selected: Page, children: &R) -> impl Renderable {
    rsx! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Jacob Jasmin"</title>
                <meta charset="UTF-8">
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1.0"
                >
                <script src="https://unpkg.com/htmx.org@2"></script>
                <link rel="stylesheet" href="/styles.css">
            </head>
            <body class="bg-base text-gray-100">
                // <h1 class="flex text-5xl mx-auto font-bold justify-center items-center mb-2">Hypertext</h1>
                <Nav selected=selected oob=false />
                <div id="page" class="mt-2">
                    (children)
                </div>
            </body>
        </html>
    }
}
