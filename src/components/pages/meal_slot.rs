#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;

use crate::content::Ingredient;

use super::meal_types::{
    cascade_from_protein, get_item, in_cuisine, is_locked,
    pairs_with_protein, SlotCtx, SlotOption,
};

use super::icons::{ICON_LOCK, ICON_UNLOCK, ICON_REROLL, ICON_CHEVRON_DOWN, ICON_REMOVE};

pub fn render_slot(
    label: &str,
    short_label: &str,
    field: &'static str,
    index: u32,
    alternatives: Vec<SlotOption>,
    mut ctx: SlotCtx,
) -> Element {
    let mut reroll_count = use_signal(|| 0u32);

    let sel = *ctx.selection.read();
    let item = get_item(&sel, field);
    let locked = is_locked(*ctx.locks.read(), field);
    let is_editing = *ctx.editing.read() == Some(field);

    let card_class = if locked { "meal-slot meal-slot--locked" } else { "meal-slot" };
    let keep_class = if locked { "slot-btn slot-btn--lock slot-btn--locked" } else { "slot-btn slot-btn--lock" };
    let arrow_class = if is_editing { "meal-slot__name-arrow meal-slot__name-arrow--open" } else { "meal-slot__name-arrow" };
    let picker_class = if is_editing { "meal-slot__picker meal-slot__picker--open" } else { "meal-slot__picker" };
    let anim = format!("animation: slotReveal 0.45s var(--ease-out) {}ms both;", index * 70);
    let reroll_cls = if *reroll_count.read() > 0 { "slot-btn slot-btn--rerolling" } else { "slot-btn" };
    let reroll_key = format!("reroll-{field}-{}", *reroll_count.read());

    rsx! {
        div { class: "{card_class}", style: "{anim}",
            div { class: "meal-slot__header",
                span { class: "meal-slot__label meal-slot__label--full", "{label}" }
                span { class: "meal-slot__label meal-slot__label--short", "{short_label}" }
                div { class: "meal-slot__actions",
                    if item.is_some() {
                        button {
                            class: "slot-btn slot-btn--remove",
                            title: "Clear slot",
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                let mut s = *ctx.selection.read();
                                match field {
                                    "protein" => s.protein = None,
                                    "starch" => s.starch = None,
                                    "veg1" => s.veg1 = None,
                                    "veg2" => s.veg2 = None,
                                    _ => {}
                                }
                                ctx.selection.set(s);
                                ctx.editing.set(None);
                            },
                            span { dangerous_inner_html: ICON_REMOVE }
                        }
                    }
                    button {
                        class: "{keep_class}",
                        title: if locked { "Unlock" } else { "Keep this" },
                        onclick: move |_| {
                            let mut w = ctx.locks.write();
                            match field {
                                "protein" => w.protein = !w.protein,
                                "starch" => w.starch = !w.starch,
                                "veg1" => w.veg1 = !w.veg1,
                                "veg2" => w.veg2 = !w.veg2,
                                _ => {}
                            }
                        },
                        span { dangerous_inner_html: if locked { ICON_LOCK } else { ICON_UNLOCK } }
                    }
                    button {
                        key: "{reroll_key}",
                        class: "{reroll_cls}",
                        title: "Re-roll",
                        onclick: move |_| {
                            reroll_count += 1;
                            reroll_field(field, ctx);
                        },
                        span { dangerous_inner_html: ICON_REROLL }
                    }
                }
            }
            div { class: "meal-slot__body",
                if let Some(i) = item {
                    span {
                        class: "meal-slot__name",
                        onclick: move |_| {
                            if *ctx.editing.read() == Some(field) {
                                ctx.editing.set(None);
                            } else {
                                ctx.editing.set(Some(field));
                            }
                        },
                        "{i.name}"
                        span { class: "{arrow_class}", dangerous_inner_html: ICON_CHEVRON_DOWN }
                    }

                } else {
                    p { class: "meal-slot__empty", "—" }
                }
            }
            div { class: "{picker_class}",
                div { class: "meal-slot__picker-grid",
                    for alt in alternatives {
                        {render_option(alt, item, field, ctx)}
                    }
                }
            }
        }
    }
}

fn render_option(
    opt: SlotOption,
    current: Option<&'static Ingredient>,
    field: &'static str,
    mut ctx: SlotCtx,
) -> Element {
    let ingredient = opt.ingredient;
    let is_compat = opt.is_compatible;
    let target_cuisine = opt.target_cuisine;
    let clear_prot = opt.clear_protein;

    let mut cls = "meal-slot__option".to_string();
    if current == Some(ingredient) {
        cls.push_str(" meal-slot__option--active");
    } else if !is_compat {
        cls.push_str(" meal-slot__option--incompatible");
    }

    rsx! {
        button {
            class: "{cls}",
            title: if is_compat {
                String::new()
            } else if let Some(c) = target_cuisine {
                format!("Switch to {c}")
            } else {
                "Incompatible".to_string()
            },
            onclick: move |_| {
                let mut s = *ctx.selection.read();
                let lock = *ctx.locks.read();
                let sv2 = *ctx.show_veg2.read();
                let mut c_val = *ctx.cuisine.read();

                // Handle incompatibility actions
                if let Some(target) = target_cuisine {
                    ctx.cuisine.set(target);
                    c_val = target;
                    if clear_prot {
                        s.protein = None;
                    }
                }

                match field {
                    "protein" => { 
                        s.protein = Some(ingredient); 
                        if !locked_conflicts(lock, field) {
                             cascade_from_protein(&mut s, lock, c_val, sv2); 
                        }
                    }
                    "starch" => { s.starch = Some(ingredient); }
                    "veg1" => { s.veg1 = Some(ingredient); }
                    "veg2" => { s.veg2 = Some(ingredient); }
                    _ => {}
                }
                ctx.selection.set(s);
                ctx.editing.set(None);
            },
            "{ingredient.name}"
        }
    }
}

fn locked_conflicts(_lock: super::meal_types::LockState, _field: &str) -> bool {
    // Ideally check if cascade would overwrite locked items?
    // cascade_from_protein respects locks.
    false
}

fn reroll_field(field: &'static str, mut ctx: SlotCtx) {
    let c = *ctx.cuisine.read();
    let lock = *ctx.locks.read();
    let sv2 = *ctx.show_veg2.read();
    let mut s = *ctx.selection.read();
    let mut rng = rand::thread_rng();

    match field {
        "protein" => {
            let list = crate::content::get_proteins();
            s.protein = in_cuisine(c, &list).choose(&mut rng).copied();
            cascade_from_protein(&mut s, lock, c, sv2);
        }
        "starch" => {
            let list = crate::content::get_starches();
            s.starch = pairs_with_protein(s.protein, c, &list, &[])
                .choose(&mut rng).copied();
        }
        "veg1" => {
            let list = crate::content::get_vegs();
            s.veg1 = pairs_with_protein(s.protein, c, &list, &[])
                .choose(&mut rng).copied();
        }
        "veg2" => {
            let list = crate::content::get_vegs();
            let exc: Vec<&str> = s.veg1.map(|v| v.id).into_iter().collect();
            s.veg2 = pairs_with_protein(s.protein, c, &list, &exc)
                .choose(&mut rng).copied();
        }
        _ => {}
    }
    ctx.selection.set(s);
}
