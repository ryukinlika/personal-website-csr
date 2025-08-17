use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::comp::page_component::*;

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

    // About Data

    // Career Data
    let srin_desc= vec![
        "Assigned as Lead Engineer that maintain .NET Framework integration on Tizen OS, resolving real-world market problems related to .NET application on Tizen TV.",
        "Collaborated in porting .NET Framework to Tizen TV on RISC-V Architecture, showcased on 2024 Samsung Developer Conference.",
        "Analyzed logs, source files, and core dumps to diagnose and resolve issues affecting .NET application on Tizen TV, including real-world market problems.",
        "Developed a Linux Daemon in C++ that manages TV workload after software update, improving responsiveness and performance of the TV.",
        "Prototyped a solution for running deep learning framework on Android for internal developers.",
        "Tech stack: C++, .NET, Tizen, Python, Java, CMake, Linux."
    ];

    let xtremax_desc= vec![
        "Performed maintenance, critical vulnerability fixes, user support, and development on a Singapore Government Website.", 
        "Collaborated in deploying a custom feature for Sitecore valued in thousands of SGD.", 
        "Tech stack: C#, .NET, Sitecore CMS.", 
    ];

    let tokopedia_desc= vec![
        "Improved code reusability by refactor old code to “Clean Architecture” in voucher microservice.",
        "Improved features of voucher microservice, developing API with unit tests and documentations.",
        "Tech stack: Go/Golang, gRPC, GraphQL, Redis",    
    ];

    // Skills Data

    // Education Data

    // Project Data

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
                    <div class="flex flex-col my-auto w-full p-4 sm:p-2">
                        <h2 class="">Ryukin Aranta Lika</h2>
                        <p class="text-shadow-white">Software Engineer</p>
                        <nav class="hidden sm:flex flex-col w-full pt-8 sm:pt-16 space-y-8 text-sm">
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
                        <div class="pt-4 sm:pt-20 w-full text-sm text-left text-gray-400">
                            <nav class="flex flex-row sm:flex-col space-x-0.5">
                                <a
                                    class="flex flex-row items-center "
                                    rel="noopener noreferrer"
                                    href="#"
                                >
                                    <img src="/public/linkedin.svg" />
                                    <div class="invisible sm:visible">LinkedIn</div>
                                </a>
                                <a
                                    class="flex flex-row items-center"
                                    rel="noopener noreferrer"
                                    href="#"
                                >
                                    <img src="/public/github.svg" />
                                    <div class="invisible sm:visible">Github</div>
                                </a>

                            </nav>
                        </div>
                    </div>

                </aside>

                <div class="divider sm:divider-horizontal mx-0 opacity-40
                before:bg-primary after:bg-primary dark:before:bg-dm-primary dark:after:bg-dm-primary"></div>

                <div class="basis-2/3 flex flex-col mx-auto p-4 pl-8">
                    <ContentSection heading="About">
                        <p>
                            "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                            "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                            "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                            "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                            "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                            "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fugit facilis dolorum vel, enim aspernatur, adipisci, impedit beatae quibusdam provident dicta quidem necessitatibus quasi nam voluptates sed! Accusamus eligendi tempore placeat."
                        </p>
                    </ContentSection>
                    <ContentSection heading="Career">
                        <ContentHighlighted
                            title="Software Engineer - Backend"
                            subtitle="Samsung R&D Institute Indonesia"
                            time="September 2023 - Now"
                        >
                            <ItemLists items=srin_desc />
                        </ContentHighlighted>
                    </ContentSection>
                    <ContentSection>"aa"</ContentSection>
                    <ContentSection>"aa"</ContentSection>
                    <ContentSection>"aa"</ContentSection>
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
