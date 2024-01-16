use crate::common::post::{Post, PostType};
#[cfg(feature = "ssr")]
use crate::common::post::{PostAttribute, PostMetadata};
use leptos::*;

#[server(GetSinglePost, "/api")]
pub async fn get_single_post(post_type: PostType, post_uri: String) -> Result<Option<Post>, ServerFnError> {
    let posts = list_posts_metadata(post_type).await.expect("to have posts!");
    Ok(posts.into_iter().find(|post| post.post_attribute.uri == post_uri))
}

#[server(GetPosts, "/api")]
pub async fn list_posts_metadata(post_type: PostType) -> Result<Vec<Post>, ServerFnError> {
    let path = post_type.get_file_path();
    Ok(process_posts(path))
}

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};

fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(PathBuf)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(path);
            }
        }
    }
    Ok(())
}
fn get_post_files(relative_path: &str) -> Vec<PathBuf> {
    let path = Path::new(relative_path).to_path_buf();
    let mut files = Vec::new();
    {
        let mut add_to_vec = |p: PathBuf| {
            if p.extension() == Some("md".as_ref()) {
                files.push(p)
            }
        };
        let _ = visit_dirs(&path, &mut add_to_vec);
    }
    files
}
fn read_post_content(entry: &PathBuf) -> Option<String> {
    fs::read_to_string(entry).ok()
}

fn parse_post_content(file_path: PathBuf) -> Option<Post> {
    let content = match read_post_content(&file_path) {
        Some(v) => v,
        None => return None,
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let matter = Matter::<YAML>::new();

    let post_data = matter
        .parse_with_struct::<PostMetadata>(&content)
        .expect("Unable to parse md frontmatter");
    let post_metadata = post_data.data;

    let content = post_data.content;

    let parser = Parser::new_ext(&content, options);
    let mut post_content = String::new();
    html::push_html(&mut post_content, parser);

    let post_type = PostType::from_path(&file_path);
    let file_path_prefix_removed = file_path
        .components()
        .skip(2) // skip first two, should be universal to all types
        .map(|comp| comp.as_os_str().to_str().expect("to have valid path str"))
        .collect::<Vec<_>>()
        .join("/");
    let uri = post_type.to_uri_prefix().to_owned()
        + &crate::common::util::file_path_to_uri(&file_path_prefix_removed);

    let post_attribute = PostAttribute { uri, post_type };

    Some(Post {
        post_metadata,
        post_content,
        post_attribute,
    })
}

fn sort_posts(posts: &mut [Post]) {
    posts.sort_by(|a, b| {
        //reverse sorting
        a.post_metadata
            .publication_date
            .cmp(&b.post_metadata.publication_date)
    });
}

fn process_posts(path: &str) -> Vec<Post> {
    let posts_path = get_post_files(path);

    let mut posts: Vec<Post> = posts_path
        .into_iter()
        .filter_map(parse_post_content)
        .collect();

    sort_posts(&mut posts);

    posts
}
}}
