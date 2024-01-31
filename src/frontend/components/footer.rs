use leptos::*;
use leptos_icons::*;

struct Social<'a> {
    name: &'a str,
    url: &'a str,
    icon: icondata::Icon,
}

#[component]
fn Svg(social: Social<'static>) -> impl IntoView {
    view! {
        <a
            key=social.name
            href=social.url
            class="hover:-translate-y-1 inline-flex justify-center items-center w-10 h-10 text-center rounded-full  transition text-gray-500 hover:text-gray-200 "
        >
            <Icon icon=social.icon/>
        </a>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    let socials = [
        Social {
            name: "github",
            url: "https://github.com/hongyilyu",
            icon: icondata::FiGithub,
        },
        Social {
            name: "linkedin",
            url: "https://linkedin.com/in/hongyilyu",
            icon: icondata::FiLinkedin,
        },
        Social {
            name: "twitter",
            url: "https://twitter.com/LHY_IS_LEARNING",
            icon: icondata::FiTwitter,
        },
    ];

    view! {
        <footer class="m-8">
            <nav class="flex flex-row justify-center gap-2 p-2">
                {socials.into_iter().map(|social| view! { <Svg social=social/> }).collect_view()}
            </nav>
            <p class="text-center text-xs leading-5 text-gray-500">
                {format!("© {}, Hongyi Lyu • ", time::OffsetDateTime::now_utc().year())}
                <a href="https://github.com/hongyilyu/hongy.io" class="font-mono">
                    local
                </a>
            </p>
        </footer>
    }
}
