use leptos::*;

#[component]
pub fn PostPreview(heading: String) -> impl IntoView {
    let heading_class = "mb-1 text-2xl font-bold";
    let title = "Test Title";
    let reading_time = "10 min read";
    let head = match heading.as_ref() {
        "h3" => view! { <h3 class=heading_class>{title}</h3> }.into_view(),
        _ => view! { <h2 class=heading_class>{title}</h2> }.into_view(),
    };
    view! {
        <article class="not-prose">
            <hgroup>
                {head} <figcaption class="flex flex-wrap gap-y-1 text-sm text-gray-500">
                    <span class="mr-2 inline-block">
                        <span className="font-semibold italic text-amber-600">Unpublished</span>

        " • "{reading_time}
                    </span>
                </figcaption> <p class="mt-3 leading-relaxed">test description</p>
            </hgroup>
        </article>
    }
}
