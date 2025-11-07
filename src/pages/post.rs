use crate::utils::{humanize_time, set_title};
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

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.post = self.generator.get_post(&ctx.props().slug);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(post) = &self.post {
            let keywords = post.meta.keywords.iter().map(
                |keyword| html! { <span class="tag is-info  is-capitalized">{ keyword }</span> },
            );
            set_title(&post.meta.title);
            let ht = humanize_time(post.meta.created_at);

            html! {
                <>
                    <section class="hero is-small is-light has-background">
                        <img class="hero-background is-transparent" src={post.meta.image_url.clone()} />
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    { &post.meta.title }
                                </h1>
                                <div class="tags">
                                    { for keywords }
                                </div>
                                <h5 class="subtitle">
                                    { ht }
                                </h5>
                            </div>
                        </div>
                    </section>
                    <div class="section container">
                        <div class="columns is-centered">
                          <div class="column is-half">
                            <div class="content">
                                <SafeHtml html={post.content.clone()} />
                            </div>
                          </div>
                        </div>
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
