use yew::Properties;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PostMeta {
    pub title: String,
    pub category: String,
    pub slug: String,
    pub keywords: Vec<String>,
    pub image_url: String,
    pub excerpt: String,
    pub show_in_home: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}
