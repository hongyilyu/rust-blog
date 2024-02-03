use crate::common::post::{Post, PostType};
use leptos::*;

#[server(GetPosts, "/api")]
pub async fn list_posts_metadata(post_type: PostType) -> Result<Vec<Post>, ServerFnError> {
    let path = post_type.get_file_path();
    Ok(process_posts(path))
}

#[server(GetYearPosts, "/api")]
pub async fn list_year_posts_metadata(post_type: PostType, year: i32) -> Result<Vec<Post>, ServerFnError> {
    let path = post_type.get_file_path();
    Ok(process_posts(path).into_iter().filter(|post| post.post_metadata.publication_date.year() == year).collect())
}

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    backend::utils::markdown_parser::get_parsed_content,
    common::{
        post::{PostAttribute, PostMetadata},
        util::calculate_reading_time,
    },
};

use gray_matter::engine::YAML;
use gray_matter::Matter;

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
            if let Some(filename) = p.file_name().and_then(|f| f.to_str()) {
                if filename == "page.md" || filename == "page.mdx" {
                    files.push(p)
                }
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

    let matter = Matter::<YAML>::new();

    let post = matter
        .parse_with_struct::<PostMetadata>(&content)
        .expect("Unable to parse md frontmatter");

    let post_metadata = post.data;
    let post_attribute = get_post_attribute(&post.content, &file_path);
    let post_content = get_parsed_content(&post.content, &file_path);

    Some(Post {
        post_metadata,
        post_content,
        post_attribute,
    })
}

fn get_post_attribute(content: &str, file_path: &PathBuf) -> PostAttribute {
    let post_type = PostType::from_path(file_path);
    let file_path_prefix_removed = std::iter::once(post_type.to_uri_prefix())
        .chain(
            file_path
                .parent()
                .expect("to have a parent directory!")
                .components()
                .skip(2) // skip first two, should be universal to all types
                .map(|comp| comp.as_os_str().to_str().expect("to have valid path str")),
        )
        .collect::<Vec<_>>()
        .join("/");
    let uri = crate::common::util::file_path_to_uri(&file_path_prefix_removed);

    let reading_time = calculate_reading_time(content);

    PostAttribute {
        post_type,
        uri,
        reading_time,
    }
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
