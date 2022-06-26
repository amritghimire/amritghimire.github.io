use crate::{content::PostMeta, posts::PostGenerator, Route};
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub slug: String,
}

pub struct PostCard {
    generator: PostGenerator,
    post: Option<PostMeta>,
}
impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let generator = PostGenerator::new();
        let post = generator.get_post_metadata(&ctx.props().slug);

        Self { post, generator }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.post = self.generator.get_post_metadata(&ctx.props().slug);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(post) = &self.post {
            return html! {
                <div class="card">
                    <div class="card-image">
                        <figure class="image is-2by1">
                            <img alt="This post's image" src={post.image_url.clone()} loading="lazy" />
                        </figure>
                    </div>
                    <div class="card-content">
                        <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { slug: post.slug.clone() }}>
                            { &post.title }
                        </Link<Route>>
                    </div>
                </div>
            };
        } else {
            return html! {};
        }
    }
}
