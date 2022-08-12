use serde::{Deserialize, Serialize};
use sitewriter::{UrlEntry, UrlEntryBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

const METADATA: &str = include_str!("./contents/metadata.json");
const WEBSITE: &str = "https://amritghimire.com/";

#[derive(Serialize, Deserialize)]
struct MetadataJson {
    created_at: String,
    category: String,
}

type PostMetadata = HashMap<String, MetadataJson>;

fn main() -> Result<(), Box<dyn Error>> {
    let metadata: PostMetadata = serde_json::from_str(METADATA)?;
    let mut urls = vec![
        UrlEntryBuilder::default()
            .loc(WEBSITE.parse().unwrap())
            .priority(1.0)
            .build()
            .unwrap(),
        UrlEntry {
            loc: format!("{}posts/", WEBSITE).parse().unwrap(),
            lastmod: None,
            changefreq: None,
            priority: Some(0.9),
        },
    ];

    for (slug, meta) in metadata.iter() {
        let url = format!("{}posts/{}/", WEBSITE, slug);
        urls.push(UrlEntry {
            loc: url.parse().unwrap(),
            changefreq: None,
            priority: Some(0.8),
            lastmod: Some(meta.created_at.parse().unwrap()),
        });
    }

    let mut categories = metadata
        .values()
        .map(|k| k.category.clone())
        .collect::<Vec<String>>();
    categories.sort();
    categories.dedup();

    for category in categories {
        let url = format!("{}category/{}/", WEBSITE, category);
        urls.push(UrlEntry {
            loc: url.parse().unwrap(),
            changefreq: None,
            priority: Some(0.9),
            lastmod: None,
        });
    }

    let buffer = File::create("sitemap.xml")?;
    sitewriter::generate(buffer, &urls)?;

    Ok(())
}
