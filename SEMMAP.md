# project -- Semantic Map

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

## Layer 0 -- Config

`Cargo.toml`
Rust package manifest and dependencies. Centralizes project configuration.

`Dioxus.toml`
Configuration for Dioxus. Centralizes project configuration.

## Layer 1 -- Core

`src/components/layout/mod.rs`
Module providing `Footer`, `Nav`. Supports application functionality.
→ Exports: Footer, Nav

`src/components/mod.rs`
Module definitions for mod. Supports application functionality.

`src/components/pages/mod.rs`
Module providing `NotFound`. Supports application functionality.
→ Exports: NotFound

`src/content/mod.rs`
Orchestrates `meal_data`. Supports application functionality.

`src/main.rs`
Orchestrates `components`, `dioxus`. Provides application entry point.

## Layer 2 -- Domain

`src/components/pages/generator_logic.rs`
Module providing `generate_slot_options`. Supports application functionality.
→ Exports: generate_slot_options

`src/components/pages/icons.rs`
Implements icons functionality. Supports application functionality.

`src/components/pages/meal_generator.rs`
Module providing `MealGenerator`. Supports application functionality.
→ Exports: MealGenerator

`src/components/pages/meal_slot.rs`
Module providing `render_slot`. Supports application functionality.
→ Exports: render_slot

`src/components/pages/meal_types.rs`
Bidirectional pairing: returns items where EITHER the protein lists the item as a pair OR the item lists the protein as a pair, both within the given cuisine. Defines domain data structures.
→ Exports: LockState, MealSelection, SlotCtx, SlotOption, cascade_from_protein, cuisine_label, describe_meal, get_item, in_cuisine, is_locked, pairs_with_protein

`src/content/meal_data.rs`
Module providing `Ingredient`, `get_proteins`, `get_starches`. Supports application functionality.
→ Exports: Ingredient, get_proteins, get_starches, get_vegs

