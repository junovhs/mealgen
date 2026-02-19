# MealGen — Spec

> "Tell Me What's For Dinner"

MealGen is a single-page meal idea generator built in Rust with Dioxus, compiled to WebAssembly. It answers one question every household asks: *what should we have for dinner tonight?* It does this by generating culinarily coherent combinations of protein, starch, and vegetable(s) drawn from a hand-curated pairing database, filtered by cuisine.

---

## 1. Core Concept

The fundamental unit is a **meal**: one protein, one starch, one or two vegetables, all belonging to a shared cuisine and validated against a pairing graph. The app is not a recipe database — it doesn't tell you *how* to cook. It tells you *what* to cook and *what to buy*, then gets out of the way.

### Design Philosophy

MealGen should feel like a slot machine crossed with a sommelier. Every interaction — generating, locking, rerolling, browsing alternatives — should feel tactile, instantaneous, and rewarding. We think about this as a **game loop**:

- **Second-to-second**: Every tap produces visible, satisfying feedback. Buttons compress on press. Cards animate in with staggered timing. The reroll button spins. Locking an ingredient gives it a visible accent border.
- **Minute-to-minute**: The user builds a meal they're excited about. They lock what they like, reroll what they don't, browse alternatives for fine control. The maître d' description at the top narrates their creation in real time, making the result feel curated rather than random.
- **Session-to-session**: The pairing data is good enough that users trust the suggestions. Every generated combo should sound like something you'd actually want to eat.

### Non-Goals

- Recipe instructions, cook times, or preparation steps.
- User accounts, saved meals, or history.
- Nutritional data or calorie tracking.
- Grocery list aggregation across multiple meals.

---

## 2. Tech Stack

| Layer | Technology | Why |
|---|---|---|
| Language | Rust | Performance, safety, compiles to WASM |
| UI Framework | Dioxus 0.7 | React-like component model for Rust, first-class WASM support |
| Rendering | Dioxus web (WASM) | Runs entirely client-side, zero backend |
| Routing | Dioxus Router | `/` → MealGenerator, `/*` → 404 |
| Randomness | `rand` 0.8 + `getrandom` (js feature) | Cryptographic RNG seeded from browser |
| Styling | Static CSS (no preprocessor) | Three files: `base.css`, `components.css`, `slots.css` |
| Fonts | Google Fonts | Syne (display), Inter (body), IBM Plex Mono (labels), Lora (meal description) |
| Build Tool | `dx serve` / `dx build` | Dioxus CLI |

### File Structure

```
mealgen/
├── index.html                          # Shell: font imports, CSS links, #main div
├── Cargo.toml                          # Rust deps: dioxus, rand, getrandom, web-sys
├── Dioxus.toml                         # Build config
├── public/assets/css/
│   ├── base.css                        # Variables, reset, nav, layout, footer, keyframes
│   ├── components.css                  # Buttons, pills, meal description, seasoning, veg toggle
│   └── slots.css                       # Meal slot cards, lock/reroll buttons, ingredient picker
├── doc/
│   └── spec.md                         # This document
└── src/
    ├── main.rs                         # App shell, router
    ├── components/
    │   ├── mod.rs                      # Declares layout + pages
    │   ├── layout/
    │   │   └── mod.rs                  # Nav, Footer
    │   └── pages/
    │       ├── mod.rs                  # Declares sub-modules, NotFound page
    │       ├── meal_generator.rs       # Main MealGenerator component
    │       ├── meal_types.rs           # Types, constants, all helper/logic fns
    │       └── meal_slot.rs            # render_slot, render_option, reroll_field
    └── content/
        ├── mod.rs                      # Re-exports from meal_data
        └── meal_data.rs                # Static ingredient database (~13k tokens)
```

---

## 3. Data Model

### 3.1 Ingredient

Every ingredient is a compile-time `&'static` struct:

```rust
pub struct Ingredient {
    pub id: &'static str,           // e.g. "chicken_breast"
    pub name: &'static str,         // e.g. "Chicken Breast"
    pub category: &'static str,     // "protein" | "starch" | "veg"
    pub buy_amount: Option<&'static str>,  // e.g. "2.5 lbs" (proteins/some starches)
    pub cuisines: &'static [(&'static str, &'static [&'static str])],
}
```

The `cuisines` field is the pairing graph. Each entry is `(cuisine_id, &[partner_ids])` — the list of ingredient IDs this ingredient pairs well with in that cuisine. An ingredient can belong to multiple cuisines with different pairing lists per cuisine.

A macro (`ingredient!`) provides shorthand for static initialization with or without `buy_amount`.

### 3.2 Categories

