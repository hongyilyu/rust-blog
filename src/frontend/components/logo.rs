use leptos::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <img
            src="/logo-black-bg.svg"
            alt="Hongyi Lyu"
            viewBox="0 0 148 144"
            width="32px"
            height="32px"
        />
    }
}
