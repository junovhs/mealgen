#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;

use crate::content::{
    get_proteins, get_starches, get_vegs, Ingredient, CUISINE_LABELS,
};

pub static SAUCE_SUGGESTIONS: &[(&str, &[&str])] = &[
    ("american", &["garlic butter", "salt & pepper", "herb seasoning", "ranch", "gravy", "mustard"]),
    ("latin", &["taco seasoning", "chimichurri", "salsa & lime", "cumin & chili", "adobo", "mojo", "hot sauce"]),
    ("asian", &["teriyaki", "soy & ginger", "sesame glaze", "sweet chili", "hoisin", "ponzu", "sriracha"]),
    ("mediterranean", &["lemon herb", "dill & lemon", "olive oil & oregano", "balsamic", "tzatziki", "harissa", "pesto"]),
    ("bbq", &["BBQ rub", "smoked paprika", "honey mustard glaze", "cajun seasoning", "dry rub", "Carolina vinegar", "Alabama white sauce"]),
];

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MealSelection {
    pub protein: Option<&'static Ingredient>,
    pub starch: Option<&'static Ingredient>,
    pub veg1: Option<&'static Ingredient>,
    pub veg2: Option<&'static Ingredient>,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LockState {
    pub protein: bool,
    pub starch: bool,
    pub veg1: bool,
    pub veg2: bool,
}

#[derive(Clone, Copy)]
pub struct SlotCtx {
    pub locks: Signal<LockState>,
    pub selection: Signal<MealSelection>,
    pub editing: Signal<Option<&'static str>>,
    pub cuisine: Signal<&'static str>,
    pub show_veg2: Signal<bool>,
}

pub fn in_cuisine(c: &str, list: &[&'static Ingredient]) -> Vec<&'static Ingredient> {
    list.iter()
        .filter(|i| i.cuisines.iter().any(|(cui, _)| *cui == c))
        .copied()
        .collect()
}

/// Bidirectional pairing: returns items where EITHER the protein
/// lists the item as a pair OR the item lists the protein as a pair,
/// both within the given cuisine. This ensures broccoli shows up for
/// steak even if steak's list doesn't mention broccoli, as long as
/// broccoli's list mentions steak.
pub fn pairs_with_protein(
    p: Option<&'static Ingredient>,
    c: &str,
    list: &[&'static Ingredient],
    exclude: &[&str],
) -> Vec<&'static Ingredient> {
    let Some(protein) = p else {
        return in_cuisine(c, list);
    };

    // Forward: ids the protein says it pairs with
    let forward: &[&str] = protein
        .cuisines
        .iter()
        .find(|(cui, _)| *cui == c)
        .map(|(_, pairs)| *pairs)
        .unwrap_or(&[]);

    list.iter()
        .filter(|i| {
            if exclude.contains(&i.id) {
                return false;
            }
            // Forward check: protein lists this item
            if forward.contains(&i.id) {
                return true;
            }
            // Reverse check: item lists this protein
            i.cuisines
                .iter()
                .find(|(cui, _)| *cui == c)
                .map_or(false, |(_, pairs)| pairs.contains(&protein.id))
        })
        .copied()
        .collect()
}

pub fn get_item(sel: &MealSelection, field: &str) -> Option<&'static Ingredient> {
    match field {
        "protein" => sel.protein,
        "starch" => sel.starch,
        "veg1" => sel.veg1,
        "veg2" => sel.veg2,
        _ => None,
    }
}

pub fn is_locked(locks: &LockState, field: &str) -> bool {
    match field {
        "protein" => locks.protein,
        "starch" => locks.starch,
        "veg1" => locks.veg1,
        "veg2" => locks.veg2,
        _ => false,
    }
}

pub fn cascade_from_protein(
    sel: &mut MealSelection,
    lock: &LockState,
    cuisine: &str,
    has_veg2: bool,
) {
    let starches = get_starches();
    let vegs = get_vegs();
    let mut rng = rand::thread_rng();

    if !lock.starch {
        sel.starch = pairs_with_protein(sel.protein, cuisine, &starches, &[])
            .choose(&mut rng).copied();
    }
    if !lock.veg1 {
        sel.veg1 = pairs_with_protein(sel.protein, cuisine, &vegs, &[])
            .choose(&mut rng).copied();
    }
    if !lock.veg2 && has_veg2 {
        let exc: Vec<&str> = sel.veg1.map(|v| v.id).into_iter().collect();
        sel.veg2 = pairs_with_protein(sel.protein, cuisine, &vegs, &exc)
            .choose(&mut rng).copied();
    }
}

pub fn cuisine_label(c: &str) -> &'static str {
    CUISINE_LABELS
        .iter()
        .find(|(k, _)| *k == c)
        .map(|(_, v)| *v)
        .unwrap_or("Unknown")
}

pub fn describe_meal(sel: &MealSelection) -> Option<String> {
    let p = sel.protein?;
    let s = sel.starch?;
    let v = sel.veg1?;
    match sel.veg2 {
        Some(v2) => Some(format!(
            "This evening, we present {} served alongside {}, \
             with a side of {} \u{2014} complemented by {}.",
            p.name, s.name, v.name, v2.name
        )),
        None => Some(format!(
            "This evening, we present {} served alongside {}, \
             accompanied by {}.",
            p.name, s.name, v.name
        )),
    }
}
