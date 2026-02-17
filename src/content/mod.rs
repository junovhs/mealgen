// Content Module — The "CMS" Data Layer

mod meal_data;

pub use meal_data::{get_proteins, get_starches, get_vegs, Ingredient, CUISINES, CUISINE_LABELS};
