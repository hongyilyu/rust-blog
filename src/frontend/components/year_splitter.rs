use leptos::*;
use leptos_router::*;

use crate::frontend::pages::year_posts::YearParams;

#[component]
pub fn YearSplitter(year: i32) -> impl IntoView {
    let params = use_params::<YearParams>();
    let param_year = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.year)
                .unwrap_or_default()
        })
    };

    view! {
        <div class="w-full grid gap-4 mb-6 mt-16 font-bold text-2xl single-line items-center after:text-gray-500 after:border-t-dashed ">
            <A
                id="year_tag"
                href=move || if year != param_year() { year.to_string() } else { "".to_string() }
                class="no-underline"
            >
                <span class="text-gray-500">{year}</span>
            </A>
        </div>
    }
}
