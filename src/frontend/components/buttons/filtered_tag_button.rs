use leptos::*;
use leptos_icons::Icon;
use leptos_router::create_query_signal;

use crate::frontend::components::{
    buttons::{button_icon::ButtonIcon, ButtonSize, ButtonStyle},
    tag::{TagRemove, Tags},
};

#[component]
pub fn FilteredTagButton() -> impl IntoView {
    let (tags, _) = create_query_signal::<Tags>("tags");
    view! {
        {move || {
            if !tags.get().unwrap_or_default().is_empty() {
                view! {
                    <ClearFilter>
                        {move || {
                            <Vec<String> as Clone>::clone(&tags.get().unwrap_or_default())
                                .into_iter()
                                .map(|tag| view! { <TagRemove tag/> })
                                .collect_view()
                        }}

                    </ClearFilter>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }
        }}
    }
}

#[component]
fn ClearFilter(children: Children) -> impl IntoView {
    let class_name = format!(
        "rounded-full
        inline-flex appearance-none
        items-center justify-center rounded
        font-medium
        leading-5
        transition-colors ease-out
        disabled:cursor-not-allowed disabled:opacity-40 disabled:shadow-none
        {}
        {}
    ",
        ButtonSize::SM,
        ButtonStyle::OutlineGray,
    );

    let (_, set_tags) = create_query_signal::<Tags>("tags");

    let on_click = move |_| set_tags(None);

    view! {
        <span class=class_name>
            <button
                on:click=on_click
                class=format!("rounded-full mr-2 inline-flex {}", ButtonStyle::GhostGray)
            >
                <ButtonIcon extra_css="mr-2">
                    <Icon icon=icondata::FiX/>
                </ButtonIcon>
                Clear Filter
            </button>
            {children()}
        </span>
    }
}