| Category | Count (approx) | Has `buy_amount` | Examples |
|---|---|---|---|
| `protein` | ~15 | Yes (always) | Chicken Breast (2.5 lbs), Steak (2 lbs), Salmon (2 lbs) |
| `starch` | ~25 | No | Mashed Potatoes, Fried Rice, Tortillas, Pita Bread |
| `veg` | ~40 | No | Broccoli, Greek Salad, Roasted Carrots, Collard Greens |

### 3.3 Cuisines

Five cuisine categories, each with an internal ID and display label:

| ID | Label | Flavor Profile |
|---|---|---|
| `american` | American | Comfort classics — butter, herbs, ranch, gravy |
| `latin` | Latin / Mexican | Bright, spicy — lime, chili, cumin, cilantro |
| `asian` | Asian | Umami-forward — soy, ginger, sesame, sriracha |
| `mediterranean` | Mediterranean | Fresh, herby — olive oil, lemon, oregano, tzatziki |
| `bbq` | BBQ / Comfort | Smoky, sweet — paprika, mustard, dry rubs, vinegar |

### 3.4 Seasoning Suggestions

A static lookup table (`SAUCE_SUGGESTIONS`) maps each cuisine to 6–7 seasoning/sauce ideas displayed as pills below the meal. These are editorial suggestions, not part of the pairing graph.

---

## 4. Pairing Algorithm

This is the heart of the app. The goal is to produce meals where every ingredient "goes with" every other ingredient.

### 4.1 Generation Sequence

Ingredients are chosen in a strict cascade order:

1. **Cuisine** — Picked randomly (or kept if locked). All subsequent choices are filtered to this cuisine.
2. **Protein** — Picked randomly from proteins available in the chosen cuisine.
3. **Starch** — Picked from starches that pair with the chosen protein in this cuisine.
4. **Vegetable 1** — Picked from vegs that pair with the chosen protein in this cuisine.
5. **Vegetable 2** (optional) — Same as veg 1, but excluding veg 1's ID from candidates.

This cascade ensures coherence: the protein is the anchor, and everything else is chosen to complement it.

### 4.2 Bidirectional Pairing

The pairing lookup is **bidirectional**. An ingredient is considered a valid pair if *either* direction declares the relationship:

```
pairs_with_protein(steak, "american", vegs) includes broccoli if:
  - steak's american pairs list contains "broccoli", OR
  - broccoli's american pairs list contains "steak"
```

This was a critical fix. The original hand-curated data has pairings declared asymmetrically — steak's list might not mention broccoli, but broccoli's list mentions steak. Bidirectional lookup roughly doubles the available options for most combos while keeping all suggestions culinarily sound (every pairing was still hand-authored, just checked in both directions).

### 4.3 Cascade on Protein Change

When the protein changes (via generate, reroll, or manual selection), unlocked downstream slots (starch, veg1, veg2) are automatically re-picked to pair with the new protein. Locked slots are preserved. This prevents stale combos like salmon with cornbread after switching from chicken.

### 4.4 Reroll Behavior

Each slot has an independent reroll button:

- **Protein reroll**: Picks a new random protein, then cascades (re-picks unlocked starch/vegs).
- **Starch reroll**: Picks a new starch that pairs with the current protein. No cascade.
- **Veg1 reroll**: Picks a new veg that pairs with the current protein. No cascade.
- **Veg2 reroll**: Same as veg1, but excludes veg1's ID from candidates.

### 4.5 Manual Selection

Clicking an ingredient name opens a picker panel showing all valid alternatives for that slot in the current cuisine. Selecting a protein from the picker triggers the same cascade as reroll. Selecting a starch or veg is a direct replacement with no cascade.

---

## 5. UI Architecture

### 5.1 State

All state lives in Dioxus signals at the `MealGenerator` component level:

| Signal | Type | Purpose |
|---|---|---|
| `cuisine` | `Signal<&'static str>` | Current cuisine ID |
| `cuisine_lock` | `Signal<bool>` | Whether cuisine is locked (vs randomized on generate) |
| `selection` | `Signal<MealSelection>` | Current protein/starch/veg1/veg2 |
| `locks` | `Signal<LockState>` | Per-slot lock booleans |
| `show_veg2` | `Signal<bool>` | Whether the extra vegetable slot is shown |
| `has_generated` | `Signal<bool>` | Whether the user has generated at least once |
| `editing` | `Signal<Option<&'static str>>` | Which slot's picker panel is open (if any) |

These are bundled into a `SlotCtx` struct and passed to `render_slot` for ergonomic access without exceeding 5 function parameters.

### 5.2 Component Breakdown

```
MealGenerator
├── Header ("Tell Me What's For Dinner", subtitle)
├── Cuisine Selector (pill buttons + lock toggle)
├── Generate Button
├── Meal Description (maître d' text, shown after first generate)
├── Slot: Protein      ─┐
├── Slot: Starch        │  Each rendered by render_slot()
├── Slot: Vegetable     │  with lock, reroll, picker
├── Slot: Extra Veg    ─┘
├── Add/Remove Extra Vegetable toggle
└── Seasoning Ideas (inline pills for current cuisine)
```

