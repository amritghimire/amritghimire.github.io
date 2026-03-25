use rss::{CategoryBuilder, ChannelBuilder, GuidBuilder, ItemBuilder};
use serde::{Deserialize, Serialize};
use sitewriter::{UrlEntry, UrlEntryBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

const METADATA: &str = include_str!("./contents/metadata.json");
const WEBSITE: &str = "https://amritghimire.com/";

#[derive(Serialize, Deserialize)]
struct MetadataJson {
    created_at: String,
    category: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    excerpt: Option<String>,
    #[serde(default)]
    keywords: Option<String>,
    #[serde(default)]
    tags: Option<String>,
}

type PostMetadata = HashMap<String, MetadataJson>;

fn main() -> Result<(), Box<dyn Error>> {
    let metadata: PostMetadata = serde_json::from_str(METADATA)?;
    let mut url_links = vec![format!("/posts/")];
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
        let category = meta.category.to_lowercase().replace(' ', "_");
        let url = format!("{}{}/{}/", WEBSITE, category, slug);
        url_links.push(format!("/{}/{}/", category, slug));
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
        url_links.push(format!("/category/{}/", category));
        urls.push(UrlEntry {
            loc: url.parse().unwrap(),
            changefreq: None,
            priority: Some(0.9),
            lastmod: None,
        });
    }

    let buffer = File::create("sitemap.xml")?;
    sitewriter::generate(buffer, &urls)?;
    let mut buffer = File::create("sitemap.txt")?;
    writeln!(buffer, "{}", WEBSITE)?;
    for link in url_links {
        writeln!(buffer, "{}{}", WEBSITE, link.trim_start_matches('/'))?;
    }

    // Generate RSS feeds
    generate_feed(&metadata, "feed.xml", "Amrit Ghimire", None)?;
    generate_feed(
        &metadata,
        "feed-tech.xml",
        "Amrit Ghimire - Tech",
        Some(&["tech"]),
    )?;
    generate_feed(
        &metadata,
        "feed-literature.xml",
        "Amrit Ghimire - Literature",
        Some(&["literature", "english_literature"]),
    )?;

    Ok(())
}

fn generate_feed(
    metadata: &PostMetadata,
    filename: &str,
    title: &str,
    categories: Option<&[&str]>,
) -> Result<(), Box<dyn Error>> {
    let mut items: Vec<(String, rss::Item)> = Vec::new();
    for (slug, meta) in metadata.iter() {
        if let Some(cats) = categories {
            if !cats.iter().any(|c| c.eq_ignore_ascii_case(&meta.category)) {
                continue;
            }
        }
        let category = meta.category.to_lowercase().replace(' ', "_");
        let url = format!("{}{}/{}/", WEBSITE, category, slug);

        let mut rss_categories = vec![CategoryBuilder::default().name(&meta.category).build()];

        if let Some(ref tags) = meta.tags {
            for tag in tags.split(',').map(|t| t.trim()).filter(|t| !t.is_empty()) {
                rss_categories.push(CategoryBuilder::default().name(tag).build());
            }
        }
        if let Some(ref keywords) = meta.keywords {
            for kw in keywords
                .split(',')
                .map(|k| k.trim())
                .filter(|k| !k.is_empty())
            {
                rss_categories.push(CategoryBuilder::default().name(kw).build());
            }
        }

        let guid = GuidBuilder::default().value(&url).permalink(true).build();

        let item = ItemBuilder::default()
            .title(meta.title.clone())
            .link(Some(url))
            .description(meta.excerpt.clone())
            .pub_date(Some(meta.created_at.clone()))
            .categories(rss_categories)
            .guid(Some(guid))
            .build();
        items.push((meta.created_at.clone(), item));
    }
    items.sort_by(|a, b| b.0.cmp(&a.0));
    let rss_items: Vec<rss::Item> = items.into_iter().map(|(_, item)| item).collect();

    let channel = ChannelBuilder::default()
        .title(title)
        .link(WEBSITE)
        .description(format!("Blog posts by {}", title))
        .items(rss_items)
        .build();

    let rss_file = File::create(filename)?;
    channel.write_to(rss_file)?;
    Ok(())
}
