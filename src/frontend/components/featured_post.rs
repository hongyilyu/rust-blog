use leptos::*;
use leptos_icons::Icon;

use crate::{common::post::Post, frontend::components::post_preview::PostPreview};

#[component]
pub fn FeaturedPost(post: Post, heading: String) -> impl IntoView {
    view! {
        <section class="relative rounded border px-4 py-3 shadow-md
        bg-bgLight dark:border-gray-800 dark:bg-gray-900 dark:shadow-inner my-6">
            <PostPreview heading post/>
            <Icon
                icon=icondata::FiBookmark
                class="absolute -top-1 right-2 !my-0
                fill-white stroke-gray-500 dark:fill-gray-900/50"
            />
        </section>
    }
}
