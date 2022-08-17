use crate::components::pagination::PageQuery;
use crate::components::{pagination::Pagination, post_card::PostCard};
use crate::posts::PostGenerator;
use crate::utils::set_title;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub const ITEMS_PER_PAGE: usize = 10;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub category: Option<String>,
}

pub enum Msg {
    PageUpdated,
}

pub struct PostList {
    page: usize,
    generator: PostGenerator,
    _listener: HistoryListener,
    category: Option<String>,
}

fn current_page(ctx: &Context<PostList>) -> usize {
    let location = ctx.link().location().unwrap();

    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

impl Component for PostList {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let listener = ctx.link().history().unwrap().listen(move || {
            link.send_message(Msg::PageUpdated);
        });

        let generator = PostGenerator::new();
        let page = current_page(ctx);
        let category: Option<String> = ctx.props().category.clone();

        Self {
            page,
            _listener: listener,
            generator,
            category,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => self.page = current_page(ctx),
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.category = ctx.props().category.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;
        let title = if let Some(c) = &self.category {
            c.clone()
        } else {
            "All Posts".to_string()
        };
        set_title(&title);

        let route_to_page = if let Some(category) = self.category.clone() {
            Route::Category { category }
        } else {
            Route::Posts
        };

        html! {
            <div class="section container">
                <h1 class="title is-capitalized">{ title }</h1>
                { self.view_posts(ctx) }
                <Pagination
                    {page}
                    total_pages={self.generator.size(self.category.clone()) / ITEMS_PER_PAGE + 1}
                    route_to_page={route_to_page}
                />
            </div>
        }
    }
}
impl PostList {
    fn view_posts(&self, _ctx: &Context<Self>) -> Html {
        let posts = self.generator.get_posts(self.page, self.category.clone());

        let mut cards = posts.iter().map(|post| {
            html! {
                <li class="tile is-child is-12">
                    <PostCard slug={post.slug.clone()} />
                </li>
            }
        });
        html! {
            <div class="container">
            <div class="tile is-ancestor">
                    <div class="tile is-parent is-vertical">
                        { for cards.by_ref().take(ITEMS_PER_PAGE as usize / 2) }
                    </div>
                    <div class="tile is-parent is-vertical">
                        { for cards }
                    </div>
                </div>
            </div>
        }
    }
}
