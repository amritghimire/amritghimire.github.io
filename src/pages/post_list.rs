use crate::components::pagination::PageQuery;
use crate::components::{pagination::Pagination, post_card::PostCard};
use crate::posts::PostGenerator;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

const ITEMS_PER_PAGE: usize = 10;

pub enum Msg {
    PageUpdated,
}

pub struct PostList {
    page: usize,
    generator: PostGenerator,
    _listener: HistoryListener,
}

fn current_page(ctx: &Context<PostList>) -> usize {
    let location = ctx.link().location().unwrap();

    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let listener = ctx.link().history().unwrap().listen(move || {
            link.send_message(Msg::PageUpdated);
        });
        let generator = PostGenerator::new();

        Self {
            page: current_page(ctx),
            _listener: listener,
            generator,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => self.page = current_page(ctx),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;

        html! {
            <div class="section container">
                <h1 class="title">{ "Posts" }</h1>
                <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
                { self.view_posts(ctx) }
                <Pagination
                    {page}
                    total_pages={self.generator.size() / ITEMS_PER_PAGE + 1}
                    route_to_page={Route::Posts}
                />
            </div>
        }
    }
}
impl PostList {
    fn view_posts(&self, _ctx: &Context<Self>) -> Html {
        let posts = self.generator.get_posts(self.page);
        let mut cards = posts.iter().map(|post| {
            html! {
                <li class="list-item mb-5">
                    <PostCard slug={post.slug.clone()} />
                </li>
            }
        });
        html! {
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for cards.by_ref().take(ITEMS_PER_PAGE as usize / 2) }
                    </ul>
                </div>
                <div class="column">
                    <ul class="list">
                        { for cards }
                    </ul>
                </div>
            </div>
        }
    }
}
