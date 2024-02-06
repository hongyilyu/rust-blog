use leptos::*;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::frontend::components::theme_control::Theme;

#[component]
pub fn Logo() -> impl IntoView {
    let (theme, _, _) = use_local_storage::<Theme, FromToStringCodec>("theme");
    view! {
        <img
            src=move || match theme.get() {
                Theme::Dark => "/logo-black-bg.svg",
                Theme::Light => "/logo-white-bg.svg",
            }
            alt="Hongyi Lyu"
            viewBox="0 0 148 144"
            width="32px"
            height="32px"
        />
    }
}
