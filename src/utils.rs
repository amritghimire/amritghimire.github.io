use time::{format_description, Duration, OffsetDateTime};
use wasm_bindgen::prelude::*;
use yew::{html, Html};

/// Break a long text with line breaks into html br tags
///
/// Some example:
/// ```rust
/// use yew::Html;
/// use yew::utils::print_node;
/// use personal_frontend::utils::line_breaks;
///  let text = "Lorem ipsum dolor sit amet,\n consectetur adipiscing elit.";
///  let html = line_breaks(text, 3);
///  if let Html::VList(nodes) = html{
///     assert_eq!(nodes.len(), 2);
/// } else {
///     panic!("Expected a VList");
/// };
/// ```
pub fn line_breaks(excerpt: &str, lines: usize) -> Html {
    excerpt
        .lines()
        .take(lines)
        .map(|e| html! {<>{e}<br/></>})
        .collect()
}

pub fn humanize_time(datetime: OffsetDateTime) -> String {
    let now = OffsetDateTime::now_utc();
    let suffix = if datetime <= now { "ago" } else { "later" };
    let diff = (datetime - now).abs();

    if diff > Duration::days(365) {
        let format = format_description::parse("[month repr:short] [year]").unwrap();

        return datetime.format(&format).unwrap();
    }

    let human_time = if diff <= Duration::MINUTE {
        item_pluralize(diff.whole_seconds(), "second")
    } else if diff <= Duration::HOUR {
        item_pluralize(diff.whole_minutes(), "minute")
    } else if diff <= Duration::DAY {
        item_pluralize(diff.whole_hours(), "hour")
    } else if diff <= Duration::WEEK {
        item_pluralize(diff.whole_days(), "day")
    } else if diff <= Duration::days(31) {
        item_pluralize(diff.whole_weeks(), "week")
    } else {
        item_pluralize(diff.whole_days() / 31, "month")
    };

    format!("{human_time} {suffix}")
}

fn item_pluralize(value: i64, unit: &str) -> String {
    format!(
        "{} {}",
        value,
        if value <= 1 {
            unit.to_string()
        } else {
            format!("{unit}s")
        }
    )
}

#[wasm_bindgen(
    inline_js = "export function set_title(title) { document.title = title.charAt(0).toUpperCase() + title.slice(1) + ' | Amrit Ghimire, Ranjit'; }"
)]
extern "C" {
    pub fn set_title(title: &str);
}
