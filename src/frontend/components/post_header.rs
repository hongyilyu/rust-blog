use leptos::*;

use crate::common::post::Post;

#[component]
pub fn PostHeader(post: Post) -> impl IntoView {
    view! {
        <figure role="header" class="not-prose">
            <h1 class="mb-4 mt-8 text-5xl font-bold leading-[1.15] text-gray-950 dark:text-gray-50">
                {post.post_metadata.title}
            </h1>
            <figcaption class="flex flex-wrap gap-2 text-sm text-gray-500">
                "Hongyi Lyu  • " {post.post_metadata.publication_date.to_string()} " • "
                {post.post_attribute.reading_time}
            // {tags && Boolean(tags.length) && (
            // <TagsNav tags={tags} class="ml-auto" />
            // )}
            </figcaption>
        </figure>
    }
}
