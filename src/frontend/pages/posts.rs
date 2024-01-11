use leptos::*;
use leptos_icons::*;

use crate::{backend::server_functions::post::list_posts_metadata, common::post::PostType, frontend::components::post_preview::PostPreview};

#[component]
pub fn Posts() -> impl IntoView {
    let featured_post =
        create_resource(|| (), |_| async { list_posts_metadata(PostType::Blog).await });
    view! {
        <header class="flex items-baseline">
            <h1>Articles</h1>
            <nav class="not-prose ml-auto space-x-2 text-sm text-gray-500">
                <Icon class="-mt-1 inline-block stroke-amber-500" icon=Icon::Fi(FiIcon::FiRss)/>
            </nav>
        </header>

        <section class="mt-12 space-y-12">

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    featured_post
                        .and_then(|posts| {
                            posts
                                .iter()
                                .map(|post| {
                                    view! { <PostPreview post=post.clone()/> }
                                })
                                .collect_view()
                        })
                }}

            </Transition>
        </section>
    }
}
