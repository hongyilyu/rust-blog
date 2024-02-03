use icondata::Icon;
use leptos::*;
use leptos_icons::Icon;

use crate::frontend::components::buttons::button_icon::ButtonIcon;

use super::{ButtonColor, ButtonSize, ButtonStyle, ButtonVarient};

#[component]
pub fn ButtonWrapper(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] color: ButtonColor,
    #[prop(optional)] varient: ButtonVarient,
    #[prop(optional)] left_icon: Option<Icon>,
    #[prop(optional)] right_icon: Option<Icon>,
    #[prop(optional, into)] extra_css: String,
    #[prop(optional, into)] icon_extra_css: String,
    children: Children,
) -> impl IntoView {
    let mut class_name = format!(
        "{extra_css}
        inline-flex appearance-none
        items-center justify-center rounded
        font-medium
        leading-5
        transition-colors ease-out
        disabled:cursor-not-allowed disabled:opacity-40 disabled:shadow-none
        {size}
        {}
    ",
        ButtonStyle::from_varient_color(&varient, &color)
    );
    if varient == ButtonVarient::OUTLINE {
        class_name = format!(
            "{class_name} {}",
            ButtonStyle::from_varient_color(&ButtonVarient::GHOST, &color)
        );
    }

    view! {
        <button class=class_name>
            {if let Some(left) = left_icon {
                view! {
                    <ButtonIcon extra_css=format!("{icon_extra_css} mr-2")>
                        <Icon icon=left/>
                    </ButtonIcon>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }}
            {children()}
            {if let Some(right) = right_icon {
                view! {
                    <ButtonIcon extra_css=format!("{icon_extra_css} ml-2")>
                        <Icon icon=right/>
                    </ButtonIcon>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }}

        </button>
    }
}
