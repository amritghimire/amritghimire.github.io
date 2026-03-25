use yew::prelude::*;

const FOLLOWIT_ALL: &str = "https://api.follow.it/subscription-form/eXpGc3hVd09uUXdVL1pLVmkrbmJGYWhQNDRVbUpkNmtDVEJUeGlBT1JoMVZoenFQYWVFeVQvUHowY3UrY2EwSjZlZjBwSG1Bc2w2YjUreW9LZjRPNGZwa1Z5UnN4M1VISzllYnlLSjRBWmo1b01rRTNJTDdxOW1HM0FPYlZFK1p8eVlrYUJDemJHNDQ2Ym5QZmMxcGx5VlNnNVZLeUJLMGluemNmWWZkdXcvdz0=/8";
const FOLLOWIT_TECH: &str = "https://api.follow.it/subscription-form/NVlWWXpGOFNhQm1NU2ZWVTIwc24zRzBxMHN5VVFOcFBzTkVuWUxDekJqei8vNnFETzNSdUNKc29VeW9xeTZWWTVFcHQvWmRwRURhNjM1b1RPQWd4a3lwdjYrTXVVaE50QXp6YzcxVlZLVG02VlJJdjhpOGxDTVhtQ3hJN1ovYWZ8SHVZUDFkUFJreXc0TzlRcThac0FrK21qcnRHaGloZWoxaE5rZlNta1ZHaz0=/8";
const FOLLOWIT_LITERATURE: &str = "https://api.follow.it/subscription-form/K2NreWMwZC9SelN1ak5nVjdwOUh3OW5ZWGNZTm1EdS8wZVhrOFVjY1hwYTVHbnFZdnl2NDBHSFFhV1pmWW1uYnNaNGRKUnR6bGtTRjMvTW4yQzdDb1dubEdBUHd3OU56N1A2cmhRaHljUzJMYVdlT2ozSGVrY1VoakIyR2VRZnV8a3M3M29TN0sxYjljS2xEaVFmQkF2OVlQSEJrYUZhcjU2SWJVbnlzdE04cz0=/8";

#[derive(Clone, PartialEq, Eq)]
pub enum NewsletterFeed {
    All,
    Tech,
    Literature,
}

impl NewsletterFeed {
    pub fn action_url(&self) -> &'static str {
        match self {
            NewsletterFeed::All => FOLLOWIT_ALL,
            NewsletterFeed::Tech => FOLLOWIT_TECH,
            NewsletterFeed::Literature => FOLLOWIT_LITERATURE,
        }
    }

    pub fn heading(&self) -> &'static str {
        match self {
            NewsletterFeed::All => "Get new posts by email:",
            NewsletterFeed::Tech => "Get new tech posts by email:",
            NewsletterFeed::Literature => "Get new literature posts by email:",
        }
    }

    pub fn from_category(category: &str) -> Option<Self> {
        match category.to_lowercase().as_str() {
            "tech" => Some(NewsletterFeed::Tech),
            "literature" | "english_literature" => Some(NewsletterFeed::Literature),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub feed: NewsletterFeed,
}

#[function_component(Newsletter)]
pub fn newsletter(props: &Props) -> Html {
    let feed = &props.feed;

    html! {
        <div class="newsletter-form">
            <form action={feed.action_url()} method="post">
                <h5 class="newsletter-heading">{feed.heading()}</h5>
                <div class="newsletter-input">
                    <input type="email" name="email" required={true} placeholder="Enter your email" spellcheck="false" />
                </div>
                <div class="newsletter-submit">
                    <button type="submit">{"Subscribe"}</button>
                </div>
            </form>
            <a href="https://follow.it" class="newsletter-powered-by" target="_blank" rel="noopener noreferrer">
                {"Powered by "}
                <img src="https://follow.it/images/colored-logo.svg" alt="follow.it" height="17" />
            </a>
        </div>
    }
}
