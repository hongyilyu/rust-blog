use leptos::*;

/// Icon within button.
#[component]
pub fn ButtonIcon(#[prop(optional, into)] extra_css: String, children: Children) -> impl IntoView {
    let className = format!("inline-flex shrink-0 self-center {extra_css}");
    view! { <span class=className>{children()}</span> }
}
