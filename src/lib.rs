use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod content;
pub mod pages;
pub mod personal;
pub mod posts;
pub mod utils;

use pages::{home::Home, page_not_found::PageNotFound, post::Post, post_list::PostList};
use yew::html::Scope;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:slug")]
    Post { slug: String },
    #[at("/posts")]
    Posts,
    #[at("/category/:category")]
    Category { category: String },
    #[at("/")]
    Home,
    #[at("/home")]
    LegacyHome,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        {"Developed completely in Rust | Amrit Ghimire, Ranjit "}
                        <p class="copyright pt-4">{"The fingerprint of " } <a href="https://keybase.io/amritghimire">{"GPG key is F83D048426BD7B2A63CDAC0008E895807FE435FE"}</a></p>
                        <p style="font-size: 16px;"><a href="https://keybase.io/amritghimire" target="_blank" rel="noopener">{"amritghimire on Keybase"} </a></p>
                   </div>
                </footer>
            </BrowserRouter>
        }
    }
}
impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-dark is-spaced has-shadow" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">
                        <Link<Route> to={Route::Home}>
                            <span class="has-text-white">{ "Amrit Ghimire" }</span>
                        </Link<Route>>
                    </h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-end">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Category{category: "literature".to_string()}}>
                            { "Literature" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Category{category: "tech".to_string()}}>
                            { "Tech" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Post { slug } => html! { <Post slug={slug} /> },
        Route::Posts => html! { <PostList /> },
        Route::Home => html! { <Home /> },
        Route::LegacyHome => html! { <Home /> },
        Route::NotFound => html! { <PageNotFound /> },
        Route::Category { category } => html! { <PostList category={Some(category)} /> },
    }
}

pub fn main() {
    yew::start_app::<Model>();
}
