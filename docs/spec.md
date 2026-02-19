# MealGen — vNext Spec: "Outstanding Meals" + "Chef Notes"

> Elevate every Generate into a plate that feels *named*, *intentional*, and *cookable* — without using AI APIs.

**Revision history:**
- v1.0 — Initial spec (ChatGPT Pro)
- v1.1 — Addendum A/B: Chef Description MVP pull-forward, editorial pipeline (ChatGPT Pro)
- v2.0 — Full synthesis: protein expansion, protein gap analysis, revised phase order, side-to-side coherence critique
- v2.1 — Spec audit fixes: phase order contradiction resolved, pairwise gate two-tier model, tag definitions corrected, archetype selection order clarified, coverage assertions added, `small_rng` dependency noted, Phase 0/1 exclusion logic deduplication, manual selection behavior under archetypes specified

---

## 1. Goals

### 1.1 Primary Goal
Every press of **Generate** yields a combination that:
- sounds like a *real plate you'd order at a restaurant*,
- maps cleanly to an established dish archetype (steakhouse, tacos, stir-fry bowl, etc.),
- includes enough "how to flavor it" guidance that it feels actionable without being a full recipe.

### 1.2 Secondary Goals
- Keep the "slot machine" feel: instant, tactile, fun.
- Maintain offline-first: no backend required.
- Preserve existing locks / rerolls / manual picker UX.
- Make the chef description the emotional centerpiece of each generation.

---

## 2. Non-Goals (for this iteration)
- Full step-by-step recipes with timers and temperatures in the UI.
- Saved meals, history, user accounts.
- Nutrition or macros.
- Paid APIs (Spoonacular, Edamam, etc.) in the MVP.
- AI-generated text (all description is programmatic and table-driven).

---

## 3. Protein Gap Analysis

> **Sequencing note:** Protein expansion (Phase 2) can proceed in parallel with Phase 1 (tags) and Phase 1.5 (Chef Description). You do not need to wait for new proteins before starting the quality engine work. However, `IngredientDescLite` entries for new proteins must exist before Chef Notes ships — treat this as a hard dependency for Phase 6, not Phase 2.

Several cuisines lack their most iconic proteins. The generator cannot produce "outstanding" results when the headlining ingredients are missing.

### 3.1 BBQ / Comfort — Critical Gaps

| Missing Protein | Why It Matters |
|---|---|
| Baby Back Ribs / Spare Ribs | *The* definitive BBQ hero. Arguably the first thing most people think of. |
| Beef Brisket | Central to Texas BBQ, Jewish deli, KC BBQ. Brisket + cornbread is canonical. |
| Pulled Pork | Low-and-slow shoulder; natural partner for coleslaw and cornbread. |
| Bratwurst / Hot Links / Smoked Sausage | Cookout staple. Different profile from Italian sausage. |
| Smoked Turkey Breast | Holiday comfort and BBQ crossover. |

### 3.2 Latin / Mexican — Gaps

| Missing Protein | Why It Matters |
|---|---|
| Carnitas | Slow-braised pork; one of the most common taco proteins. Distinct from pork chops/tenderloin. |
| Carne Asada | Specific marinated grilled beef; the existing steak entry doesn't carry this flavor signal. |

### 3.3 Mediterranean — Gaps

| Missing Protein | Why It Matters |
|---|---|
| Lamb Chops / Lamb Shoulder | Lamb is *the* Mediterranean protein hero. Its absence makes Mediterranean feel hollow. |

### 3.4 Asian — Gaps

| Missing Protein | Why It Matters |
|---|---|
| Pork Belly | Used in ramen, bao, Korean BBQ. High demand item. |
| Tofu | Vegetarian anchor; significant portion of users. |

### 3.5 American — Minor Gaps

| Missing Protein | Why It Matters |
|---|---|
| Meatloaf | Classic American comfort archetype anchor. |

### 3.6 Implementation Note

New proteins follow the same `ingredient!` macro pattern. Each needs:
- A `buy_amount` string.
- `cuisines` pairings — start with the 1-2 cuisines where the protein is archetypal.
- An `IngredientDescLite` entry before Chef Notes ships (Phase 6 dependency).

---

## 4. Known Data Bugs to Fix (Phase 0)

> **Important:** All meal-validity logic (exclusion groups, redundancy rules) is implemented in Phase 1, not Phase 0. Phase 0 is strictly renames, CSS fixes, and removal of clearly wrong cuisine assignments. Do not implement any rule engine logic in Phase 0.

