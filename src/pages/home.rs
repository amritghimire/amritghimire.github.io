use stylist::{style, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::post_card::PostCard;
use crate::personal::generator::Generator;
use crate::posts::PostGenerator;
use crate::utils::line_breaks;
use crate::Route;

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
        html! {
            <div>
                { self.view_intro() }
                <div class="container">
                    { self.view_experiences() }
                </div>
                <div class="container">
                    { self.view_blogs() }
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
        }
    }
}
impl Home {
    fn view_intro(&self) -> Html {
        html! {
        <section class={classes!("hero","is-link","is-fullheight-with-navbar", self.intro_style())}>
            <figure class="image is-fullwidth hero-background is-transparent">
                <img alt="Amrit" src="/img/amrit.webp" />
            </figure>
          <div class="hero-body">
            <div class="container has-text-centered">
              <p class="subtitle is-size-2">
                {&self.generator.website().pre_intro}
              </p>
              <p class="title is-size-1">
              {"Amrit Ghimire, Ranjit"}
              </p>
              <p class="subtitle is-size-2">
                {&self.generator.website().post_intro}
              </p>
            </div>
          </div>
        </section>
        }
    }

    fn view_blogs(&self) -> Html {
        let posts = self.post_generator.get_posts_for_home();
        let mut cards = posts.iter().map(|post| {
            html! {
                <li class="tile is-child is-4">
                    <PostCard slug={post.slug.clone()} />
                </li>
            }
        });

        html! {
            <section class="section">
            <div class="container">
                <h1 class="title content has-text-centered">{"Few of my attempts.."}</h1>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        { for cards.by_ref().take(3) }
                    </div>
                </div>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        { for cards.by_ref().take(3) }
                    </div>
                </div>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        { for cards.by_ref().take(3) }
                    </div>
                </div>
                <Link<Route> classes={classes!("is-centered", "is-text-centered")} to={Route::Posts}>
                    <button class="button is-large my-3 px-6">{ "View more..." }</button>
                </Link<Route>>
            </div>
            </section>
        }
    }

    fn view_experiences(&self) -> Html {
        let experiences = self.generator.experiences();
        let cards = experiences.iter().map(|experience| {
            html! {
                <article class="media is-flex-mobile is-flex-wrap-wrap-mobile">
                  <figure class="media-left">
                    <p class="image is-64x64 mx-3">
                      <img src={experience.logo.clone()} />
                    </p>
                  </figure>
                  <div class="media-content">
                    <div class="content">
                      <p>
                        <strong class="mr-2">{&experience.company} </strong>
                        <small>{&experience.location.clone().unwrap_or_default()} </small>
                         <small>{" ("}{&experience.start}{"-"}{&experience.end}{")"}</small>
                        <br/>
                        <strong class="is-large"><u>{&experience.title}</u></strong><br/>
                        <p class="content mt-2">{line_breaks(&experience.description, 50)}</p>
                      </p>
                    </div>
                  </div>
                </article>
            }
        });

        html! {
            <section class="section">
                <div class="container">
                    <h1 class="title content has-text-centered mb-6">{"Some of my journey"}</h1>
                    <div class="columns is-centered">
                      <div class="column is-half">
                        <p class="bd-notification is-primary">
                          {for cards}
                        </p>
                        <div class="divider"></div>
                        <div class="columns is-centered">
                          <div class="column is-half">
                            <p class="bd-notification is-primary">
                                <a style="display:block;" target="_blank" class="link centered-link ml-6 mt-4" href={self.generator.website().linkedin.clone()}>{"View more on LinkedIn"}</a>
                            </p>
                          </div>
                        </div>
                      </div>
                    </div>
                </div>
            </section>
        }
    }

    fn view_projects(&self) -> Html {
        let projects = self.generator.projects();
        let cards = projects.iter().map(|project| {
            html! {
                <div class="column is-6">
                  <div class="box">
                    <div class="columns">
                        <div class="column">
                            <a href={project.link.clone()}><span class="title" target="_blank">{&project.title}</span></a>
                            <span class="tag is-dark is-rounded ml-3">{&project.language}</span>
                        </div>
                    </div>
                    <div class="columns">
                        <div class="column">
                            <p class="content">{line_breaks(&project.description, 5)}</p>
                        </div>
                    </div>
                    <div class="columns">
                        <div class="column">
                            <div class="tags">
                                {project.tags.iter().map(|tag| {
                                    html! {
                                        <span class="tag is-light is-rounded">{tag}</span>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    </div>
                  </div>
                </div>
            }
        });
        html! {
            <section class="section">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Notable Projects"}</h1>
                  <div class="columns" style="flex-wrap: wrap;">
                    { for cards }
                  </div>
                </div>

            </section>
        }
    }

    fn view_education(&self) -> Html {
        let education = self.generator.education();
        let cards = education.iter().map(|e| {
            html! {
                <div>
                    <strong>{&e.institute}</strong><small>{" ("}{&e.start}{"-"}{&e.end}{")"}</small><br/>
                    <span class="mr-2">{&e.title}</span><strong><u>{&e.subject}</u></strong>
                </div>
            }
        });

        html! {
            <section class="section">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Education"}</h1>
                  <div class="has-text-centered">
                    { for cards }
                  </div>
                </div>
            </section>
        }
    }

    fn view_certifications(&self) -> Html {
        let certification = self.generator.certifications();
        let cards = certification.iter().map(|c| {
            html! {
                <div>
                    <a class="link" target="_blank" href={c.link.clone()}>{&c.title}</a>
                    <span class="mx-2">{&c.issuer}</span><small>{"("}{&c.issued_at}{")"}</small>
                </div>
            }
        });

        html! {
            <section class="section">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Certifications"}</h1>
                  <div class="has-text-centered">
                    { for cards }
                  </div>
                </div>
            </section>
        }
    }

    fn view_contact(&self) -> Html {
        html! {
            <section class="section">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Find me at"}</h1>
                    <div class="columns is-centered">
                      <div class="column is-half">
                          <div class="buttons" style="justify-content:center;">
                            <a  class="button is-info" href={self.generator.website().github.clone()}>
                                <span class="icon">
                                  <i class="fab fa-github"></i>
                                </span>
                                <span>{"Github"}</span>
                            </a>
                            <a  class="button is-info" href={self.generator.website().linkedin.clone()}>
                                <span class="icon">
                                  <i class="fab fa-linkedin"></i>
                                </span>
                                <span>{"LinkedIn"}</span>
                            </a>
                            <a  class="button is-info" href={self.generator.website().email.clone()}>
                                <span class="icon">
                                  <i class="fab fa-envelope"></i>
                                </span>
                                <span>{"Email"}</span>
                            </a>
                          </div>
                      </div>
                    </div>
                </div>
            </section>
        }
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
