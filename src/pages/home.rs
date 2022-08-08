use stylist::{style, Style};
use yew::prelude::*;

use crate::components::post_card::PostCard;
use crate::personal::generator::Generator;
use crate::posts::PostGenerator;

pub struct Home {
    generator: Generator,
    post_generator: PostGenerator,
}
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let generator = Generator::new();
        let post_generator = PostGenerator::new();

        Self {
            generator,
            post_generator,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <div>
                { self.view_intro() }
                <div class="container">
                    { self.view_blogs() }
                </div>
                <div class="container">
                    { self.view_experiences() }
                </div>
                <div class="container">
                    { self.view_projects() }
                </div>
                <div class="container">
                    { self.view_education() }
                </div>
                <div class="container">
                    { self.view_certifications() }
                </div>
                <div class="container">
                    { self.view_contact() }
                </div>
            </div>
        };
    }
}
impl Home {
    fn view_intro(&self) -> Html {
        return html! {
        <section class={classes!("hero","is-link","is-fullheight-with-navbar", self.intro_style())}>
            <figure class="image is-fullwidth hero-background is-transparent">
                <img alt="Fill Murray" src="/img/amrit.png" />
            </figure>
          <div class="hero-body">
            <div class="container has-text-centered">
              <p class="title is-size-1">
              {"Amrit Ghimire, Ranjit"}
              </p>
              <p class="subtitle is-size-2">
              {&self.generator.website().intro}
              </p>
            </div>
          </div>
        </section>
        };
    }

    fn view_blogs(&self) -> Html {
        let posts = self.post_generator.get_posts_for_home();
        let cards = posts.iter().map(|post| {
            html! {
                <li class="tile is-child is-4">
                    <PostCard slug={post.slug.clone()} />
                </li>
            }
        });

        return html! {
            <section class="section">
            <div class="container">
                <h1 class="title content has-text-centered">{"Some posts so far.."}</h1>
                <div class="tile is-ancestor">
                <div class="tile is-parent is-12 ">
                    { for cards }
                </div>
                </div>
            </div>
            </section>
        };
    }

    fn view_experiences(&self) -> Html {
        return html! {
            <section class="section"></section>
        };
    }

    fn view_projects(&self) -> Html {
        return html! {
            <section class="section"></section>
        };
    }

    fn view_education(&self) -> Html {
        return html! {
            <section class="section"></section>
        };
    }

    fn view_certifications(&self) -> Html {
        return html! {
            <section class="section"></section>
        };
    }

    fn view_contact(&self) -> Html {
        return html! {
            <section class="section"></section>
        };
    }
}

impl Home {
    fn intro_style(&self) -> Style {
        style!(
            r#"
                position: relative;
                overflow: hidden;
                .hero-background {
                  position: absolute;
                  object-fit: cover;
                  object-position: center center;
                  width: 100%;
                  height: 100%;
                }
                .hero-background.is-transparent {
                  opacity: 0.3;
                }
            "#
        )
        .expect("Failed to mount style!")
    }
}
