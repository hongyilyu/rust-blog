use leptos::*;

use crate::{common::post::Post, frontend::components::tag::TagAppend};

#[component]
pub fn PostPreview(#[prop(optional)] heading: String, post: Post) -> impl IntoView {
    let heading_class = "mb-1 text-2xl font-bold";
    let metadata = post.post_metadata;
    let uri = post.post_attribute.uri;

    let head = match heading.as_ref() {
        "h3" => view! {
            <h3 class=heading_class style="color:var(--tw-prose-headings)">
                <a href=uri>{move || metadata.title.clone()}</a>
            </h3>
        }
        .into_view(),
        _ => view! {
            <h2 class=heading_class style="color:var(--tw-prose-headings)">
                <a href=uri>{move || metadata.title.clone()}</a>
            </h2>
        }
        .into_view(),
    };

    view! {
        <article class="not-prose">
            <hgroup>
                {head} <figcaption class="flex flex-wrap gap-y-1 text-sm text-gray-500">
                    <span class="mr-2 inline-block">
                        {metadata.publication_date.to_string()} " • "
                        {post.post_attribute.reading_time} " • "
                    </span>
                    <div class="flex gap-2">
                        {metadata.tags.iter().map(|tag| view! { <TagAppend tag/> }).collect_view()}
                    </div>
                </figcaption> <p class="mt-3 leading-relaxed">{metadata.description}</p>
            </hgroup>
        </article>
    }
}
