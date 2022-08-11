use yew::{html, Html};

pub fn line_breaks(excerpt: &str, lines: usize) -> Html {
    excerpt
        .lines()
        .take(lines)
        .map(|e| html! {<>{e}<br/></>})
        .collect()
}
