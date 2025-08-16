use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/public/favicon.ico" />
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let dark_mode_signal = RwSignal::new(true);

    view! {
        <Title text="Leptos + Tailwindcss" />

        // Inject metadata in the <head>
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        // Body
        <main class="flex" class:dark=dark_mode_signal>
            <div class="flex flex-col sm:flex-row min-h-dvh min-w-dvh w-screen
            text-main dark:text-dm-main bg-background dark:bg-dm-background
            ">
                <aside class="basis-1/3 flex flex-col max-h-dvh top-0 text-right sm:sticky">
                    <div class="flex flex-col my-auto w-full p-2 pl-4">
                        <h2 class="">Ryukin Aranta Lika</h2>
                        <p class="text-shadow-white">Software Engineer</p>
                        <nav class="hidden sm:flex flex-col w-full pt-8 sm:pt-24 space-y-8 text-sm">
                            <div class="space-y-2">
                                <div class="flex flex-col space-y-1">
                                    <a rel="noopener noreferrer" href="#">
                                        About
                                    </a>
                                    <a rel="noopener noreferrer" href="#">
                                        Career
                                    </a>
                                    <a rel="noopener noreferrer" href="#">
                                        Skills
                                    </a>
                                    <a rel="noopener noreferrer" href="#">
                                        Education
                                    </a>
                                    <a rel="noopener noreferrer" href="#">
                                        Projects
                                    </a>
                                </div>
                            </div>
                        </nav>
                        <div class="pt-4 sm:pt-24 w-full text-sm text-left text-gray-400">
                            <nav class="flex flex-row sm:flex-col space-x-0.5">
                                <a
                                    class="flex flex-row items-center"
                                    rel="noopener noreferrer"
                                    href="#"
                                >
                                    <img src="/public/github.svg" />
                                    <div class="invisible sm:visible">Github</div>
                                </a>
                                <a
                                    class="flex flex-row items-center "
                                    rel="noopener noreferrer"
                                    href="#"
                                >
                                    <img src="/public/linkedin.svg" />
                                    <div class="invisible sm:visible">LinkedIn</div>
                                </a>
                            </nav>
                        </div>
                    </div>

                </aside>
                <div class="basis-2/3 flex flex-col items-center justify-center mx-auto p-4">
                    <h1 class="text-4xl text-center my-8">"Dark mode test"</h1>
                    <button
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                        on:click=move |_| dark_mode_signal.update(|value| *value = !*value)
                    >
                        "toggle"
                    </button>
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />
                    <p>
                        "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                    </p>
                    <br />

                </div>
            </div>

        </main>
    }
}
