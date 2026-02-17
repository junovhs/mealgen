#![allow(non_snake_case)]

use crate::content::{Ingredient, CUISINES};
use super::meal_types::SlotOption;

pub fn generate_slot_options(
    all_items: &[&'static Ingredient],
    context_protein: Option<&'static Ingredient>,
    current_cuisine: &'static str,
    exclude_ids: &[&str],
) -> Vec<SlotOption> {
    all_items.iter()
        .filter(|i| !exclude_ids.contains(&i.id))
        .map(|&i| {
            let mut is_compatible = false;
            let mut target_cuisine = None;
            let mut clear_protein = false;

            if let Some(p) = context_protein {
                // If paired with protein in current cuisine
                if check_pair(p, i, current_cuisine) {
                    is_compatible = true;
                } else {
                    // Try to find ANY cuisine where they pair
                    if let Some(c) = find_cuisine_pairing(p, i) {
                        target_cuisine = Some(c);
                        // If user picks this, we switch cuisine but keep protein
                    } else {
                        // They don't pair in ANY cuisine.
                        // Check if this item is valid in ANY cuisine at all?
                        if let Some(c) = find_any_valid_cuisine(i) {
                            target_cuisine = Some(c);
                            clear_protein = true; // Must clear protein as it conflicts globally
                        }
                        // If somehow item has no cuisines, it stays incompatible/unselectable effectively (or just grey with no target)
                    }
                }
            } else {
                // No protein selected. Compatible if in current cuisine.
                if is_in_cuisine(i, current_cuisine) {
                    is_compatible = true;
                } else {
                    // Start looking for any valid cuisine
                    if let Some(c) = find_any_valid_cuisine(i) {
                        target_cuisine = Some(c);
                    }
                }
            }

            SlotOption {
                ingredient: i,
                is_compatible,
                target_cuisine,
                clear_protein,
            }
        })
        .collect()
}

// Helpers
fn check_pair(p: &Ingredient, i: &Ingredient, c: &str) -> bool {
    // Forward: p lists i in c?
    let forward = p.cuisines.iter()
        .find(|(kc, _)| *kc == c)
        .is_some_and(|(_, list)| list.contains(&i.id));
    
    if forward { return true; }

    // Reverse: i lists p in c?
    i.cuisines.iter()
        .find(|(kc, _)| *kc == c)
        .is_some_and(|(_, list)| list.contains(&p.id))
}

fn is_in_cuisine(i: &Ingredient, c: &str) -> bool {
    i.cuisines.iter().any(|(kc, _)| *kc == c)
}

fn find_cuisine_pairing(p: &Ingredient, i: &Ingredient) -> Option<&'static str> {
    // Return first cuisine where they pair
    CUISINES.iter().find(|&c_curr| check_pair(p, i, c_curr)).copied()
}

fn find_any_valid_cuisine(i: &Ingredient) -> Option<&'static str> {
    i.cuisines.first().map(|(c, _)| *c)
}
