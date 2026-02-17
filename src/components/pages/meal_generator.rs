#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;

use crate::content::{get_proteins, get_starches, get_vegs, CUISINES};

use super::meal_slot::render_slot;
use super::meal_types::{
    cascade_from_protein, cuisine_label, describe_meal, in_cuisine,
    pairs_with_protein, LockState, MealSelection, SlotCtx,
    SAUCE_SUGGESTIONS,
};

#[component]
pub fn MealGenerator() -> Element {
    let mut cuisine = use_signal(|| CUISINES[0]);
    let mut cuisine_lock = use_signal(|| false);
    let mut selection = use_signal(MealSelection::default);
    let locks = use_signal(LockState::default);
    let mut show_veg2 = use_signal(|| false);
    let mut has_generated = use_signal(|| false);
    let editing = use_signal::<Option<&'static str>>(|| None);

    let ctx = SlotCtx { locks, selection, editing, cuisine, show_veg2 };

    let generate_meal = move |_| {
        editing.clone().set(None);
        let current_cuisine = if *cuisine_lock.read() {
            *cuisine.read()
        } else {
            let mut rng = rand::thread_rng();
            let new_c = *CUISINES.choose(&mut rng).unwrap_or(&CUISINES[0]);
            cuisine.set(new_c);
            new_c
        };

        let proteins = get_proteins();
        let mut rng = rand::thread_rng();
        let p = if locks.read().protein { selection.read().protein }
                else { in_cuisine(current_cuisine, &proteins).choose(&mut rng).copied() };

        let mut sel = MealSelection { protein: p, starch: None, veg1: None, veg2: None };
        let lock = *locks.read();
        if lock.starch { sel.starch = selection.read().starch; }
        if lock.veg1 { sel.veg1 = selection.read().veg1; }
        if lock.veg2 { sel.veg2 = selection.read().veg2; }
        cascade_from_protein(&mut sel, &lock, current_cuisine, *show_veg2.read());
        selection.set(sel);
        has_generated.set(true);
    };

    let toggle_bg = if *cuisine_lock.read() { "background:var(--accent);" } else { "background:var(--bg-elevated);" };
    let toggle_knob = if *cuisine_lock.read() { "transform:translateX(1.25rem);" } else { "" };

    let cur = *cuisine.read();
    let protein_alts = in_cuisine(cur, &get_proteins());
    let starch_alts = pairs_with_protein(selection.read().protein, cur, &get_starches(), &[]);
    let veg1_alts = pairs_with_protein(selection.read().protein, cur, &get_vegs(), &[]);
    let veg2_exc: Vec<&str> = selection.read().veg1.map(|v| v.id).into_iter().collect();
    let veg2_alts = pairs_with_protein(selection.read().protein, cur, &get_vegs(), &veg2_exc);
    let sauces: &[&str] = SAUCE_SUGGESTIONS.iter().find(|(c, _)| *c == cur).map(|(_, s)| *s).unwrap_or(&[]);

    rsx! {
        div { class: "section", style: "padding-top:5rem; max-width:560px; margin:0 auto;",
            div { style: "text-align:center; margin-bottom:2.5rem; animation:fadeSlideUp 0.5s var(--ease-out) both;",
                h1 { style: "font-family:var(--font-display); font-weight:800; font-size:clamp(1.6rem,5vw,2.4rem); letter-spacing:-0.02em; margin-bottom:0.25rem;",
                    "Tell Me What's For Dinner"
                }
                p { style: "color:var(--text-soft); font-size:0.9rem;", "Protein + Starch + Veg" }
            }

            // Cuisine selector
            div { style: "margin-bottom:1.5rem; animation:fadeSlideUp 0.5s var(--ease-out) 0.08s both;",
                div { style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:0.65rem;",
                    span { style: "font-family:var(--font-mono); font-size:0.68rem; font-weight:600; text-transform:uppercase; letter-spacing:0.06em; color:var(--text-muted);", "Cuisine" }
                    label { style: "display:flex; align-items:center; gap:0.5rem; cursor:pointer;",
                        span { style: "font-size:0.72rem; color:var(--text-dim); font-weight:500;",
                            if *cuisine_lock.read() { "Locked" } else { "Random" }
                        }
                        button {
                            onclick: move |_| cuisine_lock.toggle(),
                            style: "width:2.5rem; height:1.25rem; border-radius:9999px; position:relative; border:none; cursor:pointer; transition:background 0.35s var(--ease-smooth); {toggle_bg}",
                            div { style: "position:absolute; top:2px; left:2px; width:1rem; height:1rem; background:white; border-radius:50%; transition:transform 0.35s var(--ease-spring); box-shadow:0 1px 3px rgba(0,0,0,0.12); {toggle_knob}" }
                        }
                    }
                }
                div { style: "display:flex; flex-wrap:wrap; gap:0.45rem;",
                    for c in CUISINES.iter() {
                        { let cls = if *cuisine.read() == *c { "cuisine-pill cuisine-pill--active" } else { "cuisine-pill" };
                          let lbl = cuisine_label(c);
                          rsx! { button { class: "{cls}", onclick: move |_| { cuisine.set(c); }, "{lbl}" } }
                        }
                    }
                }
            }

            button { onclick: generate_meal, class: "btn btn--primary",
                style: "width:100%; margin-bottom:1.75rem; font-size:0.82rem; padding:0.9rem; animation:fadeSlideUp 0.5s var(--ease-out) 0.14s both;",
                if *has_generated.read() { "Generate New Meal" } else { "Generate Meal" }
            }

            if *has_generated.read() {
                if let Some(desc) = describe_meal(&*selection.read()) {
                    p { class: "meal-desc", "{desc}" }
                }
                {render_slot("Protein", "protein", 0, protein_alts, ctx)}
                {render_slot("Starch", "starch", 1, starch_alts, ctx)}
                {render_slot("Vegetable", "veg1", 2, veg1_alts, ctx)}
                if *show_veg2.read() {
                    {render_slot("Extra Vegetable", "veg2", 3, veg2_alts, ctx)}
                    button { class: "veg-toggle-btn", style: "margin-top:0.5rem;",
                        onclick: move |_| show_veg2.set(false), "Remove Extra Vegetable"
                    }
                } else {
                    button { class: "veg-toggle-btn",
                        onclick: move |_| show_veg2.set(true), "+ Add Extra Vegetable"
                    }
                }
                if !sauces.is_empty() {
                    div { class: "seasoning",
                        span { class: "seasoning__title", "Seasoning ideas · {cuisine_label(cur)}" }
                        div { class: "seasoning__pills",
                            for sauce in sauces.iter() {
                                span { class: "seasoning__pill", "{sauce}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
