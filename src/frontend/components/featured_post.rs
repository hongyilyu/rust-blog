use leptos::*;
use leptos_icons::*;

use crate::frontend::components::post_preview::PostPreview;

#[component]
pub fn FeaturedPost() -> impl IntoView {
    let heading = "h3".to_owned();
    view! {
        <section class="relative rounded border px-4 py-3 shadow-md
        bg-bgLight dark:border-gray-800 dark:bg-gray-900 dark:shadow-inner my-4">
            <PostPreview heading=heading/>
            <Icon
                icon=Icon::Fi(FiIcon::FiBookmark)
                class="absolute -top-1 right-2 !my-0
                fill-white stroke-gray-500 dark:fill-gray-900/50"
            />

        </section>
    }
}
