use leptos::*;
use web_sys::MouseEvent;

use crate::frontend::components::buttons::{button_wrapper::ButtonWrapper, ButtonSize, ButtonVarient};

/// Button showing only the icon
#[component]
pub fn IconButton(
    children: Children,
    #[prop(optional)] on_click: Option<fn(MouseEvent)>,
) -> impl IntoView {
    view! {
        //TODO: px-4 in Size::MD causing extra padding for the icon, but nothing we can do without tailwind-merge
            <ButtonWrapper
                size=ButtonSize::MD
                varient=ButtonVarient::GHOST
                extra_css="p-0 rounded-full"
                on:click=move |e| {
                    if let Some(ref click_handler) = on_click {
                        click_handler(e)
                    }
                }
            >

                {children()}
            </ButtonWrapper>
    }
}
