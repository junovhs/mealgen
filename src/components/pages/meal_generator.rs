#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;

use crate::content::{get_proteins, get_starches, get_vegs, CUISINES, CUISINE_LABELS, INGREDIENTS};

static SAUCE_SUGGESTIONS: &[(&str, &[&str])] = &[
    (
        "american",
        &[
            "garlic butter",
            "salt & pepper",
            "herb seasoning",
            "ranch",
            "gravy",
            "mustard",
        ],
    ),
    (
        "latin",
        &[
            "taco seasoning",
            "chimichurri",
            "salsa & lime",
            "cumin & chili",
            "adobo",
            "mojo",
            "hot sauce",
        ],
    ),
    (
        "asian",
        &[
            "teriyaki",
            "soy & ginger",
            "sesame glaze",
            "sweet chili",
            "hoisin",
            "ponzu",
            "sriracha",
        ],
    ),
    (
        "mediterranean",
        &[
            "lemon herb",
            "dill & lemon",
            "olive oil & oregano",
            "balsamic",
            "tzatziki",
            "harissa",
            "pesto",
        ],
    ),
    (
        "bbq",
        &[
            "BBQ rub",
            "smoked paprika",
            "honey mustard glaze",
            "cajun seasoning",
            "dry rub",
            "Carolina vinegar",
            "Alabama white sauce",
        ],
    ),
];

#[derive(Clone, Copy, Debug, PartialEq)]
struct MealSelection {
    protein: Option<&'static crate::content::Ingredient>,
    starch: Option<&'static crate::content::Ingredient>,
    veg1: Option<&'static crate::content::Ingredient>,
    veg2: Option<&'static crate::content::Ingredient>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LockState {
    protein: bool,
    starch: bool,
    veg1: bool,
    veg2: bool,
}

