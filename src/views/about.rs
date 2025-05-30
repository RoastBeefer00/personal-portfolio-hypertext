use hypertext::prelude::*;

#[component]
pub fn about() -> impl Renderable {
    rsx! {
        <div class="m-2 flex flex-col h-screen text-crust">
            <div
                class="bg-blue rounded justify-center text-center flex mx-auto m-6 p-6 lg:w-1/2"
            >
                <h1 class="text-4xl lg:text-6xl text-crust flex-none">
                    <b>About Me</b>
                </h1>
            </div>
            <div
                class="m-2 lg:w-3/4 xl:w-1/2 p-4 bg-green justify-center mx-auto rounded"
            >
                <h1 class="border-b-2 border-crust text-2xl font-bold">"Who am I?"</h1>
                <p class="mt-1">
                    "I am a developer currently residing in New Mexico, USA. I work for
                    Mayo Clinic on the MCS team and provide cloud-based
                    infrastructure on Google Cloud Platform for an entire Enterprise-grade application."
                </p>
                <br />
                <p>
                    "I am happily married to my wife Rebecca and we have 10 pets: 5 dogs
                    and 5 cats. It's a full house, but it's a house full of love.
                "</p>
                <br />
                <p>
                    "In my spare time I enjoy spending time with friends and family,
                    playing chess, playing video games, and working on coding projects."
                </p>
            </div>
            <div
                class="m-6 p-2 mb-20 rounded bg-pink w-fit h-fit flex justify-center mx-auto"
            >
                <a href="https://github.com/RoastBeefer00"
                    ><img src="github-logo.png" class="m-2 max-w-20 max-h-20" /></a
                >
                <a href="https://www.linkedin.com/in/jacob-jasmin-873562125/"
                    ><img src="linkedin.png" class="m-2 max-w-20 max-h-20" />
                </a>
            </div>
        </div>
    }
}
