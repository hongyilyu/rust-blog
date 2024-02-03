use leptos::*;
use leptos_router::create_query_signal;

use crate::frontend::components::{
    buttons::{button_wrapper::ButtonWrapper, ButtonSize, ButtonVarient},
    tag::{TagRemove, Tags},
};

#[component]
pub fn FilteredTagButton() -> impl IntoView {
    let (tags, _) = create_query_signal::<Tags>("tags");
    view! {
        {move || {
            if !tags.get().unwrap_or_default().0.is_empty() {
                view! {
                    <ButtonWrapper
                        size=ButtonSize::SM
                        varient=ButtonVarient::OUTLINE
                        left_icon=icondata::FiX
                        extra_css="rounded-full"
                    >
                        Clear Filter
                        {move || {
                            tags.get()
                                .unwrap_or_default()
                                .split(',')
                                .map(|tag| view! { <TagRemove tag/> })
                                .collect_view()
                        }}

                    </ButtonWrapper>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }
        }}
    }
}