### 5.3 Slot Anatomy

Each meal slot card has:

```
┌─────────────────────────────────────────┐
│ PROTEIN                        [◇] [↻] │  ← header: label, keep button, reroll
├─────────────────────────────────────────┤
│ Chicken Breast ▾                        │  ← body: name (clickable), arrow
│ Buy: 2.5 lbs                            │  ← buy amount (proteins only)
├─────────────────────────────────────────┤
│ Chicken Breast · Chicken Thighs ·       │  ← picker panel (expandable)
│ Steak · Salmon · Shrimp · ...           │     shows all valid alternatives
└─────────────────────────────────────────┘
```

- **Keep button** (`◇` / `✦`): Toggles lock state. Locked slots survive generate/reroll. Visual: `◇` is hollow/muted when unlocked, `✦` is filled/accented when locked.
- **Reroll button** (`↻`): Re-picks this slot only (with cascade if protein). Spins on press via CSS `rotate(-45deg)`.
- **Name click**: Toggles the picker panel open/closed. Arrow `▾` rotates 180° when open.
- **Picker panel**: Animates open via CSS `max-height` transition. Shows pill buttons for all valid alternatives. Current selection is highlighted with accent color.

### 5.4 Meal Description

After generating, an italic serif paragraph appears between the generate button and the cards:

> *This evening, we present Steak served alongside Potatoes Au Gratin, with a side of Caesar Salad — complemented by Green Beans.*

This uses the Lora font in italic for a menu/maître d' feel. The container has `min-height: 6rem` and uses flexbox centering so that text length changes (2-line vs 4-line) don't cause layout jitter by pushing cards up and down.

### 5.5 Seasoning Section

Displayed inline (always visible, not collapsible) below the meal cards. Shows only the current cuisine's suggestions as small pills. The label reads "Seasoning ideas · American" (or whichever cuisine is active). This replaced an earlier collapsible accordion that hid the suggestions as an afterthought.

---

## 6. Visual Design

### 6.1 Color Palette

Warm, appetizing, bright. The opposite of the original dark SEMMAP template.

| Token | Value | Usage |
|---|---|---|
| `--bg-root` | `#faf8f5` | Page background (warm off-white) |
| `--bg-surface` | `#ffffff` | Card backgrounds |
| `--bg-raised` | `#f5f2ee` | Card headers, inactive pills |
| `--bg-elevated` | `#ede9e3` | Toggle track, deeper surfaces |
| `--accent` | `#e85d26` | Primary action color (warm coral-orange) |
| `--accent-hover` | `#d4521e` | Darker accent for hover states |
| `--text-primary` | `#2d2a26` | Main text (warm near-black) |
| `--text-secondary` | `#5e5954` | Supporting text |
| `--text-dim` | `#b5ada4` | Placeholders, muted UI |

### 6.2 Typography

| Role | Font | Weight | Usage |
|---|---|---|---|
| Display | Syne | 800 | Page title, ingredient names |
| Body | Inter | 400–600 | General text, pills, buttons |
| Mono | IBM Plex Mono | 500–600 | Labels (PROTEIN, CUISINE), button text |
| Serif | Lora | 400 italic | Meal description only |

### 6.3 Animation System

All animations use named CSS custom easing variables for consistency:

| Variable | Curve | Feel |
|---|---|---|
| `--ease-out` | `cubic-bezier(0.0, 0, 0.2, 1)` | Quick start, gentle stop |
| `--ease-in-out` | `cubic-bezier(0.4, 0, 0.2, 1)` | Smooth both ways (Material standard) |
| `--ease-spring` | `cubic-bezier(0.34, 1.56, 0.64, 1)` | Slight overshoot, playful bounce |
| `--ease-smooth` | `cubic-bezier(0.25, 0.1, 0.25, 1)` | Gentle, natural |

#### Keyframe Animations

| Name | Used For | Behavior |
|---|---|---|
| `fadeSlideUp` | Header, cuisine selector, generate button | Fade in + translate up 14px |
| `fadeIn` | Meal description, seasoning section | Simple opacity |
| `slotReveal` | Meal slot cards | Fade in + translate up 10px, staggered per card (70ms delay × index) |
| `popIn` | — (available) | Scale from 0.96 with slight overshoot to 1.008 |

#### Micro-interactions

- **Button press**: `scale(0.98)` with 0.1s duration for snappy tactile feedback.
- **Reroll press**: `scale(0.9) rotate(-45deg)` — the ↻ visually spins.
- **Keep/lock press**: `scale(0.9)` — quick squeeze.
- **Cuisine pill hover**: `translateY(-1px)` — subtle lift.
- **Generate hover**: `translateY(-2px)` + increased box-shadow — floating up.
- **Picker panel expand**: CSS `max-height` transition from 0 to 220px over 0.4s.
- **Picker arrow**: `rotate(180deg)` over 0.3s with spring easing.

