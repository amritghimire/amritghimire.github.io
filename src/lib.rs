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
    #[at("/:category/:slug")]
    Post { category: String, slug: String },
    #[at("/posts/:slug")]
    LegacyPost { slug: String },
    #[at("/posts")]
    Posts,
    #[at("/category/:category")]
    Category { category: String },
    #[at("/tag/:tag")]
    Tag { tag: String },
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
                    <Switch<Route> render={switch} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        <p>{"© 2026 Amrit Ghimire | Built with Rust"}</p>
                        <div class="footer-social">
                            <a href="https://github.com/amritghimire" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                                <i class="fab fa-github"></i>
                            </a>
                            <a href="https://www.linkedin.com/in/iamritghimire/" target="_blank" rel="noopener noreferrer" aria-label="LinkedIn">
                                <i class="fab fa-linkedin"></i>
                            </a>
                            <a href="mailto:contact_me@amritghimire.com" aria-label="Email">
                                <svg width="20" height="20" viewBox="0 0 512 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48H48zM0 176V384c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V176L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/></svg>
                            </a>
                            <a href="/feed.xml" target="_blank" rel="noopener noreferrer" aria-label="RSS Feed">
                                <svg width="20" height="20" viewBox="0 0 448 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M0 64C0 46.3 14.3 32 32 32c229.8 0 416 186.2 416 416c0 17.7-14.3 32-32 32s-32-14.3-32-32C384 253.6 258.4 128 32 128c-17.7 0-32-14.3-32-32zM0 416a64 64 0 1 1 128 0A64 64 0 1 1 0 416zM0 208c0-17.7 14.3-32 32-32c158.8 0 288 129.2 288 288c0 17.7-14.3 32-32 32s-32-14.3-32-32C256 310.6 185.4 240 32 240c-17.7 0-32-14.3-32-32z"/></svg>
                            </a>
                        </div>
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
                        <span class="navbar-item" onclick={link.callback(|_| Msg::ToggleNavbar)}>
                            <Link<Route> to={Route::Home}>
                                { "Home" }
                            </Link<Route>>
                        </span>
                        <span class="navbar-item" onclick={link.callback(|_| Msg::ToggleNavbar)}>
                            <Link<Route> to={Route::Category{category: "english_literature".to_string()}}>
                                { "Literature" }
                            </Link<Route>>
                        </span>
                        <span class="navbar-item" onclick={link.callback(|_| Msg::ToggleNavbar)}>
                            <Link<Route> to={Route::Category{category: "literature".to_string()}}>
                                { "साहित्य" }
                            </Link<Route>>
                        </span>
                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                                <span class="navbar-link-text" style="margin-right: 1.5rem;">{ "Finrup" }</span>
                            </a>
                            <div class="navbar-dropdown">
                                <span class="navbar-item" onclick={link.callback(|_| Msg::ToggleNavbar)}>
                                    <Link<Route> to={Route::Tag{tag: "finrup".to_string()}}>
                                        { "Finrup Blog Posts" }
                                    </Link<Route>>
                                </span>
                                <a class="navbar-item" href="/finrup-budgeting-tips.html">
                                    { "Best Budgeting Tips" }
                                </a>

                                <hr class="navbar-divider" />
                                <a class="navbar-item" href="https://apps.apple.com/in/app/finrup/id6752817572" target="_blank" rel="noopener noreferrer">
                                    { "Download Finrup" }
                                </a>
                            </div>
                        </div>
                        <span class="navbar-item" onclick={link.callback(|_| Msg::ToggleNavbar)}>
                            <Link<Route> to={Route::Category{category: "tech".to_string()}}>
                                { "Tech" }
                            </Link<Route>>
                        </span>
                    </div>
                </div>
            </nav>
        }
    }
}

#[allow(clippy::let_unit_value)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Post { slug, .. } => html! { <Post slug={slug} /> },
        Route::LegacyPost { slug } => html! { <Post slug={slug} /> },
        Route::Posts => html! { <PostList /> },
        Route::Home => html! { <Home/> },
        Route::LegacyHome => html! { <Home/> },
        Route::NotFound => html! { <PageNotFound/> },
        Route::Category { category } => html! { <PostList category={Some(category)} /> },
        Route::Tag { tag } => html! { <PostList tag={Some(tag)} /> },
    }
}

pub fn main() {
    yew::Renderer::<Model>::new().render();
}
