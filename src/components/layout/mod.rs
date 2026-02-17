#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav { class: "nav",
            a { class: "nav__logo", href: "/",
                "MEAL"
                span { class: "nav__logo-accent", "GEN" }
            }
            div { class: "nav__links",
                a { class: "nav__link nav__link--active", href: "/", "Generator" }
                a { class: "nav__link", href: "https://github.com/semmaplabs", target: "_blank", "GitHub" }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: "footer-wrapper",
            div { class: "footer-logo", "MEALGEN" }
            footer { class: "footer",
                div { class: "footer__bottom",
                    span { "© 2024 MealGen. Built with Dioxus." }
                }
            }
        }
    }
}
