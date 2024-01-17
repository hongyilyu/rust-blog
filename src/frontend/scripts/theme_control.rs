use leptos::*;
use leptos_meta::Script;

/// Script to set the theme based on local storage
/// This is blocking by design: to avoid a flash of light theme
#[component]
pub fn ThemeScript() -> impl IntoView {
    const JS: &str = r#"
        if (
            localStorage.getItem("theme") === '"Dark"' || localStorage.getItem("theme") === null 
        ) {
            document.documentElement.classList.add("dark");
        }
    "#;
    view! { <Script>{JS}</Script> }
}
