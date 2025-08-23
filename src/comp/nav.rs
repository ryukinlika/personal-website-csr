use leptos::prelude::*;

#[component]
pub fn SocialLink() -> impl IntoView {
    view! {
        <div class="pt-4 sm:pt-20 w-full text-sm text-left text-gray-500 items-center">
            <nav class="flex flex-row sm:flex-col space-x-1 sm:space-y-1">
                <a class="flex flex-row items-center" rel="noopener noreferrer" href="#">
                    <svg
                        class="fill-gray-500 w-4 h-4"
                        fill="#000000"
                        height="24"
                        width="24"
                        id="Layer_1"
                        xmlns="http://www.w3.org/2000/svg"
                        xmlns:xlink="http://www.w3.org/1999/xlink"
                        viewBox="0 0 504.4 504.4"
                        xml:space="preserve"
                    >
                        <g>
                            <g>
                                <path d="M377.6,0.2H126.4C56.8,0.2,0,57,0,126.6v251.6c0,69.2,56.8,126,126.4,126H378c69.6,0,126.4-56.8,126.4-126.4V126.6
                                C504,57,447.2,0.2,377.6,0.2z M168,408.2H96v-208h72V408.2z M131.6,168.2c-20.4,0-36.8-16.4-36.8-36.8c0-20.4,16.4-36.8,36.8-36.8
                                c20.4,0,36.8,16.4,36.8,36.8C168,151.8,151.6,168.2,131.6,168.2z M408.4,408.2H408h-60V307.4c0-24.4-3.2-55.6-36.4-55.6
                                c-34,0-39.6,26.4-39.6,54v102.4h-60v-208h56v28h1.6c8.8-16,29.2-28.4,61.2-28.4c66,0,77.6,38,77.6,94.4V408.2z" />
                            </g>
                        </g>
                    </svg>
                    <div class="hidden sm:flex px-2 ">LinkedIn</div>
                </a>
                <a class="flex flex-row items-center" rel="noopener noreferrer" href="#">
                    <svg
                        class="fill-gray-500 w-4 h-4"
                        fill="#000000"
                        id="Layer_1"
                        width="24"
                        height="24"
                        xmlns="http://www.w3.org/2000/svg"
                        xmlns:xlink="http://www.w3.org/1999/xlink"
                        viewBox="0 0 97.65 96.03"
                        xml:space="preserve"
                    >
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"
                        />

                    </svg>
                    <div class="hidden sm:flex px-2 ">Github</div>
                </a>

            </nav>
        </div>
    }
}

#[component]
pub fn SectionNav(
    #[prop(default = "title")] title: &'static str,
    #[prop(default = "#")] href: &'static str,
    #[prop(default = RwSignal::new("".to_string()))] active: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div
            class="transition-all duration-100"
            class=(
                ["text-base", "font-bold", "underline", "text-n-accent", "dark:text-dm-accent"],
                move || active.get() == title.to_string(),
            )
            class=(
                ["text-sm", "font-normal", "text-main", "dark:text-dm-main"],
                move || active.get() != title.to_string(),
            )
        >
            <a rel="noopener noreferrer" href=href>
                {title}
            </a>
        </div>
    }
}
