pub mod error_template;
pub mod fileserv;
pub mod frontend;

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::frontend::components::{footer::Footer, header::Header, theme_control::ThemeScript};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (is_routing, set_is_routing) = create_signal(false);

    view! {
        <Router set_is_routing>
            <Routes>
                <Route path="/" view=move || view! { <Layout is_routing/> }>
                    <Route path="" view=frontend::components::logo::Logo/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Layout(is_routing: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/rust-blog.css"/>
        <Link rel="shortcut icon" type_="image/svg" href="/logo-black-bg.svg"/>
        <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
        <Meta name="description" content="Hongyi's Personal Website."/>
        <Title text="Hongyi Lyu"/>
        <ThemeScript/>

        <div class="flex flex-col min-h-screen">
            <Header/>
            <main class="grow">
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
