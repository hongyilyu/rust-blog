use leptos::*;

use crate::frontend::components::featured_post::FeaturedPost;

// TODO: Maybe add another button at bottom routing to all posts
#[component]
pub fn About() -> impl IntoView {
    let featured_post = [1, 2, 3];
    view! {
        <div class="mx-auto py-16 sm:py-24 lg:py-28">
            <div class="text-center  prose dark:prose-invert md:prose-lg prose-h1:font-bold prose-img:rounded mx-auto px-2 max-w-2xl">
                <h1 class="text-4xl tracking-tight sm:text-6xl">"Hi there, I’m Hongyi 👋"</h1>
                <p class="mt-6 text-lg leading-8">
                    "Welcome to my digital garden – a space where I document my ongoing projects and share the knowledge I've gathered along the way."
                </p>
            </div>
            <div>
                <h2 class="text-center">Featured Posts</h2>
                <section>
                    {featured_post
                        .into_iter()
                        .map(|_post| view! { <FeaturedPost/> })
                        .collect_view()}
                </section>
            </div>
        </div>
    }
}
