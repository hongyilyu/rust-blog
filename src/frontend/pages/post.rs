use leptos::*;
use leptos_router::use_location;

use crate::{
    backend::server_functions::post::list_posts_metadata,
    common::post::PostType,
    error_template::{AppError, ErrorTemplate},
    frontend::components::post_header::PostHeader,
};

#[component]
pub fn Post() -> impl IntoView {
    let featured_post = create_resource(
        || (),
        |_| async { list_posts_metadata(PostType::Blog).await },
    );
    view! {
        <div>
            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    featured_post
                        .and_then(|posts| {
                            let post = posts
                                .iter()
                                .find(|post| {
                                    post.post_attribute.uri
                                        == use_location().pathname.get_untracked().as_str()
                                });
                            if let Some(post) = post {
                                view! {
                                    <article>
                                        <PostHeader post=post.clone()/>
                                        <div inner_html=post.post_content.clone()></div>
                                    </article>
                                }
                                    .into_view()
                            } else {
                                let mut outside_errors = Errors::default();
                                outside_errors.insert_with_default_key(AppError::NotFound);
                                view! { <ErrorTemplate outside_errors/> }.into_view()
                            }
                        })
                }}

            </Transition>
        </div>
    }
}
