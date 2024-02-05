use std::path::Path;

use pulldown_cmark::{html, Event, Options, Parser, Tag};

pub fn get_parsed_content(content: &str, file_path: &Path) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);

    let parser = parser.map(|event| match event {
        Event::Start(tag) => match &tag {
            Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => {
                let dst = if dest_url.starts_with("./") {
                    format!(
                        "/static/{}/{}",
                        file_path
                            .strip_prefix("posts")
                            .expect("to always start with `posts`")
                            .parent()
                            .expect("to have parent")
                            .to_str()
                            .expect("to have string"),
                        dest_url.strip_prefix("./").expect("to have string")
                    )
                } else {
                    dest_url.to_string()
                };
                Event::Start(Tag::Image {
                    link_type: *link_type,
                    dest_url: dst.into(),
                    title: title.clone(),
                    id: id.clone(),
                })
            }
            _ => Event::Start(tag),
        },
        _ => event,
    });

    let mut post_content = String::new();

    html::push_html(&mut post_content, parser);

    post_content
}
