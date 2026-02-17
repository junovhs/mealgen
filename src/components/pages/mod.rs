#![allow(non_snake_case)]

mod home;
mod meal_generator;
mod sections;

pub use home::Home;
pub use meal_generator::MealGenerator;

use crate::content::{load_about_content, load_work_content};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let content = load_about_content();

    rsx! {
        section { class: "section", style: "padding-top: 10rem;",
            div { class: "container", style: "max-width: 800px;",
                h1 { class: "section-title", style: "margin-bottom: 2rem;", "{content.title}" }
                p { style: "font-size: 1.1rem; color: var(--text-soft); margin-bottom: 2rem; line-height: 1.8;",
                    "{content.bio}"
                }
                p { style: "color: var(--text-dim);",
                    "📍 {content.location}"
                }
            }
        }
    }
}

#[component]
pub fn Work() -> Element {
    let content = load_work_content();

    rsx! {
        section { class: "section", style: "padding-top: 10rem;",
            div { class: "container", style: "max-width: 900px;",
                h1 { class: "section-title", "{content.title}" }
                p { style: "font-size: 1.1rem; color: var(--text-soft); margin-bottom: 3rem;",
                    "{content.intro}"
                }

                div { class: "product-grid", style: "grid-template-columns: 1fr;",
                    for service in &content.services {
                        div { class: "product-card",
                            h3 { class: "product-card__name", "{service.name}" }
                            p { class: "product-card__desc", "{service.description}" }
                            if let Some(price) = &service.price {
                                p { style: "font-family: var(--font-mono); color: var(--accent); margin-top: 1rem;",
                                    "{price}"
                                }
                            }
                        }
                    }
                }

                div { style: "text-align: center; margin-top: 4rem;",
                    a { class: "btn btn--primary btn--large", href: "mailto:{content.contact_email}",
                        "Get in Touch"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Tools() -> Element {
    rsx! {
        section { class: "section", style: "padding-top: 10rem;",
            div { class: "container",
                div { class: "section-header",
                    span { class: "section-label", "Tools" }
                    h2 { class: "section-title", "Free Tools" }
                    p { class: "section-desc",
                        "Browser-based utilities. No sign-up, no upload, complete privacy."
                    }
                }

                div { class: "product-grid",
                    div { class: "product-card",
                        div { class: "product-card__icon", "D" }
                        h3 { class: "product-card__name", "DirAnalyze" }
                        span { class: "product-card__role", "File Analysis" }
                        p { class: "product-card__desc",
                            "Analyze and combine directory contents for AI context generation."
                        }
                        a { class: "product-card__cta", href: "https://diranalyze.semmap.dev", target: "_blank", "Open Tool" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { style: "min-height: 60vh; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center;",
            h1 { style: "font-family: var(--font-display); font-size: 8rem; font-weight: 800; color: var(--bg-elevated);",
                "404"
            }
            p { style: "font-size: 1rem; color: var(--text-soft); margin-bottom: 2rem;",
                "The page you're looking for doesn't exist."
            }
            a { class: "btn btn--primary", href: "/", "Return Home" }
        }
    }
}
