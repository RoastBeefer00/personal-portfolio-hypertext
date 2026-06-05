use hypertext::prelude::*;

#[component]
pub fn projects() -> impl Renderable {
    rsx! {
        <div class="w-3/4 lg:w-1/2 bg-blue justify-center mx-auto m-4 rounded-xl p-6 shadow-lg">
            <h1 class="text-crust text-4xl lg:text-6xl text-center font-bold tracking-tight">
                "Projects"
            </h1>
            <p class="text-crust text-center mt-2 opacity-90">
                "Check out my "
                <Link
                    href="https://github.com/RoastBeefer00?tab=repositories"
                    name="Github"
                />
                "!"
            </p>
        </div>

        <div class="p-4 flex flex-col gap-4">
            <Project title="Meal Prep" bg_color="bg-mauve">
                <ProjectRow image="magentacarrot.png" image_left=true>
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
                </ProjectRow>
            </Project>

            <Project title="Cruces Chess Club - Website" bg_color="bg-rosewater">
                <ProjectRow image="ccc.png" image_left=false>
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
                </ProjectRow>
            </Project>

            <Project title="Cruces Chess Club - Mobile App" bg_color="bg-blue">
                <ProjectRow image="ccc-admin.png" image_left=true>
                    <TextBlock title="Mobile App" bg_color="bg-text">
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
                    <div class="flex justify-center mt-2">
                        <img src="ccc-mobile.png" class="object-contain rounded-lg max-h-64" />
                    </div>
                </ProjectRow>
            </Project>

            <Project title="RJMATRIX" bg_color="bg-green">
                <ProjectRow image="rmatrix.gif" image_left=false>
                    <TextBlock title="The Matrix" bg_color="bg-sapphire">
                        "Doesn't everyone want to have The Matrix in their terminal?
                        I certainly do, so I decided to learn how to make it myself in Rust!
                        I used the "
                        <Link href="https://ratatui.rs/" name="ratatui" />
                        " crate and learned a lot about TUIs (terminal user interfaces) in the process."
                    </TextBlock>
                </ProjectRow>
            </Project>
        </div>
    }
}

#[component]
fn project_row<'a, R: Renderable>(
    image: &'a str,
    image_left: bool,
    children: &R,
) -> impl Renderable {
    let dir = if image_left {
        "lg:flex-row"
    } else {
        "lg:flex-row-reverse"
    };
    rsx! {
        <div class={ "lg:flex lg:gap-4 " (dir) }>
            <div class="lg:w-3/5 mb-3 lg:mb-0 flex items-start justify-center">
                <img src=(image) class="object-contain rounded-lg max-h-96 w-full" />
            </div>
            <div class="lg:w-2/5 flex flex-col gap-2">
                (children)
            </div>
        </div>
    }
}

#[component]
fn project<'a, R: Renderable>(title: &'a str, bg_color: &'a str, children: &R) -> impl Renderable {
    rsx! {
        <div class={ (bg_color) " rounded-xl p-5 shadow-lg" }>
            <div class="rounded-lg bg-surface0 px-4 py-3 mb-4 text-center">
                <h1 class="text-3xl text-text font-bold tracking-tight">
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
        <div class="rounded-lg bg-surface1 overflow-hidden shadow-sm">
            <div class="px-3 py-2 border-b border-surface0">
                <h2 class="text-base font-semibold text-text">
                    (title)
                </h2>
            </div>
            <p class={ (bg_color) " px-3 py-2 text-sm text-crust leading-relaxed" }>
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
            class="underline underline-offset-2 hover:underline-offset-4 font-medium transition-all"
        >
            (name)
        </a>
    }
}
