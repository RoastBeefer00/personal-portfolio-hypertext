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
            <div class="bg-blue rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">Linux Rice</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-auto">
                        <img src="desktop.png" class="object-scale-down mb-2" />
                    </div>
                    <div class="lg:flex-auto lg:ml-2 mb-2">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold underline">
                                "NOTE:"
                            </h2>
                            <p class="text-crust bg-text p-2 rounded">
                                "I have not contributed to EndeavorOS, Hyprland, Tmux, or
                                Helix. I simply use them in my workflow and have
                                customized them to my liking."
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">EndeavorOS</h2>
                            <p class="text-crust bg-text p-2 rounded">
                                <a href="https://endeavouros.com/" class="underline"
                                    >"EndeavorOS"</a
                                >
                                "is a Linux distribution based on Arch. It makes Arch installation
                                a little easier than vanilla Arch and comes with a few handy
                                applications pre-loaded. The Arch User Repository (AUR) and
                                package manager make it extremely simple to install any you
                                might need."
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">Hyprland</h2>
                            <p class="text-crust bg-text p-2 rounded">
                                "Here's my"
                                <a
                                    href="https://github.com/RoastBeefer00/hyprland"
                                    class="underline">"Hyprland config"</a
                                >
                                ". There's simply no going back after getting used to a
                                tiling window manager. Knowing how to navigate your
                                operating system is crucial. With this setup I never
                                wonder where my terminal(s) or browser are. Everything
                                is at my finger tips within a few keypresses away."
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">Tmux</h2>
                            <p class="text-crust bg-text p-2 rounded">
                                <a
                                    href="https://github.com/tmux/tmux/wiki"
                                    class="underline">"Tmux"</a
                                >
                                "is a terminal multiplexer that allows me to quickly and easily
                                switch between projects. My"
                                <a
                                    href="https://github.com/RoastBeefer00/tmux-conf"
                                    class="underline"
                                    >"config"
                                </a>
                                "contains some useful plugins that customize the way
                                it looks and restores my sessions so I can pick back up
                                right where I left off."
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2">
                            <h2 class="text-xl font-bold text-text">Helix</h2>
                            <p class="text-crust bg-text p-2 rounded">
                                "Helix doesn't require much "
                                <a
                                    href="https://github.com/RoastBeefer00/helix-config"
                                    class="underline">configuration</a
                                >
                                ", however I do like to change some keybinds to match
                                what I found to be more intuitive in Neovim."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="bg-rosewater rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">Cruces Chess Club</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-none lg:mr-2 mb-2 lg:w-1/3">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold">CCC</h2>
                            <p class="text-crust bg-yellow p-2 rounded">
                                "As a member of the board for my local chess club I took
                                it upon myself to build a "
                                <a
                                    href="https://www.cruceschessclub.org/"
                                    class="underline"
                                    >website
                                </a>
                                "so anyone can easily see where we'll be each week."
                                <br />
                                "The frontend is built using Svelte and the"
                                <a href="https://bulma.io/">Bulma </a>
                                "CSS framework. If I could go back I'd use TailwindCSS because
                                that is what I'm more comfortable with now and I find it
                                to be far more customizable. I learned a lot on this project
                                and it's the first time I ever registered a domain!"
                                <br />
                                "Check out my"
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
                                    href="https://roastbeefer00.github.io/sycamore-test/"
                                    class="underline">"website"</a
                                >
                                "to easily pick a few meals each week and list all the ingredients
                                to buy. I can "reroll" recipes I don't want, search by title
                                keyword or ingredient, and set time constraints if I want
                                a quick meal!"
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">Sycamore</h2>
                            <p class="text-crust bg-lavender p-2 rounded">
                                <a
                                    href="https://sycamore-rs.netlify.app/"
                                    class="underline"
                                    >Sycamore
                                </a>
                                "is a web framework that allows websites to be 100% using
                                Rust! While I still prefer Svelte, I simply couldn't pass
                                up the opportunity to use Rust more."
                                <br />
                                "Check out my"
                                <a
                                    href="https://github.com/RoastBeefer00/sycamore-test"
                                    class="underline">source code</a
                                >
                                "!"
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
            <div class="bg-pink rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">Fuzzy Recipes</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-auto lg:mr-2 mb-2">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl font-bold text-text">Skim</h2>
                            <p class="text-crust bg-flamingo p-2 rounded">
                                "Why would you ever leave the terminal?"
                                <br />
                                "Why go to a website when you have a"
                                <a
                                    href="https://github.com/RoastBeefer00/fuzzy-recipes"
                                    class="underline">CLI</a
                                >
                                "to grab a recipe?"
                                <br />
                                "This CLI is written in Rust and utilizes a crate called"
                                <a
                                    href="https://github.com/lotabout/skim"
                                    class="underline">Skim</a
                                >
                                ", which is an extremely fast fuzzy finder also written in Rust."
                            </p>
                        </div>
                    </div>
                    <div class="lg:flex-auto">
                        <img src="fuzzy.png" class="object-scale-down lg:float-right" />
                    </div>
                </div>
            </div>
            <div class="bg-green rounded p-4 mb-2">
                <div class="rounded bg-surface0 p-2 mb-2 text-center">
                    <h1 class="text-3xl text-text font-bold">TODO App</h1>
                </div>
                <div class="lg:flex">
                    <div class="lg:flex-auto mb-2">
                        <img src="todo.png" class="object-scale-down lg:float-left" />
                    </div>
                    <div class="lg:flex-none lg:ml-2 mb-2 lg:w-1/3">
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold">Learning</h2>
                            <p class="text-crust bg-teal p-2 rounded">
                                "Everyone starts somewhere and sometimes you need a
                                simple concept to learn something new and hone your
                                skills."
                                <br />
                                "For me that was this"
                                <a
                                    href="https://todo-app-web-rho.vercel.app/"
                                    class="underline">TODO app</a
                                >
                                "."
                            </p>
                        </div>
                        <div class="rounded bg-surface1 p-2 mb-2">
                            <h2 class="text-xl text-text font-bold">WASM</h2>
                            <p class="text-crust bg-teal p-2 rounded">
                                "What makes this project cool is the fact that it
                                utilizes Rust code compiled to web-assembly (WASM) so it
                                can run directly in the browser! No separate backend
                                required!"
                                <br />
                                "This is also the first project I used browser local storage
                                so data is saved between page refreshes. Sure using Rust
                                to store and manipulate state in local storage may not be
                                the most efficient method, but that's not the point. It's
                                super cool. WASM is the future!"
                                <br />
                                "Check out my"
                                <a
                                    href="https://github.com/RoastBeefer00/todo-app"
                                    class="underline">source code</a
                                >
                                "!"
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
