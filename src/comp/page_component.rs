use leptos::prelude::*;

#[component]
pub fn ContentSection(
    #[prop(default = "Heading")] heading: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-0.5">
            <div class="sticky top-0 z-50 bg-background dark:bg-dm-background">
                <h3 class="mt-2 sm:mt-4">{heading}</h3>
                <div class="divider sm:hidden m-0  before:bg-n-primary after:bg-n-primary dark:before:bg-dm-primary dark:after:bg-dm-primary opacity-10 "></div>
            </div>
            <div class="flex flex-col align-center w-full py-2 space-y-6">{children()}</div>
        </div>
    }
}

#[component]
pub fn ContentHighlight(
    #[prop(default = "Title")] title: &'static str,
    #[prop(default = "SubTitle")] subtitle: &'static str,
    #[prop(default = "")] time: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-row w-full">
            <div class="divider divider-horizontal m-0 p-0 pr-2 before:bg-n-secondary after:bg-n-secondary dark:before:bg-dm-secondary dark:after:bg-dm-secondary"></div>
            <div class="flex flex-col w-full px-2">
                <div class="items-center space-y-1">
                    <h4 class="text-n-secondary dark:text-dm-secondary">{title}</h4>
                    <div class="w-full flex flex-row text-sm justify-between ">
                        <div>{subtitle}</div>
                        <div class="text-gray-500">{time}</div>
                    </div>
                </div>
                <div class="items-start">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn ItemLists(items: Vec<&'static str>) -> impl IntoView {
    view! {
        <ul class="list-disc pt-2 pl-4">
            {items.into_iter().map(|i| view! { <li>{i}</li> }).collect_view()}
        </ul>
    }
}

#[component]
pub fn ItemCard() -> impl IntoView {
    view! {
        <div class="card card-side shadow-sm border rounded-2xl border-n-secondary dark:border-dm-secondary max-h-40 z-0">
            <figure class="w-[40%]">
                <img src="images/stare-blank.png" alt="project" />
            </figure>
            <div class="card-body">
                <h4>"New movie is released!"</h4>
                <p>Click the button to watch on Jetflix app.</p>
                <div class="card-actions justify-start">
                    <button class="btn btn-n-primary">Watch</button>
                </div>
            </div>
        </div>
        <div class="card card-side shadow-sm border rounded-2xl border-n-secondary dark:border-dm-secondary max-h-40">
            <figure class="w-[40%]">
                <img
                    src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
                    alt="project"
                />
            </figure>
            <div class="card-body">
                <h4>"New movie is released!"</h4>
                <p>Click the button to watch on Jetflix app.</p>
                <div class="card-actions justify-start">
                    <button class="btn btn-n-primary">Watch</button>
                </div>
            </div>
        </div>
        <div class="card card-side shadow-sm border rounded-2xl border-n-secondary dark:border-dm-secondary max-h-40 z-auto">
            <figure class="w-[40%]">
                <img
                    src="https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp"
                    alt="project"
                />
            </figure>
            <div class="card-body">
                <h4>"New movie is released!"</h4>
                <p>Click the button to watch on Jetflix app.</p>
                <div class="card-actions justify-start">
                    <button class="btn btn-n-primary">Watch</button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer sm:footer-horizontal footer-center p-4 text-gray-500">
            <aside>
                <p>"Test Footer contents Lorem ipsum dolor sit amet, consectetur"</p>
            </aside>
        </footer>
    }
}

#[component]
pub fn ToggleDarkMode(target_signal: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="flex cursor-pointer gap-2">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="5" />
                <path d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4" />
            </svg>
            <input
                type="checkbox"
                class="toggle checked:bg-dm-primary bg-n-primary checked:border-main border-dm-main"
                checked=move || target_signal.get()
                on:click=move |_| target_signal.update(|value| *value = !*value)
            />
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
            </svg>
        </label>
    }
}
