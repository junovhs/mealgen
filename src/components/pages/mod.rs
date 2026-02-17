#![allow(non_snake_case)]

mod meal_types;
mod meal_slot;
mod meal_generator;
mod icons;

pub use meal_generator::MealGenerator;

use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            style: "min-height: 60vh; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; animation: fadeSlideUp 0.5s var(--ease-out) both;",
            h1 {
                style: "font-family: var(--font-display); font-size: 8rem; font-weight: 800; color: var(--bg-elevated);",
                "404"
            }
            p {
                style: "font-size: 1rem; color: var(--text-soft); margin-bottom: 2rem;",
                "The page you're looking for doesn't exist."
            }
            a { class: "btn btn--primary", href: "/", "Return Home" }
        }
    }
}