| Bug | Description | Fix |
|---|---|---|
| CSS syntax error in `slots.css` | Stray declaration block under `.meal-slot__option--incompatible` | Fix immediately |
| `asian_cabbage_slaw` name | Name creates cognitive dissonance in BBQ and Latin contexts where the slaw is actually correct | Rename to "Cabbage Slaw" in `meal_data.rs` |
| `ground_turkey` labeled "(taco)" | Name embeds a cooking style; archetype/seasoning handles framing | Rename to "Ground Turkey" |
| `jasmine_rice` in BBQ | Results in "Pork Chops + Jasmine Rice" — breaks the BBQ flavor signal | Remove from BBQ cuisine pairings entirely |
| `ground_turkey` in American pairings | Ground turkey is not a natural American dinner protein outside of taco context; creates weird combos | Remove from American cuisine |

---

## 5. New Concepts

### 5.1 Archetype (Dish Template)

An **archetype** is a named plate pattern that constrains what combinations are considered coherent.

**Asian**
- `fried_rice_bowl` — fried rice + stir-fry veg + protein
- `noodle_stir_fry` — noodles + sprouts/crisp veg
- `rice_and_greens` — jasmine rice + bok choy / broccoli + protein
- `south_asian_plate` — naan/flatbread + marinated protein + warm veg

**Latin / Mexican**
- `tacos` — tortillas + fajita veg + fresh side (avocado, cucumber tomato)
- `fajita_plate` — tortillas + bell peppers + protein
- `beans_and_rice_plate` — black beans & rice + 1 warm + 1 fresh side
- `carne_asada_plate` — mexican rice + grilled veg + protein

**American**
- `steakhouse` — potato starch + green veg + optional salad
- `sheet_pan_roast` — roasted potato/sweet potato + roasted veg
- `pasta_night` — pasta + caesar/mixed salad + light veg
- `breakfast_for_dinner` — egg/bacon + hash browns + simple veg
- `comfort_bowl` — egg noodles + hearty veg

**BBQ / Comfort**
- `cookout_plate` — cornbread/biscuits + slaw + legume side
- `southern_plate` — cornbread + greens + beans
- `bbq_smoke_plate` — mashed/potato + coleslaw or mac + one hearty side
- `seafood_cookout` — roasted potatoes + corn + coleslaw

**Mediterranean**
- `grain_bowl` — couscous/quinoa + roasted veg + protein
- `mezze_plate` — pita + greek salad + warm veg
- `pasta_mediterranean` — pasta + roasted tomatoes + zucchini/eggplant
- `sheet_pan_mediterranean` — roasted potatoes + roasted veg + protein

### 5.2 Ingredient Tags

Defined in `src/content/meta.rs`. Tags power hard rules and scoring. They do not touch the existing `Ingredient` struct.

**Type/Temperature Tags**
- `cold_side` — salads, slaws, pickled items
- `hot_veg` — sautéed, roasted, braised vegetables
- `leafy_green` — spinach, kale, collard greens, bok choy
- `bitter_green` — kale, brussels sprouts, collard greens (subset of leafy_green)
- `legume_side` — baked beans, black-eyed peas, edamame

**Starch Role Tags** *(replaces the ambiguous "noodle" tag)*
- `starch_potato` — mashed, roasted, baked, twice-baked, au gratin, hash browns, sweet potato, fries
- `starch_rice` — jasmine rice, cilantro lime rice, rice pilaf, black beans & rice, yellow rice, mexican rice
- `starch_fried_rice` — fried rice only (one-pan mixed dish; differs from plain rice)
- `starch_noodle` — pasta, egg noodles only
- `starch_bread` — cornbread, biscuits, bread rolls, garlic bread, naan, pita, tortillas

**Flavor Redundancy Tags**
- `tomato_forward` — roasted_tomatoes, cucumber_tomato *(note: ratatouille gets `stewed_veg` + `tomato_forward`)*
- `corn_forward` — corn, elote_corn, succotash
- `cabbage_forward` — coleslaw, cabbage_slaw, sauteed_cabbage

**Cuisine-Signal Tags**
- `east_asian_signal` — bok_choy, bean_sprouts, edamame, pickled_vegetables
- `south_asian_signal` — naan *(do not pair with east_asian_signal veg)*
- `southern_signal` — collard_greens, cornbread, biscuits, black_eyed_peas
- `italian_american_signal` — pasta, garlic_bread

