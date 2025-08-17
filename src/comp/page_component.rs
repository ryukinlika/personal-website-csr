use leptos::prelude::*;

#[component]
pub fn ContentSection(
    #[prop(default = "Heading")] heading: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-0.5">
            <div class="sticky top-0 bg-background dark:bg-dm-background">
                <h3 class="mt-2 sm:mt-8">{heading}</h3>
                <div class="divider sm:hidden m-0"></div>
            </div>
            <div class="flex align-center w-full p-2">{children()}</div>
        </div>
    }
}

#[component]
pub fn ContentHighlighted(
    #[prop(default = "Title")] title: &'static str,
    #[prop(default = "SubTitle")] subtitle: &'static str,
    #[prop(default = "")] time: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-row w-full">
            <div class="divider divider-horizontal m-0 p-0 pr-2 before:bg-secondary after:bg-secondary dark:before:bg-dm-secondary dark:after:bg-dm-secondary"></div>
            <div class="flex flex-col w-full">
                <div class="items-center">
                    <h4 class="text-secondary dark:text-dm-secondary">{title}</h4>
                    <div class="w-full flex flex-row text-sm justify-between">
                        <div>{subtitle}</div>
                        <div class="opacity-70">{time}</div>
                    </div>
                </div>
                <div class="items-start">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn ItemLists(items: Vec<&'static str>) -> impl IntoView {
    view! { <ul>{items.into_iter().map(|i| view! { <li>{i}</li> }).collect_view()}</ul> }
}
