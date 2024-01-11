use leptos::*;

use crate::frontend::components::{logo::Logo, theme_control::ThemeControl};


#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("About", "/"), ("Blog", "/posts"), ("Open Source", "/open-source")];

    // TODO: fix `w-full`
    view! {
        <header class="mx-auto flex max-w-3xl w-full flex-wrap items-center gap-1 px-2 pb-8 md:pt-12">
            <nav class="flex items-center gap-6" aria-label="Navigation">
                <a href="/" class="rounded-full">
                    <Logo/>
                </a>
                {nav_items
                    .into_iter()
                    .map(|item| view! { <a href=item.1>{item.0}</a> })
                    .collect_view()}
            </nav>
            <nav class="ml-auto flex" aria-label="Settings">
                <ThemeControl/>
            </nav>
        </header>
    }
}
