use leptos::*;
use leptos_meta::*;

/// https://github.com/highlightjs/cdn-release/tree/main/build
#[component]
pub fn HighlightScript() -> impl IntoView {
    view! {
        <Link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@latest/build/styles/base16/material-palenight.min.css"
        />
        <Script
            type_="text/javascript"
            src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@latest/build/highlight.min.js"
        />
        <Script
            type_="text/javascript"
            src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@latest/build/languages/dockerfile.min.js"
        />
        <Script>
            document.addEventListener("DOMContentLoaded", (event) => {
                hljs.highlightAll();
            });
        </Script>
    }
}
