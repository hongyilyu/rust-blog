use leptos::*;

use crate::{
    common::post::Post,
    frontend::components::{post_preview::PostPreview, year_splitter::YearSplitter},
};

#[component]
pub fn PostPreviewByYear(year: i32, posts: Vec<Post>) -> impl IntoView {
    if posts.is_empty() {
        view! {}.into_view()
    } else {
        view! {
            <YearSplitter year/>
            {move || {
                posts
                    .iter()
                    .map(|post| {
                        view! { <PostPreview post=post.clone()/> }
                    })
                    .collect_view()
            }}
        }
        .into_view()
    }
}
