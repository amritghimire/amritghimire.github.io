use stylist::{style, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::newsletter::{Newsletter, NewsletterFeed};
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
            <div class="home-sections">
                { self.view_intro() }
                { self.view_apps() }
                { self.view_blogs() }
                { self.view_experiences() }
                { self.view_projects() }
                { self.view_education_and_certs() }
                { Self::view_newsletter() }
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
            <div class="container hero-text-left">
              <p class="subtitle is-size-2 fade-in-on-scroll">
                {&self.generator.website().pre_intro}
              </p>
              <p class="title is-size-1 fade-in-on-scroll">
              {"Amrit Ghimire, Ranjit"}
              </p>
              <p class="subtitle is-size-2 fade-in-on-scroll">
                {&self.generator.website().post_intro}
              </p>
              <div class="hero-cta fade-in-on-scroll">
                <a href="#projects" class="button is-large is-info">{"View Projects"}</a>
                <a href="#contact" class="button is-large is-outlined is-light">{"Get in Touch"}</a>
              </div>
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

    fn view_apps(&self) -> Html {
        let apps: Vec<_> = self
            .generator
            .projects()
            .iter()
            .filter(|p| p.language == "swift")
            .collect();

        let cards = apps.iter().map(|app| {
            html! {
                <div class="column is-4">
                    <div class="app-card">
                        <h3 class="app-card-title">{&app.title}</h3>
                        <p class="app-card-desc">{&app.description}</p>
                        <a class="app-card-link" href={app.link.clone()} target="_blank" rel="noopener noreferrer">
                            <svg width="16" height="16" viewBox="0 0 384 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M318.7 268.7c-.2-36.7 16.4-64.4 50-84.8-18.8-26.9-47.2-41.7-84.7-44.6-35.5-2.8-74.3 20.7-88.5 20.7-15 0-49.4-19.7-76.4-19.3C63.3 141.2 4 183 4 257.4c0 23.4 4.6 47.6 13.9 72.6 12.3 33.3 56.7 114.9 103 113.5 24.6-.6 42-17.2 75.4-17.2 32.7 0 49.2 17.2 76.4 16.6 44.8-.7 84.3-73.6 96-107-29.4-14.5-49.3-43.7-49.3-77.4 0-33.6 20-62.5 49.3-77.4zM262.1 128c27.8-32.6 24.8-62.4 24-72.5-24 1.3-51.9 16.4-67.9 34.9-17.5 19.8-27.8 44.3-25.6 71.9 26.1 2 49.9-11.4 69.5-34.3z"/></svg>
                            <span>{"App Store"}</span>
                        </a>
                    </div>
                </div>
            }
        });

        html! {
            <section class="section apps-strip fade-in-on-scroll">
                <div class="container">
                    <h2 class="title content has-text-centered mb-6">{"Featured Apps"}</h2>
                    <div class="columns is-centered">
                        { for cards }
                    </div>
                </div>
            </section>
        }
    }

    fn view_blogs(&self) -> Html {
        let posts = self.post_generator.get_posts_for_home();

        let featured = posts.first().map(|post| {
            html! {
                <div class="featured-post">
                    <PostCard slug={post.slug.clone()} />
                </div>
            }
        });

        let smaller: Vec<_> = posts
            .iter()
            .skip(1)
            .take(4)
            .map(|post| {
                html! {
                    <li class="column is-6">
                        <PostCard slug={post.slug.clone()} />
                    </li>
                }
            })
            .collect();

        html! {
            <section class="section fade-in-on-scroll">
            <div class="container">
                <h2 class="title content has-text-centered mb-6">{"Latest Posts"}</h2>
                { for featured }
                <ul class="columns is-multiline blog-grid" style="list-style: none; padding: 0;">
                    { for smaller }
                </ul>
                <div class="has-text-centered mt-5">
                    <Link<Route> classes={classes!("is-centered", "is-text-centered")} to={Route::Posts}>
                        <button class="button is-large is-info">{ "View all posts" }</button>
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
                    <h2 class="title content has-text-centered mb-6">{"Professional Experience"}</h2>
                    <div class="columns is-centered">
                      <div class="column is-half">
                        <div class="experience-timeline">
                          {for cards}
                        </div>
                        <div class="divider"></div>
                        <div class="columns is-centered">
                          <div class="column is-half">
                            <div class="has-text-centered mt-4">
                                <a target="_blank" rel="noopener noreferrer" class="button is-info" href={self.generator.website().linkedin.clone()}>
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
                            <a href={project.link.clone()} target="_blank" rel="noopener noreferrer"><span class="title">{&project.title}</span></a>
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
            <section class="section fade-in-on-scroll" id="projects">
                <div class="container">
                  <h2 class="title content has-text-centered mb-6">{"Projects & Apps"}</h2>
                  <div class="columns" style="flex-wrap: wrap;">
                    { for cards }
                  </div>
                </div>
            </section>
        }
    }

    fn view_education_and_certs(&self) -> Html {
        let education = self.generator.education();
        let edu_cards = education.iter().map(|e| {
            html! {
                <div class="education-card mb-4">
                    <div class="education-icon">
                        <svg width="28" height="28" viewBox="0 0 640 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M320 32L0 192l128 69.1V352c0 35.3 86 64 192 64s192-28.7 192-64V261.1L640 192 320 32zm192 256c0 17.7-85.9 32-192 32S128 305.7 128 288V283l192 104 192-104v5zm48-128L320 288 80 160l240-128 240 128z"/></svg>
                    </div>
                    <div class="education-content">
                        <h3 class="education-institute">{&e.institute}</h3>
                        <div class="education-degree">
                            <span class="degree-title">{&e.title}</span>
                            <span class="degree-subject">{&e.subject}</span>
                        </div>
                        <div class="education-period">
                            <svg width="14" height="14" viewBox="0 0 448 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M152 64H296V24C296 10.7 306.7 0 320 0s24 10.7 24 24V64h40c35.3 0 64 28.7 64 64v48H0V128c0-35.3 28.7-64 64-64h40V24C104 10.7 114.7 0 128 0s24 10.7 24 24V64zM0 192H448V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V192z"/></svg>
                            <span>{&e.start}{" - "}{&e.end}</span>
                        </div>
                    </div>
                </div>
            }
        });

        let certification = self.generator.certifications();
        let cert_cards = certification.iter().map(|c| {
            html! {
                <div class="certification-card mb-4">
                    <div class="certification-icon">
                        <svg width="24" height="24" viewBox="0 0 512 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M211 7.3C205 1 196-1.4 187.6 .8s-14.9 8.9-17.1 17.3L154.7 80.6l-62-17.5c-8.4-2.4-17.4 0-23.5 6.1s-8.5 15.1-6.1 23.5l17.5 62L18.1 170.6c-8.4 2.2-14.4 9.2-17.3 17.1S-1 205 7.3 211L64 256 7.3 301C1 307-1.4 316 .8 324.4s8.9 14.9 17.3 17.1l62.5 15.8-17.5 62c-2.4 8.4 0 17.4 6.1 23.5s15.1 8.5 23.5 6.1l62-17.5 15.8 62.5c2.2 8.4 9.2 14.4 17.1 17.3s17-.2 23.4-6.6L256 448l45 56.7c6.4 6.4 15.5 8.8 23.4 6.6s14.9-8.9 17.1-17.3l15.8-62.5 62 17.5c8.4 2.4 17.4 0 23.5-6.1s8.5-15.1 6.1-23.5l-17.5-62 62.5-15.8c8.4-2.2 14.4-9.2 17.3-17.1s-.2-17-6.6-23.4L448 256l56.7-45c6.4-6.4 8.8-15.5 6.6-23.4s-8.9-14.9-17.3-17.1l-62.5-15.8 17.5-62c2.4-8.4 0-17.4-6.1-23.5s-15.1-8.5-23.5-6.1l-62 17.5L341.4 18.1c-2.2-8.4-9.2-14.4-17.1-17.3S307 1 301 7.3L256 64 211 7.3z"/></svg>
                    </div>
                    <div class="certification-content">
                        <a class="certification-title" target="_blank" rel="noopener noreferrer" href={c.link.clone()}>
                            {&c.title}
                        </a>
                        <div class="certification-meta">
                            <span class="certification-issuer">
                                <svg width="12" height="12" viewBox="0 0 384 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M48 0C21.5 0 0 21.5 0 48V464c0 26.5 21.5 48 48 48h96V432c0-26.5 21.5-48 48-48s48 21.5 48 48v80h96c26.5 0 48-21.5 48-48V48c0-26.5-21.5-48-48-48H48zM64 240c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V240zm112-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16V240c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H272c-8.8 0-16-7.2-16-16V240zM80 96h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16V112zM272 96h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H272c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16z"/></svg>
                                {&c.issuer}
                            </span>
                            <span class="certification-date">
                                <svg width="12" height="12" viewBox="0 0 448 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M152 64H296V24C296 10.7 306.7 0 320 0s24 10.7 24 24V64h40c35.3 0 64 28.7 64 64v48H0V128c0-35.3 28.7-64 64-64h40V24C104 10.7 114.7 0 128 0s24 10.7 24 24V64zM0 192H448V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V192z"/></svg>
                                {&c.issued_at}
                            </span>
                        </div>
                    </div>
                    <div class="certification-link-icon">
                        <svg width="14" height="14" viewBox="0 0 512 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M320 0c-17.7 0-32 14.3-32 32s14.3 32 32 32h82.7L201.4 265.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L448 109.3V192c0 17.7 14.3 32 32 32s32-14.3 32-32V32c0-17.7-14.3-32-32-32H320zM80 32C35.8 32 0 67.8 0 112V432c0 44.2 35.8 80 80 80H400c44.2 0 80-35.8 80-80V320c0-17.7-14.3-32-32-32s-32 14.3-32 32V432c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16H192c17.7 0 32-14.3 32-32s-14.3-32-32-32H80z"/></svg>
                    </div>
                </div>
            }
        });

        html! {
            <section class="section edu-certs-section fade-in-on-scroll">
                <div class="container">
                  <h2 class="title content has-text-centered mb-6">{"Education & Certifications"}</h2>
                  <div class="columns is-centered">
                    <div class="column is-5 education-section">
                      <h3 class="subtitle has-text-centered mb-4">{"Education"}</h3>
                      { for edu_cards }
                    </div>
                    <div class="column is-5 certifications-section">
                      <h3 class="subtitle has-text-centered mb-4">{"Certifications"}</h3>
                      { for cert_cards }
                    </div>
                  </div>
                </div>
            </section>
        }
    }

    fn view_newsletter() -> Html {
        html! {
            <section class="section newsletter-section fade-in-on-scroll">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-6">
                            <Newsletter feed={NewsletterFeed::All} />
                        </div>
                    </div>
                </div>
            </section>
        }
    }

    fn view_contact(&self) -> Html {
        html! {
            <section class="section contact-section fade-in-on-scroll" id="contact">
                <div class="container">
                  <h2 class="title content has-text-centered mb-6">{"Find me at"}</h2>
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
                                  <svg width="24" height="24" viewBox="0 0 512 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48H48zM0 176V384c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V176L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/></svg>
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
