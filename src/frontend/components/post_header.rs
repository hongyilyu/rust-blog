use leptos::*;

use crate::common::post::PostMetadata;

#[component]
pub fn PostHeader(post: PostMetadata) -> impl IntoView {
    view! {
        <figure role="header" class="not-prose">
            <h1 class="mb-4 mt-8 text-5xl font-bold leading-[1.15] text-gray-950 dark:text-gray-50">
                {post.title}
            </h1>
            <figcaption class="flex flex-wrap gap-2 text-sm text-gray-500">
                "Hongyi Lyu  • " {post.publication_date.to_string()} " • "
                {post.reading_time}
            // {tags && Boolean(tags.length) && (
            // <TagsNav tags={tags} class="ml-auto" />
            // )}
            </figcaption>
        </figure>
    }
}
