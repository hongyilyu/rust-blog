use leptos::*;
use leptos_icons::*;
use leptos_use::storage::{use_local_storage, StringCodec};

use crate::frontend::components::buttons::icon_button::IconButton;

#[derive(PartialEq, Clone, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl std::str::FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dark" => Ok(Self::Dark),
            "Light" => Ok(Self::Light),
            _ => Err("Failed to convert to Theme".to_owned()),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Dark => write!(f, "Dark"),
            Theme::Light => write!(f, "Light"),
        }
    }
}

fn set_light_theme() {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .remove_1("dark")
        .unwrap();
}

fn set_dark_theme() {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .add_1("dark")
        .unwrap();
}

// TODO: Why local storage not working.
#[component]
pub fn ThemeControl() -> impl IntoView {
    let (theme, set_theme, _) = use_local_storage::<Theme, StringCodec>("theme");
    create_effect(move |_| match theme.get() {
        Theme::Light => set_light_theme(),
        Theme::Dark => set_dark_theme(),
    });
    let on_click = move |_: web_sys::MouseEvent| {
        match theme.get() {
            Theme::Dark => set_theme(Theme::Light),
            Theme::Light => set_theme(Theme::Dark),
        };
    };
    view! {
        <IconButton on:click=on_click>
            {move || match theme.get() {
                Theme::Dark => view! { <Icon icon=Icon::Fi(FiIcon::FiMoon)/> },
                Theme::Light => view! { <Icon icon=Icon::Fi(FiIcon::FiSun)/> },
            }}

        </IconButton>
    }
}
