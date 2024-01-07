use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub description: String,
    pub reading_time: String,
    #[serde(with = "crate::common::custom_format")]
    pub publication_date: Date,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostAttribute {
    pub file_path: PathBuf,
}

pub type PostContent = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_metadata: PostMetadata,
    pub post_content: PostContent,
    pub post_attribute: PostAttribute
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PostType {
    Blog,
    OpenSource,
    Book,
}
