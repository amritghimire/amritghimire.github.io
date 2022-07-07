use stylist::{style, Style};
use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                { self.view_intro() }
                <div class="tile is-parent container">
                    { self.view_blogs() }
                </div>
                <div class="tile is-parent container">
                    { self.view_experiences() }
                </div>
                <div class="tile is-parent container">
                    { self.view_projects() }
                </div>
                <div class="tile is-parent container">
                    { self.view_education() }
                </div>
                <div class="tile is-parent container">
                    { self.view_certifications() }
                </div>
                <div class="tile is-parent container">
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
                <img alt="Fill Murray" src="https://www.fillmurray.com/1920/1080" />
            </figure>
          <div class="hero-body">
            <div class="container has-text-centered">
              <p class="title">
              {"Title"}
              </p>
              <p class="subtitle">
              {"Subtitle"}
              </p>
            </div>
          </div>
        </section>
        }
    }

    fn view_blogs(&self) -> Html {
        html! {
            <section class="section"></section>
        }
    }

    fn view_experiences(&self) -> Html {
        html! {
            <section class="section"></section>
        }
    }

    fn view_projects(&self) -> Html {
        html! {
            <section class="section"></section>
        }
    }

    fn view_education(&self) -> Html {
        html! {
            <section class="section"></section>
        }
    }

    fn view_certifications(&self) -> Html {
        html! {
            <section class="section"></section>
        }
    }

    fn view_contact(&self) -> Html {
        html! {
            <section class="section"></section>
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
