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
    pub tag: Option<String>,
}

pub enum Msg {
    PageUpdated,
}

pub struct PostList {
    page: usize,
    generator: PostGenerator,
    _listener: LocationHandle,
    category: Option<String>,
    tag: Option<String>,
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
        let listener = ctx
            .link()
            .add_location_listener(link.callback(move |_| Msg::PageUpdated))
            .unwrap();

        let generator = PostGenerator::new();
        let page = current_page(ctx);
        let category: Option<String> = ctx.props().category.clone();
        let tag: Option<String> = ctx.props().tag.clone();

        Self {
            page,
            _listener: listener,
            generator,
            category,
            tag,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => self.page = current_page(ctx),
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.category.clone_from(&ctx.props().category);
        self.tag.clone_from(&ctx.props().tag);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;
        let title = if let Some(t) = &self.tag {
            format!("#{}", t.clone().replace('-', " "))
        } else if let Some(c) = &self.category {
            c.clone().replace('_', " ")
        } else {
            "All Posts".to_string()
        };
        set_title(&title);

        let route_to_page = if let Some(tag) = self.tag.clone() {
            Route::Tag { tag }
        } else if let Some(category) = self.category.clone() {
            Route::Category { category }
        } else {
            Route::Posts
        };

        html! {
            <div class="section container">
                <h1 class="title is-capitalized mb-6">{ title }</h1>
                { self.view_posts(ctx) }
                <div class="mt-6">
                    <Pagination
                        {page}
                        total_pages={self.generator.size_filtered(self.category.clone(), self.tag.clone()) / ITEMS_PER_PAGE + 1}
                        route_to_page={route_to_page}
                    />
                </div>
            </div>
        }
    }
}

impl PostList {
    fn view_posts(&self, _ctx: &Context<Self>) -> Html {
        let posts =
            self.generator
                .get_posts_filtered(self.page, self.category.clone(), self.tag.clone());
        let mut odd = Vec::new();
        let mut even = Vec::new();

        for (i, post) in posts.iter().enumerate() {
            if i % 2 == 0 {
                even.push(html! {
                    <li class="tile is-child is-12">
                        <PostCard slug={post.slug.clone()} />
                    </li>
                });
            } else {
                odd.push(html! {
                    <li class="tile is-child is-12">
                        <PostCard slug={post.slug.clone()} />
                    </li>
                });
            }
        }

        html! {
            <div class="container">
                <div class="tile is-ancestor">
                    <div class="tile is-parent is-vertical">
                        { for even }
                    </div>
                    <div class="tile is-parent is-vertical">
                        { for odd }
                    </div>
                </div>
            </div>
        }
    }
}
