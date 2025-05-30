use hypertext::prelude::*;

#[component]
pub fn projects() -> impl Renderable {
    rsx! {
        <div class="w-3/4 lg:w-1/2 bg-blue justify-center mx-auto m-4 rounded p-4">
            <h1 class="text-crust text-4xl lg:text-6xl text-center">
                <b>Projects</b>
            </h1>
            <p class="text-crust text-center m-2">
                "Check out my "
                <a
                    href="https://github.com/RoastBeefer00?tab=repositories"
                    class="underline">"Github"</a
                >
                "!"
            </p>
        </div>

        <div class="p-4">
            <div class="bg-mauve rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">Meal Prep</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-auto">
                        <img src="magentacarrot.png" class="object-scale-down mb-2" />
                    </div>
                    <div class="lg:flex-auto lg:ml-2 mb-2">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">Magenta Carrot</h2>
                            <p class="text-crust bg-lavender p-2 rounded">
                                "Learning to cook changed my life. Eating healthy is so
                                important and overcoming the obstacle of learning to
                                cook is one of the best things I've ever done. I created
                                this "
                                <a
                                    href="https://recipes.jacobjasmin.com"
                                    class="underline">"website"</a
                                >
                                " to easily pick a few meals each week and list all the ingredients
                                to buy. I can "reroll" recipes I don't want, search by title
                                keyword or ingredient, and set time constraints if I want
                                a quick meal!"
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">Templ</h2>
                            <p class="text-crust bg-lavender p-2 rounded">
                                <a
                                    href="https://templ.guide/"
                                    class="underline"
                                >
                                    Templ
                                </a>
                                " is a Go library that allows you to create HTML templates while also
                                using Go syntax for things like control logic and making re-usable components
                                that take in Go types as parameters! Using the backend to generate the HTML 
                                allowed me to use server-side Google OAUTH 2.0 to sign users in and store their site data
                                in Firestore." 
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">TailwindCSS</h2>
                            <p class="text-crust bg-lavender p-2 rounded">
                                "This is the first project where I primarily used "
                                <a
                                    href="https://tailwindcss.com/"
                                    class="underline">"TailwindCSS"</a
                                >
                                "for styling. It has become my primary framework because
                                I find it to have a relatively easy learning curve while
                                still providing tons of customization."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="bg-rosewater rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">"Cruces Chess Club - Website"</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-none lg:mr-2 mb-2 lg:w-1/3">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold">CCC</h2>
                            <p class="text-crust bg-yellow p-2 rounded bg-text">
                                "As a member of the board for my local chess club I took
                                it upon myself to build a "
                                <a
                                    href="https://www.cruceschessclub.org/"
                                    class="underline"
                                    >website
                                </a>
                                " so anyone can easily see where we'll be each week."
                                <br />
                                "The frontend is built using Svelte and the"
                                <a href="https://bulma.io/">Bulma </a>
                                "CSS framework. If I could go back I'd use TailwindCSS because
                                that is what I'm more comfortable with now and I find it
                                to be far more customizable. I learned a lot on this project
                                and it's the first time I ever registered a domain!"
                                <br />
                                "Check out my "
                                <a
                                    href="https://github.com/RoastBeefer00/cruceschessclub"
                                    class="underline">"source code"</a
                                >
                                "!"
                            </p>
                        </div>
                    </div>
                    <div class="lg:flex-auto">
                        <img src="ccc.png" class="object-scale-down" />
                    </div>
                </div>
            </div>
            <div class="bg-blue rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">"Cruces Chess Club - Mobile App"</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-none">
                        <img src="ccc-mobile.png" class="object-scale-down max-h-160 mb-2 lg:mb-0" />
                    </div>
                    <div class="flex flex-col mb-2 lg:w-full lg:m-2">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold">CCC</h2>
                            <p class="text-crust p-2 rounded bg-text">
                                "I also built a mobile app for my chess club so that we can send
                                push notifications to users letting them know about upcoming events."
                                <br />
                                "The application is built using "
                                <a href="https://flutter.dev/" class="underline">Flutter</a>
                                ".  This is the first time I've ever used Flutter, so I had to teach myself
                                the basics."
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold">Admin Site</h2>
                            <div>
                                <p class="p-2 bg-text text-crust">
                                    "I also made an admin site that allows board members to send push notifications with ease."
                                    <br />
                                    "The site is made using "
                                    <a href="https://leptos.dev/" class="underline">Leptos</a>
                                    "."
                                </p>
                            </div>
                        </div>
                        <img src="ccc-admin.png" class="object-scale-down" />
                    </div>
                </div>
            </div>
            <div class="bg-green rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">RJMATRIX</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-auto lg:mr-2 mb-2">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">The Matrix</h2>
                            <p class="text-crust bg-teal p-2 rounded">
                                "Doesn't everyone want to have The Matrix in their terminal?
                                I certainly do, so I decided to learn how to make it myself in Rust! 
                                I used the "
                                <a href="https://ratatui.rs/" class="underline">ratatui</a>
                                " crate and learned a lot about TUIs (terminal user interfaces) in the process."
                            </p>
                        </div>
                    </div>
                    <div class="lg:flex-auto">
                        <img src="rmatrix.gif" class="object-scale-down lg:float-right" />
                    </div>
                </div>
            </div>
        </div>
    }
}