#[component]
pub fn MealGenerator() -> Element {
    let mut cuisine = use_signal(|| CUISINES[0]);
    let mut cuisine_lock = use_signal(|| false);
    let mut selection = use_signal(MealSelection::default);
    let mut locks = use_signal(LockState::default);
    let mut show_veg2 = use_signal(|| false);
    let mut seasoning_open = use_signal(|| false);
    let mut has_generated = use_signal(|| false);

    let pick_random = |list: &[&'static crate::content::Ingredient]| {
        let mut rng = rand::thread_rng();
        list.choose(&mut rng).copied().flatten()
    };

    let in_cuisine = |c: &str, list: &[&'static crate::content::Ingredient]| {
        list.iter()
            .filter(|i| i.cuisines.iter().any(|(cui, _)| *cui == c))
            .copied()
            .collect::<Vec<_>>()
    };

    let pairs_with_protein = |p: Option<&'static crate::content::Ingredient>,
                              c: &str,
                              list: &[&'static crate::content::Ingredient],
                              exclude: &[&str]| {
        if let Some(protein) = p {
            if let Some((_, pairs)) = protein.cuisines.iter().find(|(cui, _)| *cui == c) {
                return list
                    .iter()
                    .filter(|i| !exclude.contains(&i.id) && pairs.contains(&i.id))
                    .copied()
                    .collect::<Vec<_>>();
            }
        }
        in_cuisine(c, list)
    };

    let generate_meal = move |_| {
        let current_cuisine = if *cuisine_lock.read() {
            *cuisine.read()
        } else {
            let mut rng = rand::thread_rng();
            let new_c = CUISINES.choose(&mut rng).unwrap();
            cuisine.set(new_c);
            *new_c
        };

        let proteins = get_proteins();
        let starches = get_starches();
        let vegs = get_vegs();

        // 1. Protein
        let p = if *locks.read().protein {
            selection.read().protein
        } else {
            pick_random(&in_cuisine(current_cuisine, &proteins))
        };

        // 2. Starch
        let s = if *locks.read().starch {
            selection.read().starch
        } else {
            pick_random(&pairs_with_protein(p, current_cuisine, &starches, &[]))
        };

        // 3. Veg 1
        let v1 = if *locks.read().veg1 {
            selection.read().veg1
        } else {
            pick_random(&pairs_with_protein(p, current_cuisine, &vegs, &[]))
        };

        // 4. Veg 2
        let v2 = if *show_veg2.read() {
            if *locks.read().veg2 {
                selection.read().veg2
            } else {
                let exclude = v1.map(|v| v.id).into_iter().collect::<Vec<_>>();
                pick_random(&pairs_with_protein(p, current_cuisine, &vegs, &exclude))
            }
        } else {
            None
        };

        selection.set(MealSelection {
            protein: p,
            starch: s,
            veg1: v1,
            veg2: v2,
        });
        has_generated.set(true);
    };

    rsx! {
        div { class: "section", style: "padding-top: 8rem; max-width: 600px; margin: 0 auto;",
            div { style: "text-align: center; margin-bottom: 2rem;",
                h1 { class: "section-title", style: "margin-bottom: 0.5rem;", "Meal Generator" }
                p { style: "color: var(--text-soft);", "Protein + Starch + Veg" }
            }

            // Cuisine Selector
            div { style: "margin-bottom: 1.5rem;",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;",
                    span { style: "font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-dim);", "Cuisine" }
                    label { style: "display: flex; align-items: center; gap: 0.5rem; cursor: pointer;",
                        span { style: "font-size: 0.75rem; color: var(--text-dim);", if *cuisine_lock.read() { "Locked" } else { "Random" } }
                        button {
                            onclick: move |_| cuisine_lock.toggle(),
                            style: "width: 2.5rem; height: 1.25rem; border-radius: 9999px; position: relative; border: none; cursor: pointer; transition: background 0.2s; {if *cuisine_lock.read() { \"background: var(--accent);\" } else { \"background: var(--bg-elevated);\" }}",
                            div { style: "position: absolute; top: 2px; left: 2px; width: 1rem; height: 1rem; background: white; border-radius: 50%; transition: transform 0.2s; {if *cuisine_lock.read() { \"transform: translateX(1.25rem);\" } else { \"\" }}" }
                        }
                    }
                }
                div { style: "display: flex; flex-wrap: wrap; gap: 0.5rem;",
                    for c in CUISINES.iter() {
                        button {
                            onclick: move |_| cuisine.set(c),
                            style: "padding: 0.5rem 1rem; border-radius: 0.5rem; font-size: 0.875rem; font-weight: 500; transition: all 0.2s; border: 1px solid transparent; {if *cuisine.read() == *c { \"background: var(--accent); color: var(--bg-root);\" } else { \"background: var(--bg-elevated); color: var(--text-soft); border-color: var(--border-subtle);\" }}",
                            {CUISINE_LABELS.iter().find(|(k, _)| k == c).map(|(_, v)| *v).unwrap_or(c)}
                        }
                    }
                }
            }

            // Generate Button
            button {
                onclick: generate_meal,
                class: "btn btn--primary",
                style: "width: 100%; margin-bottom: 1.5rem;",
                if *has_generated.read() { "Generate New Meal" } else { "Generate Meal" }
            }

            if *has_generated.read() {
                {render_slot("Protein", selection.read().protein, locks.read().protein, move |_| locks.write().protein = !locks.read().protein)}
                {render_slot("Starch", selection.read().starch, locks.read().starch, move |_| locks.write().starch = !locks.read().starch)}
                {render_slot("Vegetable", selection.read().veg1, locks.read().veg1, move |_| locks.write().veg1 = !locks.read().veg1)}

                if *show_veg2.read() {
                    {render_slot("Extra Vegetable", selection.read().veg2, locks.read().veg2, move |_| locks.write().veg2 = !locks.read().veg2)}
                    button {
                        onclick: move |_| show_veg2.set(false),
                        style: "width: 100%; padding: 0.75rem; background: transparent; border: 1px dashed var(--border-subtle); color: var(--text-dim); border-radius: 0.5rem; margin-top: 0.5rem; cursor: pointer;",
                        "Remove Extra Vegetable"
                    }
                } else {
                    button {
                        onclick: move |_| show_veg2.set(true),
                        style: "width: 100%; padding: 0.75rem; background: transparent; border: 1px dashed var(--border-subtle); color: var(--text-dim); border-radius: 0.5rem; cursor: pointer;",
                        "+ Add Extra Vegetable"
                    }
                }

                // Seasoning Section
                div { style: "margin-top: 2rem;",
                    button {
                        onclick: move |_| seasoning_open.toggle(),
                        class: "btn btn--secondary",
                        style: "width: 100%; display: flex; justify-content: space-between; align-items: center;",
                        span { "Seasoning & Sauce Ideas" }
                        span { style: "font-size: 0.75rem; opacity: 0.7;", "by cuisine" }
                    }
                    if *seasoning_open.read() {
                        div { style: "margin-top: 0.5rem; background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: 0.5rem; padding: 1rem;",
                            for (c, sauces) in SAUCE_SUGGESTIONS.iter() {
                                div { style: "margin-bottom: 1rem;",
                                    h4 { style: "font-size: 0.75rem; text-transform: uppercase; color: var(--text-dim); margin-bottom: 0.5rem;",
                                        {CUISINE_LABELS.iter().find(|(k, _)| k == c).map(|(_, v)| *v).unwrap_or(c)}
                                    }
                                    div { style: "display: flex; flex-wrap: wrap; gap: 0.5rem;",
                                        for sauce in sauces.iter() {
                                            span { style: "font-size: 0.75rem; padding: 0.25rem 0.5rem; background: var(--bg-elevated); border-radius: 0.25rem; color: var(--text-soft);",
                                                {sauce}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_slot(
    label: &str,
    item: Option<&'static crate::content::Ingredient>,
    locked: bool,
    on_toggle_lock: impl Fn(()) + 'static,
) -> Element {
    rsx! {
        div { style: "background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: 0.75rem; margin-bottom: 0.75rem; overflow: hidden; {if locked { \"border-color: var(--accent);\" } else { \"\" }}",
            div { style: "display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 1rem; border-bottom: 1px solid var(--border-subtle); background: var(--bg-elevated);",
                span { style: "font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-dim);",
                    "{label}"
                }
                button {
                    onclick: move |e| on_toggle_lock(e),
                    style: "padding: 0.25rem; background: transparent; border: none; cursor: pointer; color: {if locked { \"var(--accent)\" } else { \"var(--text-dim)\" }};",
                    if locked {
                        "🔒"
                    } else {
                        "🔓"
                    }
                }
            }
            div { style: "padding: 1rem;",
                if let Some(i) = item {
                    h3 { style: "font-size: 1.1rem; font-weight: 600; margin-bottom: 0.25rem;", "{i.name}" }
                    if let Some(amt) = i.buy_amount {
                        p { style: "font-size: 0.875rem; color: var(--text-dim);", "Buy: {amt}" }
                    }
                } else {
                    p { style: "color: var(--text-dim);", "None selected" }
                }
            }
        }
    }
}
