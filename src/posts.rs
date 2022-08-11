use crate::content::{Post, PostMeta};

use include_dir::{include_dir, Dir};
use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Result;

const METADATA: &str = include_str!("../contents/metadata.json");
const PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/contents");

#[derive(Serialize, Deserialize)]
struct MetadataJson {
    title: String,
    category: String,
    file: String,
    keywords: Option<String>,
    image_url: Option<String>,
    excerpt: Option<String>,
    show_in_home: Option<bool>,
    created_at: String,
}

impl MetadataJson {
    pub fn post_meta(&self, slug: &String) -> PostMeta {
        PostMeta {
            title: self.title.clone(),
            category: self.category.clone(),
            slug: String::from(slug),
            excerpt: self.excerpt.clone().unwrap_or_else(|| String::from("")),
            image_url: self.image_url.clone(),
            show_in_home: self.show_in_home.unwrap_or(true),
            created_at: self.created_at.clone(),
            keywords: self
                .keywords
                .clone()
                .unwrap_or_else(|| String::from(""))
                .split(',')
                .map(String::from)
                .collect(),
        }
    }
}

type PostMetadata = HashMap<String, MetadataJson>;

pub struct PostGenerator {
    metadata: PostMetadata,
}

impl PostGenerator {
    pub fn new() -> Self {
        let result = Self::get_metadata();
        if let Ok(metadata) = result {
            return Self { metadata };
        }
        Self {
            metadata: HashMap::new(),
        }
    }

    pub fn size(&self, category: Option<String>) -> usize {
        if let Some(category) = category {
            return self
                .metadata
                .values()
                .filter(|meta| meta.category.to_lowercase() == category.to_lowercase())
                .count();
        };
        self.metadata.len()
    }

    fn get_metadata() -> Result<PostMetadata> {
        let metadata: PostMetadata = serde_json::from_str(METADATA)?;
        Ok(metadata)
    }

    pub fn get_post_metadata(&self, slug: &String) -> Option<PostMeta> {
        if let Some(metadata) = self.metadata.get(slug) {
            return Some(metadata.post_meta(slug));
        }
        None
    }

    pub fn get_post(&self, slug: &String) -> Option<Post> {
        if let Some(metadata) = self.metadata.get(slug) {
            let post_meta = metadata.post_meta(slug);
            if let Some(md_content) = PROJECT_DIR.get_file(&metadata.file) {
                let options = Options::all();
                let parser = Parser::new_ext(md_content.contents_utf8().unwrap(), options);

                // Write to String buffer.
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                let post = Post {
                    meta: post_meta,
                    content: html_output,
                };
                return Some(post);
            }
        }
        None
    }

    pub fn get_posts(&self, page: usize, category: Option<String>) -> Vec<PostMeta> {
        let mut skip_count = (page - 1) * 10;
        if skip_count > 0 {
            skip_count -= 1;
        }
        let mut posts: Vec<PostMeta> = self
            .metadata
            .iter()
            .map(|(key, value)| value.post_meta(key))
            .collect();
        if let Some(category) = category {
            posts = posts
                .into_iter()
                .filter(|it| it.category.to_lowercase() == category.to_lowercase())
                .collect();
        }
        posts.sort();
        posts.reverse();
        posts.into_iter().skip(skip_count).take(10).collect()
    }

    pub fn get_posts_for_home(&self) -> Vec<PostMeta> {
        let mut posts: Vec<PostMeta> = self
            .metadata
            .iter()
            .filter(|(_, value)| value.show_in_home.unwrap_or(true))
            .map(|(key, value)| value.post_meta(key))
            .collect();
        posts.sort();
        posts.reverse();
        posts.into_iter().take(9).collect()
    }
}

impl Default for PostGenerator {
    fn default() -> Self {
        Self::new()
    }
}
