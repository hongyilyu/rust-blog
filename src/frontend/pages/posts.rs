use leptos::*;
use leptos_icons::*;
use leptos_router::create_query_signal;
use time::OffsetDateTime;

use crate::frontend::components::buttons::filtered_tag_button::FilteredTagButton;
use crate::frontend::components::post_preview_by_year::PostPreviewByYear;
use crate::frontend::components::tag::Tags;
use crate::{backend::server_functions::post::list_posts_metadata, common::post::PostType};

#[component]
pub fn Posts() -> impl IntoView {
    let featured_post = create_resource(
        || (),
        |_| async { list_posts_metadata(PostType::Blog).await },
    );
    let (tags, _) = create_query_signal::<Tags>("tags");

    view! {
        <header class="flex items-baseline">
            <h1>Articles</h1>
            <nav class="not-prose ml-auto space-x-2 text-sm text-gray-500">
                <Icon class="-mt-1 inline-block stroke-amber-500" icon=icondata::FiRss/>
            </nav>
        </header>

        <nav class="flex items-center justify-between">
            <FilteredTagButton/>
        </nav>

        <section class="mt-12 space-y-12">

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                // TODO: The logic is ugly as fk.
                // TODO: In theroy serde should preserve order with json.
                {move || {
                    featured_post
                        .and_then(|map| {
                            (2023..=OffsetDateTime::now_utc().year())
                                .rev()
                                .filter_map(|year| {
                                    map.get(&year)
                                        .map(|posts| {
                                            let mut sorted_posts = if !tags
                                                .get()
                                                .unwrap_or_default()
                                                .is_empty()
                                            {
                                                posts
                                                    .iter()
                                                    .filter(|post| {
                                                        tags.get()
                                                            .unwrap_or_default()
                                                            .iter()
                                                            .all(|tag| post.post_metadata.tags.contains(tag))
                                                    })
                                                    .cloned()
                                                    .collect()
                                            } else {
                                                posts.to_vec()
                                            };
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
