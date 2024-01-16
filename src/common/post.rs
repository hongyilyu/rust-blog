use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use time::Date;

const BLOG_FILE_PATH: &str = "posts/blogs";
const OPEN_SOURCE_FILE_PATH: &str = "posts/projects";
const BOOK_FILE_PATH: &str = "posts/books";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub description: String,
    #[serde(with = "crate::common::custom_format")]
    pub publication_date: Date,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostAttribute {
    pub post_type: PostType,
    pub uri: String,
    pub reading_time: String,
}

pub type PostContent = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_metadata: PostMetadata,
    pub post_content: PostContent,
    pub post_attribute: PostAttribute,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PostType {
    Blog,
    OpenSource,
    Book,
}

impl PostType {
    pub fn get_file_path(&self) -> &str {
        match self {
            PostType::Blog => BLOG_FILE_PATH,
            PostType::OpenSource => OPEN_SOURCE_FILE_PATH,
            PostType::Book => BOOK_FILE_PATH,
        }
    }

    pub fn from_path(path: &PathBuf) -> Self {
        match path {
            _ if path.starts_with(BLOG_FILE_PATH) => PostType::Blog,
            _ if path.starts_with(OPEN_SOURCE_FILE_PATH) => PostType::OpenSource,
            _ if path.starts_with(BOOK_FILE_PATH) => PostType::Book,
            _ => panic!("Should have a match!"),
        }
    }

    pub fn to_uri_prefix(&self) -> &str {
        match self {
            PostType::Blog => "/posts/",
            PostType::OpenSource => "/projects/",
            PostType::Book => "/books/",
        }
    }
}
