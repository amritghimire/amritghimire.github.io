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
