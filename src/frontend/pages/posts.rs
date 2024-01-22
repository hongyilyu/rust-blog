use std::collections::HashMap;

use leptos::*;
use leptos_icons::*;
use time::OffsetDateTime;

use crate::common::post::Post;
use crate::frontend::components::post_preview_by_year::PostPreviewByYear;
use crate::{backend::server_functions::post::list_posts_metadata, common::post::PostType};

#[component]
pub fn Posts() -> impl IntoView {
    let featured_post = create_resource(
        || (),
        |_| async { list_posts_metadata(PostType::Blog).await },
    );

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
                // TODO: The logic is ugly as fk.
                {move || {
                    featured_post
                        .and_then(|posts| {
                            let mut map = HashMap::<i32, Vec<Post>>::new();
                            posts
                                .iter()
                                .for_each(|post| {
                                    map.entry(post.post_metadata.publication_date.year())
                                        .or_default()
                                        .push(post.clone())
                                });
                            (2023..=OffsetDateTime::now_utc().year())
                                .rev()
                                .filter_map(|year| {
                                    map.get(&year)
                                        .map(|posts| {
                                            let mut sorted_posts = posts.to_vec();
                                            sorted_posts
                                                .sort_by_key(|post| post.post_metadata.publication_date);
                                            view! { <PostPreviewByYear year posts=sorted_posts/> }
                                        })
                                })
                                .collect_view()
                        })
                }}

            </Transition>
        </section>
    }
}
