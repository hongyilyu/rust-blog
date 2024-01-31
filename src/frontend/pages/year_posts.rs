use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use crate::frontend::components::buttons::filtered_tag_button::FilteredTagButton;
use crate::frontend::components::post_preview_by_year::PostPreviewByYear;
use crate::{backend::server_functions::post::list_posts_metadata, common::post::PostType};

#[derive(Params, PartialEq)]
pub struct YearParams {
    pub year: i32,
}

#[component]
pub fn YearPosts() -> impl IntoView {
    let featured_post = create_resource(
        || (),
        |_| async { list_posts_metadata(PostType::Blog).await },
    );

    let params = use_params::<YearParams>();
    let year = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.year)
                .unwrap_or_default()
        })
    };

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
                            let mut year_posts = posts
                                .iter()
                                .filter(|post| {
                                    post.post_metadata.publication_date.year().eq(&year())
                                })
                                .cloned()
                                .collect::<Vec<_>>();
                            year_posts.sort_by_key(|post| post.post_metadata.publication_date);
                            view! { <PostPreviewByYear year=year() posts=year_posts/> }
                        })
                }}

            </Transition>
        </section>
    }
}
