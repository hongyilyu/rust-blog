use leptos::*;
use leptos_meta::*;

/// https://github.com/highlightjs/cdn-release/tree/main/build
#[component]
pub fn HighlightScript() -> impl IntoView {
    view! {
        <script
            type_="text/javascript"
            src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@latest/build/highlight.min.js"
        ></script>
        <link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@latest/build/styles/base16/material-palenight.min.css"
        />
        <script
            type_="text/javascript"
            src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@latest/build/languages/dockerfile.min.js"
        ></script>
        <Script>
            setTimeout(()=>{
                hljs.highlightAll();
            }, 500)
        </Script>
    }
}