**Semantic Exclusion Groups** *(any two items from the same group cannot co-exist on a plate)*
- `group_corn` — corn, elote_corn
- `group_salad` — mixed_salad, garden_salad
- `group_tomato_side` — roasted_tomatoes, cucumber_tomato *(max 1 per plate; ratatouille excluded from this group since it's a full dish)*
- `group_bitter_green` — kale, brussels_sprouts, collard_greens *(max 1 per plate)*
- `group_cabbage` — coleslaw, cabbage_slaw, sauteed_cabbage *(max 1 per plate)*

### 5.3 Chef Notes (Flavor Plan)

Short, programmatic "how to make this plate taste like a dish" guidance — driven entirely by static tables in Rust. No AI. Instant. Deterministic per seed.

Output per generated plate:
- 1 headline: e.g. "Steakhouse Night" or "Baja Tacos"
- 2–4 short action lines, one per ingredient with a descriptor entry

Example:
```
Steakhouse Night
· Ribeye — sear hard with thyme + roasted garlic; rest 5 min; red wine pan jus
· Pomme Purée — silky with butter and chives
· Asparagus — roast hot at 425°F; finish with lemon zest and olive oil
· Caesar — classic, extra black pepper
```

### 5.4 Dish Name

A short editorial name generated from the archetype + protein + primary starch/veg:
- "Hibachi-Style Steak Fried Rice"
- "Baja Fish Tacos"
- "Southern Smoked Ribs Plate"
- "Weeknight Sheet-Pan Salmon"
- "Steakhouse Ribeye + Pomme Purée"

Displayed above the maître d' description. Acts as the vetting signal — if the meal feels nameable, it feels curated.

---

## 6. Data Model Additions

> Keep existing `Ingredient` static list for pairings. Add parallel metadata tables keyed by `ingredient.id`.

### 6.1 IngredientMeta

```rust
// src/content/meta.rs
pub struct IngredientMeta {
    pub id: &'static str,
    pub tags: &'static [&'static str],
    pub exclusion_groups: &'static [&'static str],
    pub aliases: &'static [&'static str], // for recipe search / dish naming
}
```

**Coverage requirement:** `IngredientMeta` must have an entry for every ID in `INGREDIENTS`. Enforce via debug assertion (see Section 6.4).

### 6.2 IngredientDescLite

Used for Chef Notes + upgraded maître d' text. Proteins are required; starch/veg use adjective pools for MVP.

```rust
// src/content/desc_lite.rs
pub struct IngredientDescLite {
    pub id: &'static str,
    // (cuisine_id, fancy_name) — e.g. ("american", "ribeye"), ("bbq", "smoked brisket")
    pub fancy_names: &'static [(&'static str, &'static str)],
    // (cuisine_id, &[method variants]) — pick 1 using meal_style_seed
    pub methods: &'static [(&'static str, &'static [&'static str])],
    // (cuisine_id, &[seasoning combos]) — pick 1 using meal_style_seed
    pub seasonings: &'static [(&'static str, &'static [&'static str])],
    // (cuisine_id, &[finish variants]) — pick 1 using meal_style_seed
    pub finishes: &'static [(&'static str, &'static [&'static str])],
}
```

Generic adjective pools for starches and veg (no per-item tables required for MVP):
```rust
pub static STARCH_ADJECTIVES: &[(&str, &[&str])] = &[
    ("starch_potato", &["silky", "buttery", "golden-crusted", "creamy"]),
    ("starch_rice",   &["fluffy", "fragrant", "steamed", "lightly seasoned"]),
    ("starch_noodle", &["al dente", "tossed", "sauced"]),
    ("starch_bread",  &["warm", "golden", "fresh-baked"]),
    ("starch_fried_rice", &["wok-tossed", "smoky", "savory"]),
];

pub static VEG_ADJECTIVES_HOT: &[&str] = &["roasted", "sautéed", "charred", "caramelized"];
pub static VEG_ADJECTIVES_COLD: &[&str] = &["crisp", "fresh", "bright", "classic"];
```

**Coverage requirement:** `IngredientDescLite` must cover 100% of protein IDs. Enforce via debug assertion (see Section 6.4).

### 6.3 Archetype Definition

```rust
// src/content/archetypes.rs
pub struct Archetype {
    pub id: &'static str,
    pub cuisine: &'static str,
    pub label: &'static str,       // "Steakhouse", "Tacos" — shown in UI badge
    pub constraints: ArchetypeConstraints,
    pub dish_name_templates: &'static [&'static str],
    pub chef_voice: &'static str,  // "elegant" | "casual" | "bold"
}

pub struct ArchetypeConstraints {
    pub starch_must_have_any: &'static [&'static str],  // tag names
    pub starch_avoid_any: &'static [&'static str],
    pub veg_prefer_any: &'static [&'static str],
    pub veg_avoid_any: &'static [&'static str],
    pub max_cold_sides: u8,
    pub forbidden_signal_combos: &'static [(&'static str, &'static str)], // tag pairs that cannot coexist
}
```

### 6.4 Coverage Assertions

Add to `src/content/meta.rs` and `src/content/desc_lite.rs`:

```rust
#[cfg(debug_assertions)]
pub fn assert_meta_coverage() {
    use crate::content::meal_data::INGREDIENTS;
    let meta_ids: std::collections::HashSet<&str> =
        INGREDIENT_META.iter().map(|m| m.id).collect();
    for ing in INGREDIENTS.iter() {
        debug_assert!(
            meta_ids.contains(ing.id),
            "IngredientMeta missing entry for: {}", ing.id
        );
    }
}

#[cfg(debug_assertions)]
pub fn assert_desc_coverage() {
    use crate::content::meal_data::INGREDIENTS;
    let desc_ids: std::collections::HashSet<&str> =
        INGREDIENT_DESC_LITE.iter().map(|d| d.id).collect();
    for ing in INGREDIENTS.iter().filter(|i| i.category == "protein") {
        debug_assert!(
            desc_ids.contains(ing.id),
            "IngredientDescLite missing protein entry for: {}", ing.id
        );
    }
}
```

Call both from `main()` or app init in debug builds.

### 6.5 MealPlan

Replaces bare `MealSelection` as the output of the quality engine.

```rust
pub struct MealPlan {
    pub cuisine: &'static str,
    pub archetype_id: &'static str,
    pub selection: MealSelection,
    pub meal_style_seed: u64,

    // Presentation
    pub dish_name: Option<String>,
    pub chef_desc: String,
    pub chef_notes: Vec<String>,     // 2–4 bullet lines
    pub recipe_queries: Vec<String>,
}
```

### 6.6 Dependency Update

Add `small_rng` feature to `Cargo.toml` for deterministic seeded descriptions:

```toml
rand = { version = "0.8", features = ["small_rng"] }
```

Use `rand::rngs::SmallRng::seed_from_u64(seed)` in `describe_meal_chef_lite`. This avoids `StdRng` overhead in WASM.

---

## 7. Generation v2: Quality Engine

### 7.1 Two-Tier Constraint Model

> **Critical design decision:** The existing pairing graph was authored for protein↔side relationships. Starch↔veg edges exist in the data but are *not guaranteed to be complete* across all combinations. Enforcing hard rejection on missing edges would eliminate too many valid candidates and degrade output quality.
>
> Therefore: use a **two-tier model**.

**Tier 1 — Hard Reject** (missing = rejected):
- Exclusion group violations (two items from the same group)
- Explicit cuisine-signal conflicts (e.g., `south_asian_signal` + `east_asian_signal` on same plate)
- Redundancy tag count violations (> 1 `bitter_green`, > 1 `tomato_forward`, etc.)
- Archetype starch/veg hard constraints (e.g., `steakhouse` hard-rejects `starch_fried_rice`)
- `max_cold_sides` exceeded per archetype

**Tier 2 — Scored** (missing = neutral, present = bonus):
- Starch↔veg1 pairing edge exists: +3
- Starch↔veg2 pairing edge exists: +3
- Veg1↔veg2 pairing edge exists: +2
- Missing edges score 0, not negative

> **Future work:** If you want a true clique-gate, add a "Graph completion sprint" (Phase 2.5) to editorially add starch↔veg edges across all cuisines. Until then, two-tier is the correct model.

### 7.2 Full Scoring Table

| Rule | Points |
|---|---|
| Each bidirectional pairing edge between any two selected ingredients | +3 |
| All four slots pairwise compatible (protein + starch + veg1 + veg2 all connected) | +5 bonus |
| Starch↔veg1 edge exists | +3 |
| Starch↔veg2 edge exists | +3 |
| Veg1↔veg2 edge exists | +2 |
| Starch matches archetype preferred tags | +4 |
| Each veg matches archetype preferred tags | +2 |
| Protein has `IngredientDescLite` entry (ensures chef notes are rich) | +1 |
| Two `cold_side` items (soft penalty even if under archetype limit) | -3 |
| Cuisine-signal tag conflicts with archetype's expected signals | -3 |

> Store all weights as top-level `const i32` values in `quality_engine.rs`. Never embed magic numbers in scoring logic. This makes tuning fast — change a constant, observe output, repeat.

### 7.3 Archetype Selection Order

Selection always proceeds in this exact order:

```
1. Choose cuisine (locked or random from CUISINES)
2. Choose protein (locked or random from proteins available in cuisine)
3. Choose archetype — conditioned on (cuisine, protein, show_veg2)
   - Filter archetypes to those compatible with the chosen protein
   - Weight by archetype frequency (equal weights initially; tune later)
4. Generate N=40 candidates for (starch, veg1, veg2)
5. Apply Tier 1 hard constraints — reject failures
6. Score remaining candidates (Tier 2)
7. Return highest-scoring candidate
8. Fallback: if 0 pass Tier 1, relax archetype starch/veg preferences (not signal rules);
   if still 0, use existing cascade and mark meal "Experimental" (optional badge)
```

**Why protein-before-archetype:** Archetype must be conditioned on protein because certain proteins are archetype-defining (ribs → `bbq_smoke_plate`; tortilla-context protein → `tacos`). Picking archetype first and then filtering proteins risks empty candidate pools.

### 7.4 Reroll Behavior (v2)

- **Protein reroll:** re-run full best-of-N from step 2 (re-pick protein and archetype).
- **Starch reroll:** optimize starch only, given fixed protein + veg(s) + archetype constraints.
- **Veg reroll:** optimize that veg slot only, given fixed others + archetype constraints.
- All rerolls must produce a result: relax soft constraints before giving up.

### 7.5 Manual Selection Under Archetypes

> **Decision required — pick exactly one. This spec chooses Option B.**

| Option | Behavior |
|---|---|
| A | Manual selection preserves archetype; other unlocked slots re-optimize to satisfy it |
| **B** *(chosen)* | **Manual selection triggers archetype re-pick conditioned on new full selection; other unlocked slots re-optimize for the new archetype** |
| C | Manual selection can violate archetype; meal becomes "Experimental" |

**Rationale for Option B:** It respects user intent (they picked this ingredient for a reason) while keeping the rest of the plate coherent. Option A can produce impossible states if the manually selected ingredient doesn't fit the current archetype. Option C trains users to ignore the archetype system.

**Implementation note:** When a user picks from the ingredient picker:
1. Update that slot in `MealSelection`
2. Re-run archetype selection conditioned on the full current selection
3. Re-optimize all *unlocked* slots under the new archetype
4. Update `meal_style_seed` to get a fresh chef description

---

## 8. Chef Description v2 (Maître d' Upgrade)

### 8.1 Current → Target

**Current:**
> "This evening, we present Steak served alongside Mashed Potatoes, with a side of Asparagus — complemented by Caesar Salad."

**Target:**
> "Good evening. Tonight, the Chef presents a dry-aged ribeye, seared hard over cast iron with fresh thyme and roasted garlic — finished with a velvety red wine pan jus. Accompanied by a silky pomme purée with chives, oven-roasted asparagus brightened with lemon zest, and a classic Caesar with Parmigiano-Reggiano. Bon appétit."

### 8.2 Assembly Logic

```
describe_meal_chef_lite(plan, seed) -> String:

1. Seed SmallRng from plan.meal_style_seed
2. Look up protein in INGREDIENT_DESC_LITE
3. Select method, seasoning, finish using seeded RNG
4. Compose protein_phrase: "{fancy_name}, {method} with {seasoning} — {finish}"
5. For each side (starch, veg1, veg2):
   - Look up starch role tag → select adjective from STARCH_ADJECTIVES pool
   - Check hot_veg / cold_side tag → select from VEG_ADJECTIVES_HOT or VEG_ADJECTIVES_COLD
   - If descriptor entry exists: use specific phrase; else: "{adjective} {ingredient.name}"
6. Select archetype voice template from plan.archetype chef_voice field:
   - "elegant": "Good evening. Tonight, the Chef presents {protein_phrase}. Accompanied by {starch_phrase}, {veg1_phrase}{, and {veg2_phrase}}. Bon appétit."
   - "casual": "Tonight: {protein_phrase}, with {starch_phrase} and {veg1_phrase}."
   - "bold": "Get ready. {protein_phrase}. Paired with {starch_phrase} and {veg1_phrase}."
7. Fallback: existing describe_meal() output if protein missing from INGREDIENT_DESC_LITE
```

### 8.3 Deterministic Variation

- `meal_style_seed: Signal<u64>` stored in `MealGenerator` state.
- Updated on: Generate, Reroll, manual selection confirm.
- Same seed + same meal = same description on every render. No flickering.
- Generate seed via `rand::random::<u64>()` at generation time.

---

## 9. Chef Notes (Inline Flavor Guidance)

### 9.1 Replace Seasoning Pills

Current `SAUCE_SUGGESTIONS` is cuisine-level and disconnected from what was actually generated. Replace with per-ingredient bullets.

### 9.2 Format

```
[Archetype label]
· [Protein name] — [method]; [finish]
· [Starch name] — [adjective phrase]
· [Veg1 name] — [method]; [finish or brief note]
· [Veg2 name] — [brief phrase]  (if present)
```

Example (Steakhouse / American / Steak + Mashed + Asparagus + Caesar):
```
Steakhouse Night
· Ribeye — sear hard on cast iron; rest 5 min; deglaze with red wine for pan jus
· Mashed Potatoes — finish with cold butter and chives; season aggressively
· Asparagus — roast at 425°F; finish with lemon zest and good olive oil
· Caesar — classic dressing; fresh Parm; extra cracked black pepper
```

---

## 10. Recipe Finder (No Backend Required)

After generating a meal the user likes, provide search launch buttons that open external links. Treat domains as configurable constants — do not hardcode URLs in logic.

```rust
// src/content/recipe_sites.rs (or top of quality_engine.rs)
pub const RECIPE_SEARCH_GOOGLE: &str = "https://www.google.com/search?q=";
pub const RECIPE_SEARCH_YOUTUBE: &str = "https://www.youtube.com/results?search_query=";
pub const RECIPE_SEARCH_SERIOUS_EATS: &str = "https://www.seriouseats.com/search?q=";

pub fn build_recipe_query(plan: &MealPlan) -> String {
    if let Some(ref name) = plan.dish_name {
        format!("{} recipe", name)
    } else {
        let p = plan.selection.protein.map(|p| p.id).unwrap_or("");
        let c = cuisine_label(plan.cuisine);
        format!("{} {} recipe", p, c)
    }
}
```

Links open via `web_sys::Window::open_with_url` or plain anchor tags with `target="_blank"`.

---

## 11. Editorial Work Plan (Descriptor Tables)

This is the largest single effort item — roughly 60-70% of Phase 1.5 + Phase 6 total work. Treat it as a content production sprint.

### 11.1 Minimum Viable Coverage

| Category | MVP Target (Phase 1.5) | Full Target (Phase 6) |
|---|---|---|
| Proteins | 100% — all proteins × applicable cuisines | 100% including new proteins from Phase 2 |
| Starches | Adjective pool only (no per-item tables) | Top 12 with specific phrases |
| Vegetables | Adjective pool only | Top 20 with specific phrases |

### 11.2 Per-Protein Deliverables

For each protein × applicable cuisine:
- 3 method variants
- 5 seasoning combos (mix of simple and specific)
- 3 finish variants
- 1–2 cuisine-specific fancy names

### 11.3 Voice Guide

- Sentences are short, active, imperative
- **Allowed:** specific ingredient names (thyme, not just "herbs"), temperature cues (high heat, low and slow), texture cues (silky, crispy, charred)
- **Banned:** vague adjectives ("delicious", "amazing"), passive voice, more than 15 words per bullet
- **Never write:** ingredient-method contradictions — no "braised shrimp", no "slow-roasted Caesar", no "seared tuna noodle soup"

### 11.4 Sanity Harness

Build before shipping Chef Notes to production:

```rust
#[cfg(debug_assertions)]
fn dump_sample_descriptions(n: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        // generate random MealPlan, print chef_desc and chef_notes
        // scan output manually for: duplicates, awkward phrases, contradictions
    }
}
```

### 11.5 Acceptance Criteria for Descriptors

- 200-sample dump yields < 5 outputs requiring rewrite.
- Zero ingredient-method contradictions in sample.
- At least 60% of generated meals feel "named" or plausibly restaurant-like.

---

## 12. UI Changes

### 12.1 New Elements

- **Archetype Badge** — small tag near cuisine selector: "Steakhouse", "Tacos", "Noodle Bowl"
- **Dish Name** — displayed above the maître d' paragraph (when available)
- **Chef Notes section** — replaces seasoning pills; 2–4 ingredient-specific bullets
- **"Find a Recipe" buttons** — appear below Chef Notes (Phase 7, optional)

### 12.2 Updated Maître d' Area Layout

```
[Generate Button]
[Dish Name — e.g. "Steakhouse Ribeye Night"]    ← new
[Archetype badge — "Steakhouse"]                ← new
[Maître d' description paragraph — richer prose]
[Slot cards]
[Chef Notes bullets — 2–4 lines]               ← replaces seasoning pills
[Find a Recipe buttons — optional]             ← new (Phase 7)
```

### 12.3 Preserved Elements

- Cuisine selector, pills, lock toggle
- Slot cards with lock, reroll, picker
- Veg2 toggle

---

## 13. Engineering Plan / Phases

> **Goal:** shippable at the end of each phase. No big-bang releases.

### Phase 0 — Hygiene Only *(renames + CSS fixes, NO logic changes)*

- [ ] Fix CSS syntax error in `public/assets/css/slots.css`
- [ ] Rename "Cabbage Slaw" in `meal_data.rs` (was "Asian Cabbage Slaw")
- [ ] Rename "Ground Turkey" (remove "(taco)")
- [ ] Remove `jasmine_rice` from BBQ cuisine pairings
- [ ] Remove `ground_turkey` from American cuisine pairings

### Phase 1 — Ingredient Tags + Redundancy Rules *(first quality win)*

- [ ] Add `src/content/meta.rs`
  - [ ] `IngredientMeta` struct + static table for all current ingredients
  - [ ] `has_tag(id, tag) -> bool`
  - [ ] `exclusion_group(id) -> &[&str]`
  - [ ] `assert_meta_coverage()` debug assertion
- [ ] Implement `check_plate_rules(selection: &MealSelection) -> bool`:
  - [ ] No two items from same exclusion group
  - [ ] `bitter_green` count ≤ 1
  - [ ] `tomato_forward` count ≤ 1
  - [ ] `corn_forward` count ≤ 1 (catches corn + elote, corn + succotash)
  - [ ] `east_asian_signal` + `south_asian_signal` never co-occur
  - [ ] `italian_american_signal` + `southern_signal` never co-occur (pasta + collard greens)
- [ ] Wire `check_plate_rules` into generation cascade and reroll handlers
- [ ] Tests: verify each exclusion group rule fires correctly

### Phase 1.5 — Chef Description MVP *(highest leverage, pull forward)*

- [ ] Add `src/content/desc_lite.rs`
  - [ ] `IngredientDescLite` struct
  - [ ] Full table for all 15 existing proteins × applicable cuisines
  - [ ] `STARCH_ADJECTIVES` and `VEG_ADJECTIVES_*` pools
  - [ ] `assert_desc_coverage()` debug assertion
- [ ] Add `meal_style_seed: Signal<u64>` to `MealGenerator` state
- [ ] Update seed on Generate, Reroll, manual selection confirm
- [ ] Implement `describe_meal_chef_lite(plan, seed) -> String`
- [ ] Keep `describe_meal()` as fallback
- [ ] Show dish name area in UI (placeholder if archetype not yet shipping)
- [ ] Run 200-sample sanity dump; iterate on awkward outputs

### Phase 2 — Protein Expansion *(can run in parallel with Phase 1)*

- [ ] Add to `meal_data.rs`:
  - [ ] BBQ: Baby Back Ribs, Beef Brisket, Pulled Pork, Smoked Sausage/Hot Links, Smoked Turkey
  - [ ] Latin: Carnitas, Carne Asada
  - [ ] Mediterranean: Lamb Chops
  - [ ] Asian: Pork Belly, Tofu
  - [ ] American: Meatloaf (optional)
- [ ] Add pairing data for all new proteins
- [ ] Add `IngredientMeta` entries (required — assert_meta_coverage will catch omissions)
- [ ] Add `IngredientDescLite` entries (required before Phase 6 ships)

### Phase 3 — Pairwise Coherence Gate *(Tier 2 scoring)*

- [ ] Add `pairs_any_direction(a_id, b_id, cuisine) -> bool`
- [ ] Add `meal_pairwise_score(selection, cuisine) -> i32` (sums edge scores)
- [ ] Update generation to filter by `check_plate_rules` first, then score
- [ ] Verify fallback fires cleanly when no candidates pass Tier 1

### Phase 4 — Best-of-N Quality Engine

- [ ] Create `src/components/pages/quality_engine.rs`
  - [ ] Scoring weight constants at top of file
  - [ ] `score_meal(selection, cuisine, archetype_id) -> i32`
  - [ ] `generate_candidate_meal(...) -> Option<MealSelection>`
  - [ ] `generate_best_meal(n: usize, ...) -> MealSelection`
- [ ] `N = 40` default; store as `const usize CANDIDATE_N: usize = 40`
- [ ] Replace `cascade_from_protein` with best-of-N in `generate_meal` handler
- [ ] Update reroll handlers to use local best-of-N

### Phase 5 — Archetypes (Dish-Family Coherence)

- [ ] Add `src/content/archetypes.rs` with full static definitions
- [ ] Add `archetype: Signal<&'static str>` to `MealGenerator` state
- [ ] Implement archetype selection (cuisine → protein → archetype)
- [ ] Enforce archetype constraints in Quality Engine (Tier 1 hard constraints)
- [ ] Display archetype badge in UI
- [ ] Implement manual-selection-triggers-archetype-repick (Option B from Section 7.5)
- [ ] Implement `describe_meal_chef()` full version with archetype voice templates

### Phase 6 — Chef Notes UI + Full Descriptor Tables

- [ ] Replace seasoning pills section with Chef Notes bullets
- [ ] Display dish name above maître d' paragraph
- [ ] Expand `desc_lite.rs` to cover top 12 starches and top 20 vegetables
- [ ] Run final 200-sample sanity dump; fix any remaining awkward outputs

### Phase 7 — Recipe Finder (Optional)

- [ ] Add `build_recipe_query(plan) -> String` to quality_engine or separate module
- [ ] Add configurable domain constants
- [ ] Add "Find a Recipe" / "Search YouTube" buttons in UI
- [ ] Open links via `web_sys::Window::open_with_url`

---

## 14. File / Module Map

```
src/
├── content/
│   ├── meal_data.rs         # existing ingredient + pairing graph (add new proteins here)
│   ├── meta.rs              # NEW: IngredientMeta, tags, exclusion groups, coverage assertion
│   ├── archetypes.rs        # NEW: Archetype definitions + constraints
│   └── desc_lite.rs         # NEW: IngredientDescLite, adjective pools, coverage assertion
└── components/pages/
    ├── quality_engine.rs    # NEW: best-of-N, scoring constants, plate rules
    ├── meal_types.rs        # existing types; add MealPlan
    ├── meal_generator.rs    # add meal_style_seed, archetype signal
    └── meal_slot.rs         # minimal changes; update manual selection to trigger archetype repick

public/assets/css/
├── variables.css
├── animations.css
├── base.css
├── components.css           # add: .archetype-badge, .dish-name, .chef-notes
├── slots.css                # fix CSS bug (Phase 0)
└── mobile-slots.css
```

---

## 15. Acceptance Criteria

### Quality
- Double salads, corn-on-corn, tomato-on-tomato: never appear.
- BBQ meals never include jasmine rice.
- Signal conflicts (naan + bok choy, pasta + collard greens): never appear.
- At least 80% of generated meals can be given a recognizable dish archetype name.

### Chef Description
- Every meal with a protein produces a chef-styled description.
- Description is stable across renders; only changes on user-initiated events.
- No ingredient-method contradictions in 200-sample test.
- Tone is consistent and not purple.

### UX
- Generate remains subjectively instant (best-of-N in WASM must be imperceptible).
- Locks, rerolls, and manual selection behave predictably and consistently.
- Chef Notes are ingredient-specific, not generic cuisine filler.
- Manual selection triggers coherent archetype re-pick, not broken state.

### Maintainability
- No function exceeds cognitive complexity 15.
- No function exceeds 5 arguments.
- New proteins added to `meal_data.rs` are caught by coverage assertions if `meta.rs` or `desc_lite.rs` is not updated.
- All scoring weights are named `const` values.

---

## 16. Appendix: Sample Target Output

**Generated:** Steak / American / Steakhouse archetype / Mashed Potatoes + Asparagus + Caesar

```
                    Steakhouse Night  [badge]

  "Good evening. Tonight, the Chef presents a dry-aged ribeye, seared 
   hard over cast iron with fresh thyme and roasted garlic — finished 
   with a velvety red wine pan jus. Accompanied by a silky pomme purée 
   with chives, oven-roasted asparagus brightened with lemon zest, and 
   a classic Caesar with Parmigiano-Reggiano. Bon appétit."

  · Ribeye — sear hard on cast iron; rest 5 min; deglaze for pan jus
  · Mashed Potatoes — finish with cold butter and chives
  · Asparagus — roast at 425°F; lemon zest + olive oil to finish
  · Caesar — fresh Parm; extra cracked black pepper

  [Find a Recipe ↗]   [YouTube ↗]
```

---

*v2.1 — Last updated 2026-02-19. Incorporates: multi-session AI code review, 100+ sample generation analysis, protein gap analysis, and 8-point spec audit.*
