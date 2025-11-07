use crate::utils::{humanize_time, line_breaks};
use crate::{content::PostMeta, posts::PostGenerator, Route};
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
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
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.post = self.generator.get_post_metadata(&ctx.props().slug);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(post) = &self.post {
            let keywords = post.keywords.iter().map(
                |keyword| html! { <span class="tag is-info is-capitalized mx-1">{ keyword }</span> },
            );
            let ht = humanize_time(post.created_at);

            html! {
                <>
                <Link<Route> to={Route::Post { category: post.category.to_lowercase().replace(' ', "_"), slug: post.slug.clone() }}>
                    <div class="card mx-3">
                        <div class="card-header">
                            <div class="card-header-title">
                                <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { category: post.category.to_lowercase().replace(' ', "_"), slug: post.slug.clone() }}>
                                    { &post.title }
                                </Link<Route>>
                            </div>
                            <div class="subtitle is-6" style="padding: 0 1rem;align-self: center;">{ ht } </div>
                        </div>
                        {
                            if let Some(image_url) = &post.image_url {
                                html! {
                                    <div class="card-image">
                                        <figure class="image is-2by1">
                                            <img src={image_url.clone()} alt={post.title.clone()} loading="lazy"/>
                                        </figure>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <div class="card-content">
                            <div class="content">
                                { line_breaks(&post.excerpt.clone(), 5) }
                            </div>
                            {for keywords}
                        </div>
                    </div>
                </Link<Route>>
                </>
            }
        } else {
            html! {}
        }
    }
}
