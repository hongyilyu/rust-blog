use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use crate::backend::server_functions::post::list_year_posts_metadata;
use crate::frontend::components::buttons::filtered_tag_button::FilteredTagButton;
use crate::frontend::components::post_preview_by_year::PostPreviewByYear;
use crate::common::post::PostType;
use crate::frontend::components::tag::Tags;

#[derive(Params, PartialEq)]
pub struct YearParams {
    pub year: i32,
}

#[component]
pub fn YearPosts() -> impl IntoView {
    let params = use_params::<YearParams>();
    let year = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.year)
                .unwrap_or_default()
        })
    };

    let featured_post = create_resource(
        year, 
        |val| async move { list_year_posts_metadata(PostType::Blog, val).await },
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
                {move || {
                    featured_post
                        .and_then(|posts| {
                            let mut posts = if tags.get().unwrap_or_default().is_empty() {
                                posts.clone()
                            } else {
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
                            };
                            posts.sort_by_key(|post| post.post_metadata.publication_date);
                            view! { <PostPreviewByYear year=year() posts/> }
                        })
                }}

            </Transition>
        </section>
    }
}
