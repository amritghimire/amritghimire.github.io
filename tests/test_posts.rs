use personal_frontend::pages::post_list::ITEMS_PER_PAGE;
use personal_frontend::posts;

#[test]
fn post_generator() {
    let post_generator = posts::PostGenerator::new();

    let total_pages = post_generator.size(None) / ITEMS_PER_PAGE + 1;
    for page in 1..total_pages {
        let posts = post_generator.get_posts(page, None);
        assert_ne!(posts.len(), 0);

        for meta in posts.iter() {
            assert_ne!(post_generator.get_post(&meta.slug), None);
            assert_ne!(post_generator.get_post_metadata(&meta.slug), None);
        }
    }
}
