#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;

use crate::content::{get_proteins, get_starches, get_vegs, CUISINES};

use super::meal_slot::render_slot;
use super::meal_types::{
    cascade_from_protein, cuisine_label, describe_meal, in_cuisine,
    LockState, MealSelection, SlotCtx,
    SAUCE_SUGGESTIONS,
};
use super::generator_logic::generate_slot_options;

use super::icons::ICON_CHEVRON_DOWN;

#[component]
pub fn MealGenerator() -> Element {
    let mut cuisine = use_signal(|| *CUISINES.first().unwrap_or(&"american"));
    let mut cuisine_lock = use_signal(|| false);
    let mut selection = use_signal(MealSelection::default);
    let locks = use_signal(LockState::default);
    let mut show_veg2 = use_signal(|| false);
    let mut has_generated = use_signal(|| false);
    let editing = use_signal::<Option<&'static str>>(|| None);
    let mut cuisine_open = use_signal(|| false);

    let ctx = SlotCtx { locks, selection, editing, cuisine, show_veg2 };

    let generate_meal = move |_| {
        editing.clone().set(None);
        let current_cuisine = if *cuisine_lock.read() {
            *cuisine.read()
        } else {
            let mut rng = rand::thread_rng();
            let new_c = *CUISINES.choose(&mut rng).unwrap_or_else(|| CUISINES.first().unwrap_or(&"american"));
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
        cascade_from_protein(&mut sel, lock, current_cuisine, *show_veg2.read());
        selection.set(sel);
        has_generated.set(true);
    };

    let toggle_bg = if *cuisine_lock.read() { "background:var(--accent);" } else { "background:var(--bg-elevated);" };
    let toggle_knob = if *cuisine_lock.read() { "transform:translateX(1.25rem);" } else { "" };

    let cur = *cuisine.read();
    let cur_label = cuisine_label(cur);
    let sel_val = *selection.read();

    // Generate option lists with compatibility metadata
    let protein_alts = generate_slot_options(
        &get_proteins(),
        None, // Proteins check against cuisine directly
        cur,
        &[],
    );

    let starch_alts = generate_slot_options(
        &get_starches(),
        sel_val.protein,
        cur,
        &[],
    );

    let veg1_alts = generate_slot_options(
        &get_vegs(),
        sel_val.protein,
        cur,
        &[],
    );

    let veg2_exc: Vec<&str> = sel_val.veg1.map(|v| v.id).into_iter().collect();
    let veg2_alts = generate_slot_options(
        &get_vegs(),
        sel_val.protein,
        cur,
        &veg2_exc,
    );

    let sauces: &[&str] = SAUCE_SUGGESTIONS.iter().find(|(c, _)| *c == cur).map_or(&[], |(_, s)| *s);
    let pills_cls = if *cuisine_open.read() { "cuisine-pills-row cuisine-pills-row--open" } else { "cuisine-pills-row" };

    rsx! {
        div { class: "section generator-page",
            div { class: "generator-header", style: "animation:fadeSlideUp 0.5s var(--ease-out) both;",
                h1 { class: "generator-title",
                    "What's For Dinner?"
                }
                p { class: "generator-subtitle", "Protein + Starch + Veg" }
            }

            // Cuisine selector
            div { class: "cuisine-section", style: "animation:fadeSlideUp 0.5s var(--ease-out) 0.08s both;",
                div { class: "cuisine-label-row",
                    span { class: "cuisine-label-text", "Cuisine" }
                    // Compact mobile button — shows current cuisine
                    button {
                        class: "cuisine-compact-btn",
                        onclick: move |_| cuisine_open.toggle(),
                        "{cur_label}"
                        span { class: "cuisine-compact-arrow", dangerous_inner_html: ICON_CHEVRON_DOWN }
                    }
                    label { class: "cuisine-lock-toggle",
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
                div { class: "{pills_cls}",
                    for c in CUISINES.iter() {
                        { let cls = if *cuisine.read() == *c { "cuisine-pill cuisine-pill--active" } else { "cuisine-pill" };
                          let lbl = cuisine_label(c);
                          rsx! { button { class: "{cls}", onclick: move |_| { cuisine.set(c); cuisine_open.set(false); }, "{lbl}" } }
                        }
                    }
                }
            }

            button { onclick: generate_meal, class: "btn btn--primary generate-btn",
                style: "animation:fadeSlideUp 0.5s var(--ease-out) 0.14s both;",
                if *has_generated.read() { "Generate New Meal" } else { "Generate Meal" }
            }

            if *has_generated.read() {
                if let Some(desc) = describe_meal(&selection.read()) {
                    p { key: "{desc}", class: "meal-desc", "{desc}" }
                }
                {render_slot("Protein", "P", "protein", 0, protein_alts, ctx)}
                {render_slot("Starch", "S", "starch", 1, starch_alts, ctx)}
                {render_slot("Vegetable", "V", "veg1", 2, veg1_alts, ctx)}
                if *show_veg2.read() {
                    {render_slot("Extra Veg", "V₂", "veg2", 3, veg2_alts, ctx)}
                    button { class: "veg-toggle-btn",
                        onclick: move |_| show_veg2.set(false), "Remove Extra Veg"
                    }
                } else {
                    button { class: "veg-toggle-btn",
                        onclick: move |_| show_veg2.set(true), "+ Extra Veg"
                    }
                }
                if !sauces.is_empty() {
                    div { class: "seasoning",
                        span { class: "seasoning__title", "Seasoning · {cur_label}" }
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


