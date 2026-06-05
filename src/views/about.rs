use hypertext::prelude::*;

#[component]
pub fn about() -> impl Renderable {
    rsx! {
        <div class="p-4 flex flex-col gap-6 items-center">

            // Header
            <div class="bg-blue rounded-xl shadow-lg text-center px-8 py-6 w-3/4 lg:w-1/2">
                <h1 class="text-4xl lg:text-6xl text-crust font-bold tracking-tight">
                    "About Me"
                </h1>
            </div>

            // Bio card
            <div class="bg-green rounded-xl shadow-lg p-6 w-full lg:w-3/4 xl:w-1/2">
                <div class="flex flex-col lg:flex-row gap-6 items-start">
                    <img
                        src="jake.png"
                        class="rounded-full w-32 h-32 object-cover shadow-md mx-auto lg:mx-0 lg:flex-none"
                    />
                    <div class="flex-1">
                        <h2 class="text-2xl font-bold text-crust border-b-2 border-crust pb-1 mb-3">
                            "Who am I?"
                        </h2>
                        <div class="space-y-3 text-crust leading-relaxed">
                            <p>
                                "I am a developer currently residing in New Mexico, USA. I work for
                                Mayo Clinic on the MCS team and provide cloud-based
                                infrastructure on Google Cloud Platform for an entire Enterprise-grade application."
                            </p>
                            <p>
                                "I am happily married to my wife Rebecca and we have 10 pets: 5 dogs
                                and 5 cats. It's a full house, but it's a house full of love."
                            </p>
                            <p>
                                "In my spare time I enjoy spending time with friends and family,
                                playing chess, playing video games, and working on coding projects."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Social links
            <div class="bg-pink rounded-xl shadow-lg px-8 py-4 flex gap-6 items-center">
                <a
                    href="https://github.com/RoastBeefer00"
                    class="transition-transform hover:scale-110"
                >
                    <img src="github-logo.png" class="w-16 h-16 object-contain" />
                </a>
                <a
                    href="https://www.linkedin.com/in/jacob-jasmin-873562125/"
                    class="transition-transform hover:scale-110"
                >
                    <img src="linkedin.png" class="w-16 h-16 object-contain" />
                </a>
            </div>

        </div>
    }
}
