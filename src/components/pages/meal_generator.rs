#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;

use crate::content::{get_proteins, get_starches, get_vegs, Ingredient, CUISINES, CUISINE_LABELS};

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

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct MealSelection {
    protein: Option<&'static Ingredient>,
    starch: Option<&'static Ingredient>,
    veg1: Option<&'static Ingredient>,
    veg2: Option<&'static Ingredient>,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct LockState {
    protein: bool,
    starch: bool,
    veg1: bool,
    veg2: bool,
}

fn in_cuisine(c: &str, list: &[&'static Ingredient]) -> Vec<&'static Ingredient> {
    list.iter()
        .filter(|i| i.cuisines.iter().any(|(cui, _)| *cui == c))
        .copied()
        .collect()
}

fn pairs_with_protein(
    p: Option<&'static Ingredient>,
    c: &str,
    list: &[&'static Ingredient],
    exclude: &[&str],
) -> Vec<&'static Ingredient> {
    if let Some(protein) = p {
        if let Some((_, pairs)) = protein.cuisines.iter().find(|(cui, _)| *cui == c) {
            return list
                .iter()
                .filter(|i| !exclude.contains(&i.id) && pairs.contains(&i.id))
                .copied()
                .collect();
        }
    }
    in_cuisine(c, list)
}

#[component]
pub fn MealGenerator() -> Element {
    let mut cuisine = use_signal(|| CUISINES[0]);
    let mut cuisine_lock = use_signal(|| false);
    let mut selection = use_signal(MealSelection::default);
    let locks = use_signal(LockState::default);
    let mut show_veg2 = use_signal(|| false);
    let mut seasoning_open = use_signal(|| false);
    let mut has_generated = use_signal(|| false);

    let generate_meal = move |_| {
        let current_cuisine = if *cuisine_lock.read() {
            *cuisine.read()
        } else {
            let mut rng = rand::thread_rng();
            // SAFETY: CUISINES is non-empty static slice
            let new_c = *CUISINES.choose(&mut rng).unwrap_or(&CUISINES[0]);
            cuisine.set(new_c);
            new_c
        };

        let proteins = get_proteins();
        let starches = get_starches();
        let vegs = get_vegs();

        // 1. Protein
        let p = if locks.read().protein {
            selection.read().protein
        } else {
            let mut rng = rand::thread_rng();
            in_cuisine(current_cuisine, &proteins)
                .choose(&mut rng)
                .copied()
        };

        // 2. Starch
        let s = if locks.read().starch {
            selection.read().starch
        } else {
            let mut rng = rand::thread_rng();
            pairs_with_protein(p, current_cuisine, &starches, &[])
                .choose(&mut rng)
                .copied()
        };

        // 3. Veg 1
        let v1 = if locks.read().veg1 {
            selection.read().veg1
        } else {
            let mut rng = rand::thread_rng();
            pairs_with_protein(p, current_cuisine, &vegs, &[])
                .choose(&mut rng)
                .copied()
        };

        // 4. Veg 2
        let v2 = if *show_veg2.read() {
            if locks.read().veg2 {
                selection.read().veg2
            } else {
                let exclude = v1.map(|v| v.id).into_iter().collect::<Vec<_>>();
                let mut rng = rand::thread_rng();
                pairs_with_protein(p, current_cuisine, &vegs, &exclude)
                    .choose(&mut rng)
                    .copied()
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

    let toggle_bg = if *cuisine_lock.read() {
        "background: var(--accent);"
    } else {
        "background: var(--bg-elevated);"
    };

    let toggle_knob = if *cuisine_lock.read() {
        "transform: translateX(1.25rem);"
    } else {
        ""
    };

    rsx! {
        div { class: "section", style: "padding-top: 6rem; max-width: 560px; margin: 0 auto;",

            // ── Header ──
            div { style: "text-align: center; margin-bottom: 2.5rem; animation: fadeSlideUp 0.5s ease both;",
                h1 { class: "section-title", style: "margin-bottom: 0.25rem;", "Meal Generator" }
                p { style: "color: var(--text-soft); font-size: 0.95rem;", "Protein + Starch + Veg" }
            }

            // ── Cuisine Selector ──
            div { style: "margin-bottom: 1.5rem; animation: fadeSlideUp 0.5s ease 0.1s both;",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;",
                    span { style: "font-family: var(--font-mono); font-size: 0.7rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em; color: var(--text-muted);", "Cuisine" }
                    label { style: "display: flex; align-items: center; gap: 0.5rem; cursor: pointer;",
                        span { style: "font-size: 0.75rem; color: var(--text-dim); font-weight: 500;",
                            if *cuisine_lock.read() { "Locked" } else { "Random" }
                        }
                        button {
                            onclick: move |_| cuisine_lock.toggle(),
                            style: "width: 2.5rem; height: 1.25rem; border-radius: 9999px; position: relative; border: none; cursor: pointer; transition: background 0.3s ease; {toggle_bg}",
                            div { style: "position: absolute; top: 2px; left: 2px; width: 1rem; height: 1rem; background: white; border-radius: 50%; transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1); box-shadow: 0 1px 3px rgba(0,0,0,0.15); {toggle_knob}" }
                        }
                    }
                }
                div { style: "display: flex; flex-wrap: wrap; gap: 0.5rem;",
                    for c in CUISINES.iter() {
                        {
                            let is_active = *cuisine.read() == *c;
                            let btn_style = if is_active {
                                "background: var(--accent); color: #fff; border-color: var(--accent); box-shadow: 0 2px 8px rgba(255, 107, 53, 0.25);"
                            } else {
                                "background: var(--bg-surface); color: var(--text-secondary); border-color: var(--border-subtle);"
                            };
                            let label = CUISINE_LABELS.iter().find(|(k, _)| k == c).map(|(_, v)| *v).unwrap_or(c);
                            rsx! {
                                button {
                                    onclick: move |_| cuisine.set(c),
                                    style: "padding: 0.5rem 1rem; border-radius: var(--radius-full); font-size: 0.85rem; font-weight: 500; transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1); border: 1.5px solid transparent; cursor: pointer; {btn_style}",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // ── Generate Button ──
            button {
                onclick: generate_meal,
                class: "btn btn--primary",
                style: "width: 100%; margin-bottom: 2rem; font-size: 0.85rem; padding: 1rem; animation: fadeSlideUp 0.5s ease 0.2s both;",
                if *has_generated.read() { "Generate New Meal" } else { "Generate Meal" }
            }

            // ── Results ──
            if *has_generated.read() {
                {render_slot("Protein", selection.read().protein, locks.read().protein, locks.clone(), "protein", 0)}
                {render_slot("Starch", selection.read().starch, locks.read().starch, locks.clone(), "starch", 1)}
                {render_slot("Vegetable", selection.read().veg1, locks.read().veg1, locks.clone(), "veg1", 2)}

                if *show_veg2.read() {
                    {render_slot("Extra Vegetable", selection.read().veg2, locks.read().veg2, locks.clone(), "veg2", 3)}
                    button {
                        onclick: move |_| show_veg2.set(false),
                        style: "width: 100%; padding: 0.75rem; background: transparent; border: 2px dashed var(--border-subtle); color: var(--text-dim); border-radius: var(--radius-md); margin-top: 0.5rem; cursor: pointer; font-size: 0.85rem; font-weight: 500; transition: all 0.2s ease;",
                        "Remove Extra Vegetable"
                    }
                } else {
                    button {
                        onclick: move |_| show_veg2.set(true),
                        style: "width: 100%; padding: 0.75rem; background: transparent; border: 2px dashed var(--border-subtle); color: var(--text-dim); border-radius: var(--radius-md); cursor: pointer; font-size: 0.85rem; font-weight: 500; transition: all 0.2s ease;",
                        "+ Add Extra Vegetable"
                    }
                }

                // ── Seasoning Section ──
                div { style: "margin-top: 2rem; animation: fadeSlideUp 0.4s ease both;",
                    button {
                        onclick: move |_| seasoning_open.toggle(),
                        class: "btn btn--secondary",
                        style: "width: 100%; display: flex; justify-content: space-between; align-items: center;",
                        span { "Seasoning & Sauce Ideas" }
                        span { style: "font-size: 0.7rem; opacity: 0.6;", "by cuisine" }
                    }
                    if *seasoning_open.read() {
                        div { style: "margin-top: 0.75rem; background: var(--bg-surface); border: 1.5px solid var(--border-subtle); border-radius: var(--radius-md); padding: 1.25rem; animation: popIn 0.3s ease both;",
                            for (c, sauces) in SAUCE_SUGGESTIONS.iter() {
                                {
                                    let label = CUISINE_LABELS.iter().find(|(k, _)| k == c).map(|(_, v)| *v).unwrap_or(c);
                                    rsx! {
                                        div { style: "margin-bottom: 1.25rem;",
                                            h4 { style: "font-family: var(--font-mono); font-size: 0.7rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-muted); margin-bottom: 0.5rem;",
                                                "{label}"
                                            }
                                            div { style: "display: flex; flex-wrap: wrap; gap: 0.4rem;",
                                                for sauce in sauces.iter() {
                                                    span { style: "font-size: 0.8rem; padding: 0.3rem 0.65rem; background: var(--bg-raised); border-radius: var(--radius-full); color: var(--text-secondary); font-weight: 450;",
                                                        "{sauce}"
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
    }
}

fn render_slot(
    label: &str,
    item: Option<&'static Ingredient>,
    locked: bool,
    mut locks: Signal<LockState>,
    field: &'static str,
    index: u32,
) -> Element {
    let border_style = if locked {
        "border-color: var(--accent); box-shadow: 0 0 0 1px var(--accent);"
    } else {
        ""
    };

    let icon_color = if locked {
        "var(--accent)"
    } else {
        "var(--text-dim)"
    };

    let delay = format!("animation: popIn 0.35s ease {}ms both;", index * 80);

    rsx! {
        div { style: "background: var(--bg-surface); border: 1.5px solid var(--border-subtle); border-radius: var(--radius-md); margin-bottom: 0.75rem; overflow: hidden; transition: border-color 0.3s ease, box-shadow 0.3s ease; {border_style} {delay}",
            div { style: "display: flex; justify-content: space-between; align-items: center; padding: 0.6rem 1rem; border-bottom: 1px solid var(--border-subtle); background: var(--bg-raised);",
                span { style: "font-family: var(--font-mono); font-size: 0.7rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-muted);",
                    "{label}"
                }
                button {
                    onclick: move |_| {
                        let mut w = locks.write();
                        match field {
                            "protein" => w.protein = !w.protein,
                            "starch" => w.starch = !w.starch,
                            "veg1" => w.veg1 = !w.veg1,
                            "veg2" => w.veg2 = !w.veg2,
                            _ => {}
                        }
                    },
                    style: "padding: 0.25rem 0.4rem; background: transparent; border: none; cursor: pointer; color: {icon_color}; font-size: 1rem; transition: transform 0.2s ease;",
                    if locked {
                        "🔒"
                    } else {
                        "🔓"
                    }
                }
            }
            div { style: "padding: 1rem 1.25rem;",
                if let Some(i) = item {
                    h3 { style: "font-family: var(--font-display); font-size: 1.15rem; font-weight: 700; margin-bottom: 0.2rem; color: var(--text-primary);", "{i.name}" }
                    if let Some(amt) = i.buy_amount {
                        p { style: "font-size: 0.85rem; color: var(--text-dim); font-weight: 450;", "Buy: {amt}" }
                    }
                } else {
                    p { style: "color: var(--text-dim);", "None selected" }
                }
            }
        }
    }
}
