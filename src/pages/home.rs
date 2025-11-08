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
                { self.view_blogs() }
                { self.view_experiences() }
                { self.view_projects() }
                { self.view_education() }
                { self.view_certifications() }
                { self.view_contact() }
            </div>
        }
    }
}
impl Home {
    fn view_intro(&self) -> Html {
        html! {
        <section class={classes!("hero","is-link","is-fullheight-with-navbar", "has-background", self.intro_style())}>
            <figure class="image is-fullwidth hero-background is-transparent">
                <img alt="Amrit" src="/img/amrit.webp" loading="eager" fetchpriority="high" />
            </figure>
          <div class="hero-body">
            <div class="container has-text-centered">
              <p class="subtitle is-size-2 fade-in-on-scroll">
                {&self.generator.website().pre_intro}
              </p>
              <p class="title is-size-1 fade-in-on-scroll">
              {"Amrit Ghimire, Ranjit"}
              </p>
              <p class="subtitle is-size-2 fade-in-on-scroll">
                {&self.generator.website().post_intro}
              </p>
            </div>
          </div>
          <div class="hero-footer">
            <div class="scroll-indicator">
              <span class="scroll-text">{"Scroll to explore"}</span>
              <div class="scroll-arrow">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M7 10L12 15L17 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </div>
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
            <section class="section fade-in-on-scroll">
            <div class="container">
                <h1 class="title content has-text-centered mb-6">{"Few of my attempts.."}</h1>
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
                <div class="has-text-centered mt-5">
                    <Link<Route> classes={classes!("is-centered", "is-text-centered")} to={Route::Posts}>
                        <button class="button is-large is-info">{ "View more..." }</button>
                    </Link<Route>>
                </div>
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
                      <img src={experience.logo.clone()} alt={format!("{} logo", experience.company.clone())} loading="lazy" decoding="async" />
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
            <section class="section fade-in-on-scroll">
                <div class="container">
                    <h1 class="title content has-text-centered mb-6">{"Some of my journey"}</h1>
                    <div class="columns is-centered">
                      <div class="column is-half">
                        <div class="experience-timeline">
                          {for cards}
                        </div>
                        <div class="divider"></div>
                        <div class="columns is-centered">
                          <div class="column is-half">
                            <div class="has-text-centered mt-4">
                                <a target="_blank" class="button is-info" href={self.generator.website().linkedin.clone()}>
                                    <span class="icon">
                                      <i class="fab fa-linkedin"></i>
                                    </span>
                                    <span>{"View more on LinkedIn"}</span>
                                </a>
                            </div>
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
            <section class="section fade-in-on-scroll">
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
                <div class="column is-6">
                    <div class="education-card">
                        <div class="education-icon">
                            <i class="fas fa-graduation-cap"></i>
                        </div>
                        <div class="education-content">
                            <h3 class="education-institute">{&e.institute}</h3>
                            <div class="education-degree">
                                <span class="degree-title">{&e.title}</span>
                                <span class="degree-subject">{&e.subject}</span>
                            </div>
                            <div class="education-period">
                                <i class="fas fa-calendar-alt"></i>
                                <span>{&e.start}{" - "}{&e.end}</span>
                            </div>
                        </div>
                    </div>
                </div>
            }
        });

        html! {
            <section class="section education-section fade-in-on-scroll">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Education"}</h1>
                  <div class="columns is-centered">
                    <div class="column is-10">
                      <div class="columns is-multiline is-vcentered">
                        { for cards }
                      </div>
                    </div>
                  </div>
                </div>
            </section>
        }
    }

    fn view_certifications(&self) -> Html {
        let certification = self.generator.certifications();
        let cards = certification.iter().map(|c| {
            html! {
                <div class="column is-6">
                    <div class="certification-card">
                        <div class="certification-icon">
                            <i class="fas fa-certificate"></i>
                        </div>
                        <div class="certification-content">
                            <a class="certification-title" target="_blank" href={c.link.clone()}>
                                {&c.title}
                            </a>
                            <div class="certification-meta">
                                <span class="certification-issuer">
                                    <i class="fas fa-building"></i>
                                    {&c.issuer}
                                </span>
                                <span class="certification-date">
                                    <i class="fas fa-calendar"></i>
                                    {&c.issued_at}
                                </span>
                            </div>
                        </div>
                        <div class="certification-link-icon">
                            <i class="fas fa-external-link-alt"></i>
                        </div>
                    </div>
                </div>
            }
        });

        html! {
            <section class="section certifications-section fade-in-on-scroll">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Certifications"}</h1>
                  <div class="columns is-centered">
                    <div class="column is-10">
                      <div class="columns is-multiline">
                        { for cards }
                      </div>
                    </div>
                  </div>
                </div>
            </section>
        }
    }

    fn view_contact(&self) -> Html {
        html! {
            <section class="section contact-section fade-in-on-scroll">
                <div class="container">
                  <h1 class="title content has-text-centered mb-6">{"Find me at"}</h1>
                    <div class="columns is-centered">
                      <div class="column is-8">
                          <div class="social-links">
                            <a class="social-link github-link" href={self.generator.website().github.clone()} target="_blank" rel="noopener">
                                <div class="social-icon">
                                  <i class="fab fa-github"></i>
                                </div>
                                <div class="social-content">
                                  <span class="social-label">{"GitHub"}</span>
                                  <span class="social-desc">{"View my code"}</span>
                                </div>
                            </a>
                            <a class="social-link linkedin-link" href={self.generator.website().linkedin.clone()} target="_blank" rel="noopener">
                                <div class="social-icon">
                                  <i class="fab fa-linkedin"></i>
                                </div>
                                <div class="social-content">
                                  <span class="social-label">{"LinkedIn"}</span>
                                  <span class="social-desc">{"Connect with me"}</span>
                                </div>
                            </a>
                            <a class="social-link email-link" href={self.generator.website().email.clone()}>
                                <div class="social-icon">
                                  <i class="fas fa-envelope"></i>
                                </div>
                                <div class="social-content">
                                  <span class="social-label">{"Email"}</span>
                                  <span class="social-desc">{"Get in touch"}</span>
                                </div>
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
            "#
        )
        .expect("Failed to mount style!")
    }
}
