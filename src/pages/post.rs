use crate::{components::safe_html::SafeHtml, content, posts::PostGenerator, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub slug: String,
}

pub struct Post {
    generator: PostGenerator,
    post: Option<content::Post>,
}
impl Component for Post {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let generator = PostGenerator::new();
        let post = generator.get_post(&ctx.props().slug);
        Self { generator, post }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.post = self.generator.get_post(&ctx.props().slug);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(post) = &self.post {
            let keywords = post
                .meta
                .keywords
                .iter()
                .map(|keyword| html! { <span class="tag is-info">{ keyword }</span> });

            html! {
                <>
                    <section class="hero is-medium is-light has-background">
                        <img alt="The hero's background" class="hero-background is-transparent" src={post.meta.image_url.clone()} />
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    { &post.meta.title }
                                </h1>
                                <div class="tags">
                                    { for keywords }
                                </div>
                            </div>
                        </div>
                    </section>
                    <div class="section container">
                        <SafeHtml html={post.content.clone()} />
                    </div>
                </>
            }
        } else {
            html! {
                    <Redirect<Route> to={Route::NotFound}/>

            }
        }
    }
}
