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
                <Link
                    href="https://github.com/RoastBeefer00?tab=repositories"
                    name="Github"
                />
                "!"
            </p>
        </div>

        <div class="p-4">
            <Project title="Meal Prep" bg_color="bg-mauve">
                <div class="lg:flex">
                    <div class="lg:flex-auto">
                        <img src="magentacarrot.png" class="object-scale-down mb-2" />
                    </div>
                    <div class="lg:flex-auto lg:ml-2 mb-2">
                        <TextBlock title="Magenta Carrot" bg_color="bg-lavender">
                            "Learning to cook changed my life. Eating healthy is so
                            important and overcoming the obstacle of learning to
                            cook is one of the best things I've ever done. I created
                            this "
                            <Link href="https://recipes.jacobjasmin.com" name="website" />
                            " to easily pick a few meals each week and list all the ingredients
                            to buy. I can \"reroll\" recipes I don't want, search by title
                            keyword or ingredient, and set time constraints if I want
                            a quick meal!"
                        </TextBlock>
                        <TextBlock title="Templ" bg_color="bg-lavender">
                            <Link href="https://templ.guide/" name="Templ" />
                            " is a Go library that allows you to create HTML templates while also
                            using Go syntax for things like control logic and making re-usable components
                            that take in Go types as parameters! Using the backend to generate the HTML 
                            allowed me to use server-side Google OAUTH 2.0 to sign users in and store their site data
                            in Firestore." 
                        </TextBlock>
                        <TextBlock title="TailwindCSS" bg_color="bg-lavender">
                            "This is the first project where I primarily used "
                            <Link href="https://tailwindcss.com/" name="TailwindCSS" />
                            " for styling. It has become my primary framework because
                            I find it to have a relatively easy learning curve while
                            still providing tons of customization."
                        </TextBlock>
                    </div>
                </div>
            </Project>

            <Project title="Cruces Chess Club - Website" bg_color="bg-rosewater">
                <div class="lg:flex">
                    <div class="lg:flex-none lg:mr-2 mb-2 lg:w-1/3">
                        <TextBlock title="CCC" bg_color="bg-text">
                            "As a member of the board for my local chess club I took
                            it upon myself to build a "
                            <Link href="https://www.cruceschessclub.org/" name="website" />
                            " so anyone can easily see where we'll be each week."
                            <br />
                            "The frontend is built using Svelte and the "
                            <Link href="https://bulma.io/" name="Bulma" />
                            " CSS framework. If I could go back I'd use TailwindCSS because
                            that is what I'm more comfortable with now and I find it
                            to be far more customizable. I learned a lot on this project
                            and it's the first time I ever registered a domain!"
                            <br />
                            "Check out my "
                            <Link
                                href="https://github.com/RoastBeefer00/cruceschessclub"
                                name="source code"
                            />
                            "!"
                        </TextBlock>
                    </div>
                    <div class="lg:flex-auto">
                        <img src="ccc.png" class="object-scale-down" />
                    </div>
                </div>
            </Project>

            <Project title="Cruces Chess Club - Mobile App" bg_color="bg-blue">
                <div class="lg:flex">
                    <div class="lg:flex-none">
                        <img src="ccc-mobile.png" class="object-scale-down max-h-160 mb-2 lg:mb-0" />
                    </div>
                    <div class="flex flex-col mb-2 lg:w-full lg:m-2">
                        <TextBlock title="CCC" bg_color="bg-text">
                            "I also built a mobile app for my chess club so that we can send
                            push notifications to users letting them know about upcoming events."
                            <br />
                            "The application is built using "
                            <Link href="https://flutter.dev/" name="Flutter"/>
                            ".  This is the first time I've ever used Flutter, so I had to teach myself
                            the basics."
                        </TextBlock>
                        <TextBlock title="Admin Site" bg_color="bg-text">
                            "I also made an admin site that allows board members to send push notifications with ease."
                            <br />
                            "The site is made using "
                            <Link href="https://leptos.dev/" name="Leptos" />
                            "."
                        </TextBlock>
                        <img src="ccc-admin.png" class="object-scale-down" />
                    </div>
                </div>
            </Project>

            <Project title="RJMATRIX" bg_color="bg-green">
                <div class="lg:flex">
                    <div class="lg:flex-auto lg:mr-2 mb-2">
                        <TextBlock title="The Matrix" bg_color="bg-teal">
                            "Doesn't everyone want to have The Matrix in their terminal?
                            I certainly do, so I decided to learn how to make it myself in Rust! 
                            I used the "
                            <Link href="https://ratatui.rs/" name="ratatui" />
                            " crate and learned a lot about TUIs (terminal user interfaces) in the process."
                        </TextBlock>
                    </div>
                    <div class="lg:flex-auto">
                        <img src="rmatrix.gif" class="object-scale-down lg:float-right" />
                    </div>
                </div>
            </Project>
        </div>
    }
}

#[component]
fn project<'a, R: Renderable>(title: &'a str, bg_color: &'a str, children: &R) -> impl Renderable {
    rsx! {
        <div class={ (bg_color) " rounded p-4 mb-2" }>
            <div class="rounded bg-surface0 p-2 mb-2 text-center">
                <h1 class="text-3xl text-text font-bold">
                    (title)
                </h1>
            </div>
            (children)
        </div>
    }
}

#[component]
fn text_block<'a, R: Renderable>(
    title: &'a str,
    bg_color: &'a str,
    children: &R,
) -> impl Renderable {
    rsx! {
        <div class="rounded bg-surface1 p-2 mb-2">
            <h2 class="text-xl font-bold text-text">
                (title)
            </h2>
            <p class={ (bg_color) " p-2 rounded text-crust" }>
                (children)
            </p>
        </div>
    }
}

#[component]
fn link<'a>(href: &'a str, name: &'a str) -> impl Renderable {
    rsx! {
        <a
            href=(href)
            class="underline"
        >
            (name)
        </a>
    }
}
