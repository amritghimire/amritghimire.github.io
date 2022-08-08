use std::cmp::Ordering;
use yew::Properties;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PostMeta {
    pub title: String,
    pub category: String,
    pub slug: String,
    pub keywords: Vec<String>,
    pub image_url: Option<String>,
    pub excerpt: String,
    pub show_in_home: bool,
    pub created_at: String,
}

impl Ord for PostMeta {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialOrd<Self> for PostMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}