---

## 7. Responsive Behavior

The app targets a single-column layout at `max-width: 560px`, centered. This works well on both desktop (feels focused, like a card) and mobile (fills the viewport naturally).

At `≤768px`:
- Nav link spacing reduces.
- Section padding switches from `var(--space-xl)` to `var(--space-lg)` horizontal.
- Cuisine pills wrap naturally via flexbox.

There is no hamburger menu, sidebar, or multi-column layout. The app is intentionally narrow and vertically scrollable.

---

## 8. Edge Cases & Decisions

| Scenario | Behavior |
|---|---|
| No valid pairs for a slot | Shows "None available" in italics. Can happen if a protein has no pairs in the current cuisine for a given category. |
| Veg2 same as veg1 | Prevented: veg2 candidate pool always excludes veg1's ID. |
| All slots locked + generate | Only cuisine randomizes (if unlocked). All ingredient slots keep their current values. The generate button still works but produces the same meal — this is intentional, as the user may want to randomize just the cuisine. |
| Switching cuisine with locks | Locked slots are preserved even if the ingredient doesn't exist in the new cuisine. This can produce cross-cuisine combos. This is considered acceptable — if the user locked it, they want it. |
| Picker shows 0 alternatives | The picker panel is empty but still opens. This is rare and only happens with very restrictive cuisine/protein combos. |
| `buy_amount` missing | Only displayed if `Some(...)`. Starches and vegs generally don't have buy amounts. |

---

## 9. Development Governance

The project uses **SlopChop** for structural governance. Key constraints:

| Metric | Limit |
|---|---|
| File size | < 2,000 tokens per file |
| Cognitive complexity | ≤ 15 per function |
| Nesting depth | ≤ 3 levels |
| Function arguments | ≤ 5 (use structs to bundle) |

The Rust code is linted with strict clippy:
```
cargo clippy --all-targets -- -D warnings -W clippy::pedantic
    -W clippy::unwrap_used -W clippy::expect_used -W clippy::indexing_slicing
```

No `#[allow(...)]` attributes to silence violations — refactor instead. No `unwrap()`/`expect()` outside tests.

### Known Violations

`meal_data.rs` is ~13,000 tokens, well over the 2,000 token limit. This is the static ingredient database and is a structural exception — splitting a single `&[Ingredient]` const across files requires workarounds. Recommended approach if enforced: split into `proteins.rs`, `starches.rs`, `vegs.rs` and concatenate at the module level.

---

## 10. Provenance & Cleanup History

The project originated from a **SEMMAP Labs** website template — a dark-themed developer tools landing page with physics-based jello animations, product cards, workflow diagrams, and stats sections. None of that content is relevant to MealGen.

### Removed

- `src/components/pages/home.rs` — SEMMAP homepage with canvas-based jello physics (`gloo_timers`, `wasm_bindgen`, `web_sys::CanvasRenderingContext2d`). Dead code, not routed.
- `src/components/pages/sections.rs` — Product/Workflow/Stats/CTA sections about SEMMAP, Talos, DirAnalyze. Only imported by `home.rs`.
- `public/assets/css/main.css` — Monolithic dark-theme stylesheet with grid overlay, 400+ lines of unused SEMMAP classes. Replaced by `base.css` + `components.css` + `slots.css`.
- Grid background overlay (`body::before` with CSS grid lines).
- All references to SEMMAP, Talos, DirAnalyze, semmaplabs GitHub, and jello physics.
- Neon green accent color (`#e2ff54`), dark palette (`#050507`), monospace-only typography.

### Kept

- Dioxus framework, router, project structure.
- Font imports (Syne, Inter, IBM Plex Mono — added Lora).
- Core meal generation logic and pairing data.
- `ingredient!` macro system.

---

## 11. Future Considerations

These are ideas discussed but not yet implemented:

- **Shake animation on reroll** — Brief horizontal shake on the card when content swaps, reinforcing that something changed.
- **Text crossfade on description** — Smooth opacity transition when the maître d' text updates, rather than an instant swap.
- **Lock snap animation** — A brief scale pulse or glow when toggling the keep state.
- **Grocery list mode** — Aggregate `buy_amount` values across a week of meals.
- **Share meal** — Copy a formatted text summary to clipboard.
- **Cuisine weighting** — Let users mark preferred cuisines so random selection skews toward their taste.
- **Seasonal ingredients** — Tag vegs as spring/summer/fall/winter and filter by current month.
- **meal_data.rs split** — Break the 13k-token ingredient database into per-category files to satisfy the 2,000-token governance limit.
