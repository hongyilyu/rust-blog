use leptos::*;
use leptos_router::create_query_signal;

use std::{
    convert::Infallible,
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Tags(pub String);

impl Tags {
    pub fn add_tag(mut self, tag: &str) -> Option<Self> {
        let mut elements: Vec<&str> = if self.0.is_empty() {
            Vec::new()
        } else {
            self.0.split(',').collect()
        };
        if !elements.contains(&tag) {
            elements.push(tag);
            self.0 = elements.join(",");
        }
        Some(self)
    }

    pub fn remove_tag(mut self, s: &str) -> Option<Self> {
        let elements: Vec<&str> = self.0.split(',').collect();
        let filtered_elements: Vec<&str> = elements
            .into_iter()
            .filter(|&element| element != s)
            .collect();

        if filtered_elements.is_empty() {
            None
        } else {
            self.0 = filtered_elements.join(",");
            Some(self)
        }
    }
}

impl FromStr for Tags {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tags(s.to_owned()))
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Tags {
    type Target = String;

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
