use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::comp::nav::*;
use crate::comp::page_component::*;
use crate::contents::data::*;

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
    let (career_data, _) = signal(generate_career_data());
    // Skills Data

    // Education Data
    let (edu_data, _) = signal(generate_edu_data());

    // Project Data

    // Active heading
    let heading_list: RwSignal<Vec<String>> = RwSignal::new(vec![]);
    let active_heading = RwSignal::new("".to_string());

    view! {
        <Title text="Leptos + Tailwindcss" />

        // Inject metadata in the <head>
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        // Body
        <main
            class="flex bg-background dark:bg-dm-background transition-all duration-1000"
            class:dark=dark_mode_signal
        >
            <div class="flex flex-col sm:flex-row min-h-dvh mx-auto text-sm
            text-main dark:text-dm-main bg-background dark:bg-dm-background transition-all duration-1000
            ">
                <aside class="md: basis-auto lg:basis-2/5 flex flex-col max-h-dvh top-0 text-right sm:sticky">
                    <div class="flex flex-col my-auto w-full px-4 py-2 sm:pr-2 sm:pl-6">
                        <h2 class="">Ryukin Aranta Lika</h2>
                        <p class="text-n-primary dark:text-dm-primary">Software Engineer</p>
                        <ToggleDarkMode target_signal=dark_mode_signal />
                        // <button
                        // class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                        // on:click=move |_| dark_mode_signal.update(|value| *value = !*value)
                        // >
                        // "toggle"
                        // </button>
                        <nav class="hidden sm:flex flex-col w-full pt-8 sm:pt-16 space-y-8 ">
                            <div class="space-y-2">
                                <div class="flex flex-col space-y-1">
                                    <SectionNav title="About" href="#about" active=active_heading />
                                    <SectionNav
                                        title="Career"
                                        href="#career"
                                        active=active_heading
                                    />
                                    <SectionNav
                                        title="Skills"
                                        href="#skills"
                                        active=active_heading
                                    />
                                    <SectionNav
                                        title="Education"
                                        href="#education"
                                        active=active_heading
                                    />
                                    <SectionNav
                                        title="Projects"
                                        href="#projects"
                                        active=active_heading
                                    />
                                </div>
                            </div>
                        </nav>
                        <SocialLink />
                    </div>

                </aside>

                <div class="divider sm:divider-horizontal my-1 sm:my-0 mx-0 opacity-40 max-h-screen px-2 sm:px-0 sm:py-[5%] sm:sticky sm:top-0
                before:bg-n-primary after:bg-n-primary dark:before:bg-dm-primary dark:after:bg-dm-primary"></div>

                <div class="basis-2/3 flex flex-col mx-auto p-4 sm:pl-8 sm:pt-8">
                    <ContentSection
                        heading="About"
                        id="about"
                        heading_list=heading_list
                        active=active_heading
                    >
                        <div>
                            <p>
                                "An engineer with an interest in wide range of software topics, eager to learn new concepts and technologies, and
                                worked on diverse projects. A fast learner and quick to adapt to new challenges."
                            </p>
                            <br />
                        </div>
                    </ContentSection>
                    <ContentSection
                        heading="Career"
                        id="career"
                        heading_list=heading_list
                        active=active_heading
                    >
                        <For each=move || career_data.get() key=|k| k.title let(child)>
                            <ContentHighlight
                                title=child.title
                                subtitle=child.subtitle
                                time=child.time
                            >
                                <ItemLists items=child.description />
                            </ContentHighlight>

                        </For>
                    </ContentSection>
                    <ContentSection
                        heading="Skills"
                        id="skills"
                        heading_list=heading_list
                        active=active_heading
                    >
                        "aa"
                    </ContentSection>
                    <ContentSection
                        heading="Education"
                        id="education"
                        heading_list=heading_list
                        active=active_heading
                    >
                        <For each=move || edu_data.get() key=|k| k.title let(child)>
                            <ContentHighlight
                                title=child.title
                                subtitle=child.subtitle
                                time=child.time
                            >
                                <ItemLists items=child.description />
                            </ContentHighlight>

                        </For>
                    </ContentSection>
                    <ContentSection
                        heading="Projects"
                        id="projects"
                        heading_list=heading_list
                        active=active_heading
                    >
                        <ItemCard>
                            <a href="aa">
                                <SmallButton text="Github Link" />
                            </a>
                        </ItemCard>
                        <ItemCard>
                            <a href="aa">
                                <SmallButton text="Github Link" />
                            </a>
                        </ItemCard>
                    </ContentSection>

                    <Footer text="This personal website was created with Rust (Leptos) and TailwindCSS + daisyui to challenge myself in learning new framework and designing a website" />
                </div>
            </div>

        </main>
    }
}
