pub mod backend;
pub mod common;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod frontend;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::frontend::{
    components::{footer::Footer, header::Header},
    pages::{about::About, post::Post, posts::Posts, year_posts::YearPosts},
    scripts::{highlight_js::HighlightScript, theme_control::ThemeScript},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Layout>
                    <Route path="" view=About/>
                    <Route path="/posts" view=Posts/>
                    <Route path="/posts/:year" view=YearPosts/>
                    <Route path="/open-source" view=move || view!{} />
                    <Route
                        path="/posts/:year/:post"
                        view=move || {
                            view! {
                                <Post/>
                                <HighlightScript/>
                            }
                        }
                    />

                </Route>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/rust-blog.css"/>
        <Link rel="shortcut icon" type_="image/svg" href="/logo-black-bg.svg"/>
        <Meta name="description" content="Hongyi's Personal Website."/>
        <Title text="Hongyi Lyu"/>
        <ThemeScript/>

        <div class="flex flex-col min-h-screen">
            <Header/>
            <main
                class="grow prose dark:prose-invert md:prose-lg prose-h1:font-bold prose-img:rounded prose-li:my-1 mx-auto px-2 max-w-3xl w-full"
                id="content"
                role="main"
            >
                <Outlet/>
            </main>
            <Footer/>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
