use leptos::{html::Div, logging, prelude::*};
use leptos_use::{use_element_visibility, use_mouse_in_element, UseMouseInElementReturn};

use crate::contents::data::*;

#[component]
pub fn ContentSection(
    #[prop(default = "Heading")] heading: &'static str,
    #[prop(default = "")] id: &'static str,
    #[prop(default = RwSignal::new(vec!{}))] heading_list: RwSignal<Vec<String>>,
    #[prop(default = RwSignal::new("".to_string()))] active: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let UseMouseInElementReturn { is_outside, .. } = use_mouse_in_element(el);
    let is_visible = use_element_visibility(el);

    heading_list.write().push(heading.to_string());

    Effect::new(move |_| {
        // logging::log!("component= {}, is_visible={}", heading, is_visible.get());
        let heading_list = heading_list.get();
        if is_visible.get() {
            //appear
            if active.get() == "".to_string() {
                //none is active, first
                active.set(heading.to_string());
            } else {
                let new = heading_list.iter().position(|x| *x == heading.to_string());
                let curr = heading_list.iter().position(|x| *x == active.get());
                if new.is_some() && curr.is_some() && new.unwrap() < curr.unwrap() {
                    active.set(heading.to_string());
                }
            }
        } else {
            //disappear (1x occur on startup)
            if active.get() == heading.to_string() {
                logging::log!(
                    "disappear - active = heading {}, headingList={:?}",
                    active.get(),
                    heading_list
                );
                let curr = heading_list.iter().position(|x| *x == active.get());
                if curr.is_some() {
                    let next = curr.unwrap() + 1;
                    if next < heading_list.len() {
                        active.set(heading_list[next].clone());
                    } else {
                        logging::log!(
                            "WARNING, GOT INVALID INDEX WHEN {} DISAPPEAR, len {}, next {}",
                            heading,
                            heading_list.len(),
                            next
                        );
                    }
                } else {
                    logging::log!("WARNING, ACTIVE EXISTS BUT NOT IN HEADING LIST");
                }
            }
        }
    });
    view! {
        <div
            node_ref=el
            class="flex flex-col space-y-0.5 transition-colors duration-500 bg-radial-[at_0%_50%]"
            class=(
                [
                    "from-background-1",
                    "to-background",
                    "dark:from-dm-background-1",
                    "dark:to-dm-background",
                ],
                move || !is_outside.get(),
            )
            class=(["bg-background", "dark:bg-dm-background"], move || is_outside.get())
        >

            <div class="sticky top-0 z-50 bg-background dark:bg-dm-background transition-colors duration-500 pb-2">
                <h2
                    class="mt-2 sm:mt-4 text-xl xl:text-2xl transition-colors duration-500"
                    class=(
                        ["text-n-accent", "dark:text-dm-accent"],
                        move || active.get() == heading.to_string(),
                    )
                    class=(
                        ["text-main", "dark:text-dm-main"],
                        move || active.get() != heading.to_string(),
                    )
                    id=id
                >
                    {heading}
                </h2>
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
                    <h3 class="text-base xl:text-xl text-n-secondary dark:text-dm-secondary">
                        {title}
                    </h3>
                    <div class="w-full flex flex-row text-sm xl:text-base justify-between ">
                        <div class="font-bold">{subtitle}</div>
                        <div class="text-gray-500">{time}</div>
                    </div>
                </div>
                <div class="items-start text-main dark:text-dm-main-1">{children()}</div>
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
pub fn SkillSection(skill_data: ReadSignal<Vec<SkillData>>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-6 space-y-2 items-start">
            <For each=move || skill_data.get() key=|k| k.skill_type let(child)>
                <div class="col-span-2 px-2">
                    <p>{child.skill_type}</p>
                </div>
                <div class="col-span-4 space-x-0.5">
                    {child
                        .skill_subtype
                        .into_iter()
                        .map(|i| view! { <SmallButton text=i /> })
                        .collect_view()}
                </div>
            </For>
        </div>
    }
}

#[component]
pub fn ItemCard(
    #[prop(default = "Heading")] heading: &'static str,
    #[prop(default = "Description")] description: &'static str,
    #[prop(default = "images/stare-blank.png")] img: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card card-side shadow-sm border rounded-2xl border-n-secondary dark:border-dm-secondary z-0 text-sm xl:text-base">
            <figure class="flex-[35%] md:flex-[25%] lg:flex-[22.5%] 2xl:flex-[17.5%]">
                <img class="w-100% aspect-square object-cover" src=img alt="project image" />
            </figure>
            <div class="card-body p-0 sm:py-2 m-2 sm:ml-4 text-sm xl:text-base flex-[65%] md:flex-[75%] lg:flex-[77.5%] xl:flex-[85%]">
                <h3 class="text-base xl:text-xl">{heading}</h3>
                <p class="text-main dark:text-dm-main-1">{description}</p>
                <div class="justify-start">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn SmallButton(#[prop(default = "text")] text: &'static str) -> impl IntoView {
    view! {
        <button class="btn h-fit bg-n-primary dark:bg-dm-primary text-dm-main dark:text-main text-xs xl:text-sm p-1">
            {text}
        </button>
    }
}

#[component]
pub fn Footer(children: Children) -> impl IntoView {
    view! {
        <footer class="footer sm:footer-horizontal footer-center p-4 text-gray-500 text-xs xl:text-sm">
            <aside>{children()}</aside>
        </footer>
    }
}

#[component]
pub fn ToggleDarkMode(target_signal: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="flex cursor-pointer gap-2 mr-auto items-center pt-0 sm:pt-2">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
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
                class="toggle w-10 h-6 checked:bg-dm-primary bg-n-primary checked:border-main border-dm-main"
                checked=move || target_signal.get()
                on:click=move |_| target_signal.update(|value| *value = !*value)
            />
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
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
