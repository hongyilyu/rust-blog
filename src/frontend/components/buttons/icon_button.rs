use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn IconButton(
    children: Children,
    #[prop(optional)] on_click: Option<fn(MouseEvent)>,
) -> impl IntoView {
    view! {
        <div class="inline-flex appearance-none items-center justify-center
        font-medium transition-colors ease-out disabled:cursor-not-allowed disabled:opacity-40 disabled:shadow-none text-md min-w-[2.5rem] h-10 
        text-gray-800 dark:text-white/90 hover:bg-gray-100 dark:hover:bg-gray-800 active:bg-gray-200 dark:active:bg-gray-700 rounded-full">
            <button
                class="p-0"
                on:click=move |e| {
                    if let Some(ref click_handler) = on_click {
                        click_handler(e)
                    }
                }
            >

                {children()}
            </button>
        </div>
    }
}
