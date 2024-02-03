use leptos::*;
use leptos_router::create_query_signal;

use std::{
    convert::Infallible,
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Tags(Vec<String>);

impl Tags {
    pub fn add_tag(mut self, tag: &String) -> Option<Self> {
        if !self.contains(tag) {
            self.push(tag.clone());
        }
        Some(self)
    }

    pub fn remove_tag(self, s: &str) -> Option<Self> {
        let filtered_elements: Vec<String> = <Vec<String> as Clone>::clone(&self)
            .into_iter()
            .filter(|element| element != s)
            .collect();

        if filtered_elements.is_empty() {
            None
        } else {
            Some(Tags(filtered_elements))
        }
    }
}

impl FromStr for Tags {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Tags(Vec::new()))
        } else {
            Ok(Tags(s.split(',').map(|tag| tag.to_owned()).collect()))
        }
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if self.is_empty() {
            String::new()
        } else {
            self.join(",")
        };
        write!(f, "{s}")
    }
}

impl Deref for Tags {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const TAG_CLASS: &str = "flex h-5 items-center px-2 
                        bg-indigo-50 text-indigo-800 
                        dark:bg-indigo-950 dark:text-indigo-200 
                        rounded-md text-xs font-semibold";

#[component]
pub fn TagAppend(#[prop(into)] tag: String) -> impl IntoView {
    let class_name = TAG_CLASS;
    let (tags, set_tags) = create_query_signal::<Tags>("tags");
    let tc = tag.clone();
    let new_val = move || tags.get().unwrap_or_default().add_tag(&tc);
    let on_click = move |_| {
        set_tags.set(new_val());
    };

    view! {
        <button class=class_name on:click=on_click>
            {tag}
        </button>
    }
}

#[component]
pub fn TagRemove(#[prop(into)] tag: String) -> impl IntoView {
    let class_name = TAG_CLASS;
    let (tags, set_tags) = create_query_signal::<Tags>("tags");
    let tc = tag.clone();
    let new_val = move || tags.get().unwrap_or_default().remove_tag(&tc);
    let on_click = move |_| {
        set_tags.set(new_val());
    };

    view! {
        <button class=class_name on:click=on_click>
            <span class="no-underline hover:line-through">{tag}</span>
        </button>
    }
}

#[component]
pub fn TagStatic(#[prop(into)] tag: String) -> impl IntoView {
    let class_name = TAG_CLASS;

    view! { <span class=class_name>{tag}</span> }
}
