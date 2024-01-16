use leptos::*;

#[component]
pub fn YearSplitter(year: i32) -> impl IntoView {
    view! {
        <div class="w-full grid gap-4 mb-6 mt-16 font-bold text-2xl single-line items-center after:text-gray-500 after:border-t-dashed ">
            <span class="text-gray-500">{year}</span>
        </div>
    }
}
