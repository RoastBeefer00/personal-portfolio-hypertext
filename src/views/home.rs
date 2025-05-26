use crate::views::StackCard;
use hypertext::prelude::*;

#[component]
pub fn home() -> impl Renderable {
    rsx! {
    <div class="m-2">
        <div
            class="bg-blue rounded justify-center text-center flex mx-auto m-6 p-6 w-5/6 lg:w-1/2 text-crust"
        >
            <div class="flex-auto">
                <h1 class="text-2xl lg:text-4xl">"Hi, I'm"</h1>
                <h1 class="text-4xl lg:text-6xl flex-none">
                    <b>Jake Jasmin</b>
                </h1>
                <p class="text-crust mt-5">"I'm in all the movies!"</p>
            </div>
            <img
                src="jake.png"
                class="rounded-full max-w-40 max-h-40 float-right flex-none"
            />
        </div>

        <div
            class="rounded bg-surface0 lg:p-4 mx-auto justify-center lg:w-5/6 pt-2"
        >
            <div
                class="justify-center text-center bg-blue rounded w-2/3 lg:w-1/3 mx-auto mb-2"
            >
                <h1 class="text-crust text-2xl font-bold p-2">My tech stack</h1>
            </div>
            <div class="justify-center mx-auto grid grid-cols-1 lg:grid-cols-3">
                <StackCard
                    title="Rust"
                    link="https://www.rust-lang.org/"
                    bg_color="bg-red"
                    img_url="rust.png"
                >
                    <p>
                        "Currently addicted to Rust. I "
                        <i>"love"</i>
                        " creating CLIs, TUIs,
                        and websites with Rust! At work I've created multiple CLIs in
                        Rust to help automate tasks and make key information more easily
                        accessible. Most recently I created the game "snake" using the Rust game 
                        engine "
                        <a href="https://bevyengine.org/" class="underline">"Bevy"</a>
                        "!"
                    </p>
                </StackCard>
                <StackCard
                    title="Go"
                    link="https://go.dev/"
                    bg_color="bg-blue"
                    img_url="go.png"
                >
                    <p>
                        "Go is an awesome language and I'm very glad I learned it. I use Go at
                        work as my scripting language of choice.  It has great GCP integration and 
                        the type system can be quite powerful.  Go can be the right tool for the 
                        job when you need fearless concurrency or maybe when you want to prototype something
                        a little faster than Rust."
                    </p>
                </StackCard>
                <StackCard
                    title="Linux"
                    link="https://archlinux.org/"
                    bg_color="bg-rosewater"
                    img_url="linux.png"
                >
                    <p>
                        "Linux is home. I use Linux for ALL of my personal computers
                        and I really wish I could use Linux at work. I've tried many
                        different distros and desktop environments but I always come
                        back to the "
                        <a
                            href="https://hyprland.org/"
                            class="underline"
                            >"Hyprland"
                        </a>
                        " window manager on Arch linux."
                    </p>
                </StackCard>
                <StackCard
                    title="Neovim"
                    link="https://neovim.io/"
                    bg_color="bg-green"
                    img_url="neovim.png"
                >
                    <p>
                        "Neovim is my text editor of choice. It is a modal editor that
                        utilizes vim motions, is configured using lua, and has amazing plugins.
                        Neovim gives me complete control over my development environment and allows me 
                        to set up everything exactly how I want it."
                    </p>
                    <p>
                        "If you leave the terminal or touch your mouse you've already
                        lost!"
                    </p>
                </StackCard>
                <StackCard
                    title="Svelte"
                    link="https://svelte.dev/"
                    bg_color="bg-peach"
                    img_url="svelte.png"
                >
                    <p>
                        "Svelte has been my main tool for learning web design. I find
                        it to be very intuitive and simple to use. TailwindCSS is
                        the CSS framework I've grown most used to in combination
                        with Svelte."
                    </p>
                    <br />
                    <p>
                        "I've combined Svelte with a Python FastAPI as well as a Rust
                        backend compiled to WebAssembly."
                    </p>
                </StackCard>
                <StackCard
                    title="Terraform"
                    link="https://www.terraform.io/"
                    bg_color="bg-lavender"
                    img_url="terraform.png"
                >
                    <p>
                        "I utilize Terraform at work to deploy and manage
                        infrastructure in Google Cloud Platform."
                    </p>
                    <p>
                        "I have experience using both Terraform Enterprise (TFE) and
                        Terraform Open Source (TFO)."
                    </p>
                </StackCard>
            </div>
        </div>
    </div>
        }
}
