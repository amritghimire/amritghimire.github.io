use yew::{Html, html};

pub fn line_breaks(excerpt: &String, lines: usize) -> Html {
    excerpt
        .lines()
        .take(lines)
        .map(|e| html! {<>{e}<br/></>})
        .collect()
}