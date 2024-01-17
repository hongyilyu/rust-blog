pub mod backend;
pub mod common;
pub mod error_template;
pub mod fileserv;
pub mod frontend;

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{frontend::{
    components::{footer::Footer, header::Header},
    pages::{about::About, post::Post, posts::Posts}, scripts::{theme_control::ThemeScript, highlight_js::HighlightScript},
}, backend::server_functions::post::list_posts_metadata, common::post::PostType};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let featured_post =
        create_resource(|| (), |_| async { list_posts_metadata(PostType::Blog).await });
    provide_context(featured_post);

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Layout>
                    <Route path="" view=About/>
                    <Route path="/posts" view=Posts/>
                    <Route
                        path="/posts/:post"
                        view=move || {
                            view! {
                                <HighlightScript/>
                                <Post/>
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
                class="grow prose dark:prose-invert md:prose-lg prose-h1:font-bold prose-img:rounded prose-li:my-1 mx-auto px-2"
                id="content"
                role="main"
            >
                <Outlet/>
            </main>
            <Footer/>
        </div>
    }
}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}
