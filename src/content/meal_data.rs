// Static Data Module for Meal Generation

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ingredient {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub buy_amount: Option<&'static str>,
    pub cuisines: &'static [(&'static str, &'static [&'static str])],
}

pub const CUISINES: &[&str] = &["american", "latin", "asian", "mediterranean", "bbq"];
pub const CUISINE_LABELS: &[(&str, &str)] = &[
    ("american", "American"),
    ("latin", "Latin / Mexican"),
    ("asian", "Asian"),
    ("mediterranean", "Mediterranean"),
    ("bbq", "BBQ / Comfort"),
];

macro_rules! ingredient {
    ($id:literal, $name:literal, $cat:literal, $buy:literal, $cuisines:expr) => {
        Ingredient {
            id: $id,
            name: $name,
            category: $cat,
            buy_amount: Some($buy),
            cuisines: $cuisines,
        }
    };
    ($id:literal, $name:literal, $cat:literal, $cuisines:expr) => {
        Ingredient {
            id: $id,
            name: $name,
            category: $cat,
            buy_amount: None,
            cuisines: $cuisines,
        }
    };
}

pub static INGREDIENTS: &[Ingredient] = &[
    // ===== PROTEINS =====
    ingredient!("chicken_breast", "Chicken Breast", "protein", "2.5 lbs", &[
        ("american", &["mashed_potatoes", "green_beans", "roasted_potatoes", "broccoli", "garlic_bread", "caesar_salad", "sweet_potato", "corn"]),
        ("asian", &["fried_rice", "bok_choy", "jasmine_rice", "broccoli", "sugar_snap_peas", "egg_noodles", "bean_sprouts", "cabbage_slaw"]),
        ("latin", &["cilantro_lime_rice", "black_beans_rice", "bell_peppers", "tortillas", "corn", "avocado_salad", "elote_corn", "mexican_rice"]),
        ("mediterranean", &["couscous", "quinoa", "greek_salad", "roasted_tomatoes", "zucchini", "cucumber_tomato", "rice_pilaf", "spinach"]),
        ("bbq", &["cornbread", "coleslaw", "baked_beans", "mashed_potatoes", "mac_cheese", "collard_greens", "corn", "biscuits"]),
    ]),
    ingredient!("chicken_thighs", "Chicken Thighs", "protein", "2.5 lbs", &[
        ("american", &["roasted_potatoes", "green_beans", "mashed_potatoes", "carrots", "sweet_potato", "bread_rolls", "brussels_sprouts", "corn"]),
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "broccoli", "sugar_snap_peas", "egg_noodles", "cabbage_slaw", "edamame"]),
        ("latin", &["cilantro_lime_rice", "tortillas", "black_beans_rice", "bell_peppers", "corn", "elote_corn", "avocado_salad", "mexican_rice"]),
        ("mediterranean", &["rice_pilaf", "roasted_tomatoes", "zucchini", "couscous", "greek_salad", "spinach", "cucumber_tomato", "quinoa"]),
        ("bbq", &["cornbread", "coleslaw", "mac_cheese", "baked_beans", "mashed_potatoes", "collard_greens", "corn", "sweet_potato"]),
    ]),
    ingredient!("drumsticks", "Chicken Drumsticks", "protein", "12 drumsticks", &[
        ("american", &["mashed_potatoes", "corn", "coleslaw", "baked_potato", "mac_cheese", "green_beans", "biscuits", "cornbread"]),
        ("bbq", &["cornbread", "coleslaw", "baked_beans", "mac_cheese", "corn", "collard_greens", "biscuits", "sweet_potato"]),
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "broccoli", "cabbage_slaw", "egg_noodles", "sugar_snap_peas", "bean_sprouts"]),
        ("latin", &["cilantro_lime_rice", "black_beans_rice", "elote_corn", "tortillas", "corn", "bell_peppers", "mexican_rice", "avocado_salad"]),
    ]),
    ingredient!("steak", "Steak", "protein", "2 lbs", &[
        ("american", &["mashed_potatoes", "baked_potato", "twice_baked_potato", "au_gratin_potatoes", "hash_browns", "fries", "asparagus", "mushrooms", "garlic_bread", "roasted_potatoes", "green_beans", "caesar_salad", "creamed_corn"]),
        ("latin", &["cilantro_lime_rice", "black_beans_rice", "bell_peppers", "grilled_onions", "tortillas", "corn", "avocado_salad", "mexican_rice"]),
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "broccoli", "egg_noodles", "sugar_snap_peas", "bean_sprouts", "cabbage_slaw"]),
        ("bbq", &["cornbread", "coleslaw", "baked_potato", "mashed_potatoes", "mac_cheese", "baked_beans", "corn", "grilled_onions"]),
        ("mediterranean", &["roasted_potatoes", "asparagus", "roasted_tomatoes", "zucchini", "rice_pilaf", "greek_salad", "couscous", "grilled_onions"]),
    ]),
    ingredient!("ground_beef", "Ground Beef", "protein", "2.5 lbs", &[
        ("american", &["mashed_potatoes", "pasta", "garlic_bread", "green_beans", "corn", "mac_cheese", "baked_potato", "caesar_salad"]),
        ("latin", &["tortillas", "cilantro_lime_rice", "black_beans_rice", "bell_peppers", "corn", "mexican_rice", "elote_corn", "avocado_salad"]),
        ("bbq", &["cornbread", "baked_beans", "coleslaw", "mashed_potatoes", "mac_cheese", "collard_greens", "biscuits", "corn"]),
        ("asian", &["fried_rice", "jasmine_rice", "egg_noodles", "bok_choy", "broccoli", "bell_peppers", "bean_sprouts", "sugar_snap_peas"]),
    ]),
    ingredient!("ground_turkey", "Ground Turkey (taco)", "protein", "2.5 lbs", &[
        ("latin", &["tortillas", "cilantro_lime_rice", "black_beans_rice", "bell_peppers", "corn", "mexican_rice", "avocado_salad", "elote_corn"]),
        ("american", &["pasta", "mashed_potatoes", "green_beans", "garlic_bread", "corn", "roasted_potatoes", "caesar_salad", "zucchini"]),
    ]),
    ingredient!("pork_chops", "Pork Chops", "protein", "2.5 lbs", &[
        ("american", &["mashed_potatoes", "green_beans", "corn", "sweet_potato", "roasted_potatoes", "bread_rolls", "baked_potato", "carrots"]),
        ("bbq", &["cornbread", "coleslaw", "collard_greens", "baked_beans", "mac_cheese", "mashed_potatoes", "corn", "sweet_potato"]),
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "broccoli", "egg_noodles", "cabbage_slaw", "sugar_snap_peas", "bean_sprouts"]),
        ("latin", &["cilantro_lime_rice", "black_beans_rice", "tortillas", "bell_peppers", "corn", "elote_corn", "mexican_rice", "avocado_salad"]),
    ]),
    ingredient!("pork_tenderloin", "Pork Tenderloin", "protein", "2 lbs", &[
        ("american", &["roasted_potatoes", "green_beans", "mashed_potatoes", "asparagus", "brussels_sprouts", "sweet_potato", "bread_rolls", "carrots"]),
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "broccoli", "sugar_snap_peas", "egg_noodles", "cabbage_slaw", "edamame"]),
        ("bbq", &["cornbread", "coleslaw", "baked_beans", "mac_cheese", "mashed_potatoes", "collard_greens", "corn", "sweet_potato"]),
        ("mediterranean", &["couscous", "quinoa", "roasted_tomatoes", "asparagus", "zucchini", "rice_pilaf", "greek_salad", "spinach"]),
    ]),
    ingredient!("sausage", "Italian Sausage", "protein", "2 lbs", &[
        ("american", &["mashed_potatoes", "pasta", "garlic_bread", "bell_peppers", "green_beans", "roasted_potatoes", "corn", "zucchini"]),
        ("mediterranean", &["pasta", "couscous", "roasted_tomatoes", "bell_peppers", "zucchini", "garlic_bread", "greek_salad", "spinach"]),
        ("bbq", &["cornbread", "baked_beans", "coleslaw", "mashed_potatoes", "mac_cheese", "collard_greens", "corn", "grilled_onions"]),
    ]),
    ingredient!("salmon", "Salmon", "protein", "2 lbs", &[
        ("asian", &["jasmine_rice", "bok_choy", "edamame", "sugar_snap_peas", "fried_rice", "broccoli", "cabbage_slaw", "spinach"]),
        ("american", &["roasted_potatoes", "asparagus", "quinoa", "green_beans", "broccoli", "rice_pilaf", "mixed_salad", "sweet_potato"]),
        ("mediterranean", &["couscous", "asparagus", "roasted_tomatoes", "zucchini", "quinoa", "greek_salad", "cucumber_dill", "spinach"]),
        ("bbq", &["cornbread", "coleslaw", "roasted_potatoes", "corn", "asparagus", "sweet_potato", "mixed_salad", "grilled_onions"]),
    ]),
    ingredient!("shrimp", "Shrimp", "protein", "2 lbs", &[
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "sugar_snap_peas", "egg_noodles", "bean_sprouts", "broccoli", "cabbage_slaw"]),
        ("latin", &["cilantro_lime_rice", "tortillas", "black_beans_rice", "bell_peppers", "corn", "mexican_rice", "avocado_salad", "elote_corn"]),
        ("mediterranean", &["couscous", "quinoa", "greek_salad", "zucchini", "roasted_tomatoes", "cucumber_dill", "rice_pilaf", "spinach"]),
        ("american", &["garlic_bread", "pasta", "green_beans", "corn", "mixed_salad", "roasted_potatoes", "rice_pilaf", "caesar_salad"]),
        ("bbq", &["cornbread", "coleslaw", "corn", "roasted_potatoes", "grilled_onions", "mixed_salad", "biscuits", "fries"]),
    ]),
    ingredient!("tuna", "Tuna Steaks", "protein", "2 lbs", &[
        ("mediterranean", &["couscous", "greek_salad", "cucumber_dill", "roasted_tomatoes", "quinoa", "zucchini", "spinach", "cucumber_tomato"]),
        ("asian", &["jasmine_rice", "bok_choy", "edamame", "sugar_snap_peas", "cabbage_slaw", "fried_rice", "pickled_vegetables", "bean_sprouts"]),
        ("american", &["roasted_potatoes", "green_beans", "mixed_salad", "quinoa", "asparagus", "rice_pilaf", "cucumber_tomato", "garden_salad"]),
    ]),
    ingredient!("white_fish", "White Fish (Tilapia/Cod)", "protein", "2 lbs", &[
        ("american", &["roasted_potatoes", "green_beans", "coleslaw", "fries", "corn", "mixed_salad", "rice_pilaf", "garden_salad"]),
        ("latin", &["tortillas", "cilantro_lime_rice", "black_beans_rice", "corn", "cabbage_slaw", "avocado_salad", "mexican_rice", "elote_corn"]),
        ("mediterranean", &["couscous", "roasted_tomatoes", "zucchini", "cucumber_dill", "quinoa", "greek_salad", "spinach", "cucumber_tomato"]),
        ("bbq", &["coleslaw", "cornbread", "fries", "corn", "roasted_potatoes", "biscuits", "mixed_salad", "garden_salad"]),
    ]),
    ingredient!("eggs", "Eggs", "protein", "1 dozen", &[
        ("american", &["biscuits", "roasted_potatoes", "green_beans", "mixed_salad", "bread_rolls", "sweet_potato", "mushrooms", "spinach"]),
        ("asian", &["fried_rice", "jasmine_rice", "bok_choy", "bean_sprouts", "sugar_snap_peas", "cabbage_slaw", "edamame", "broccoli"]),
        ("mediterranean", &["pita", "cucumber_tomato", "greek_salad", "spinach", "zucchini", "roasted_tomatoes", "couscous", "bell_peppers"]),
    ]),
    ingredient!("bacon", "Bacon", "protein", "1 lb", &[
        ("american", &["eggs", "biscuits", "green_beans", "mashed_potatoes", "mac_cheese", "corn", "brussels_sprouts", "baked_potato"]),
        ("bbq", &["cornbread", "baked_beans", "collard_greens", "mashed_potatoes", "mac_cheese", "coleslaw", "biscuits", "corn"]),
    ]),

    // ===== STARCHES =====
    ingredient!("jasmine_rice", "Jasmine Rice", "starch", &[
        ("asian", &["chicken_breast", "bok_choy", "broccoli", "shrimp", "salmon", "sugar_snap_peas", "tuna", "edamame"]),
        ("american", &["chicken_breast", "shrimp", "salmon", "green_beans", "broccoli", "mixed_salad", "carrots", "cauliflower"]),
        ("mediterranean", &["chicken_breast", "salmon", "shrimp", "zucchini", "roasted_tomatoes", "cucumber_tomato", "spinach", "bell_peppers"]),
        ("bbq", &["chicken_breast", "steak", "pork_chops", "collard_greens", "corn", "green_beans", "mixed_salad", "grilled_onions"]),
    ]),
    ingredient!("cilantro_lime_rice", "Cilantro Lime Rice", "starch", &[
        ("latin", &["chicken_breast", "steak", "shrimp", "ground_turkey", "bell_peppers", "corn", "avocado_salad", "elote_corn"]),
        ("american", &["chicken_breast", "shrimp", "steak", "green_beans", "corn", "mixed_salad", "avocado_salad", "bell_peppers"]),
    ]),
    ingredient!("fried_rice", "Fried Rice", "starch", &[
        ("asian", &["chicken_breast", "shrimp", "bok_choy", "broccoli", "sugar_snap_peas", "bean_sprouts", "eggs", "edamame"]),
    ]),
    ingredient!("rice_pilaf", "Rice Pilaf", "starch", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "pork_tenderloin", "roasted_tomatoes", "zucchini", "spinach"]),
        ("american", &["chicken_breast", "chicken_thighs", "pork_tenderloin", "green_beans", "carrots", "broccoli", "mushrooms", "asparagus"]),
    ]),
    ingredient!("black_beans_rice", "Black Beans & Rice", "starch", &[
        ("latin", &["chicken_breast", "steak", "shrimp", "ground_beef", "ground_turkey", "bell_peppers", "corn", "avocado_salad"]),
    ]),
    ingredient!("mashed_potatoes", "Mashed Potatoes", "starch", &[
        ("american", &["chicken_breast", "steak", "drumsticks", "pork_chops", "green_beans", "corn", "mushrooms", "carrots"]),
        ("bbq", &["drumsticks", "chicken_thighs", "steak", "pork_chops", "coleslaw", "collard_greens", "corn", "baked_beans"]),
    ]),
    ingredient!("roasted_potatoes", "Roasted Potatoes", "starch", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "pork_tenderloin", "asparagus", "green_beans", "carrots"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "steak", "zucchini", "roasted_tomatoes", "asparagus", "spinach"]),
        ("bbq", &["steak", "chicken_thighs", "salmon", "pork_tenderloin", "asparagus", "grilled_onions", "mixed_salad", "corn"]),
    ]),
    ingredient!("baked_potato", "Baked Potato", "starch", &[
        ("american", &["steak", "chicken_breast", "pork_chops", "drumsticks", "green_beans", "corn", "broccoli", "bacon"]),
        ("bbq", &["steak", "chicken_breast", "pork_chops", "drumsticks", "coleslaw", "corn", "baked_beans", "green_beans"]),
    ]),
    ingredient!("twice_baked_potato", "Twice-Baked Potato", "starch", &[
        ("american", &["steak", "chicken_breast", "pork_chops", "green_beans", "broccoli", "asparagus"]),
        ("bbq", &["steak", "pork_chops", "green_beans"]),
    ]),
    ingredient!("au_gratin_potatoes", "Potatoes Au Gratin", "starch", &[
        ("american", &["steak", "chicken_breast", "pork_chops", "green_beans", "broccoli"]),
    ]),
    ingredient!("hash_browns", "Hash Browns", "starch", &[
        ("american", &["steak", "eggs", "bacon", "chicken_breast"]),
    ]),
    ingredient!("sweet_potato", "Sweet Potato", "starch", &[
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "steak", "pork_chops", "green_beans", "broccoli", "brussels_sprouts"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "pork_chops", "drumsticks", "collard_greens", "corn", "coleslaw", "green_beans"]),
        ("mediterranean", &["chicken_breast", "salmon", "pork_tenderloin", "zucchini", "roasted_squash", "spinach", "quinoa", "roasted_tomatoes"]),
        ("latin", &["chicken_breast", "steak", "pork_chops", "black_beans_rice", "corn", "avocado_salad", "bell_peppers", "elote_corn"]),
    ]),
    ingredient!("fries", "Fries / Wedges", "starch", &[
        ("american", &["steak", "chicken_breast", "drumsticks", "white_fish", "coleslaw", "mixed_salad", "corn", "green_beans"]),
        ("bbq", &["steak", "drumsticks", "white_fish", "chicken_breast", "coleslaw", "corn", "mixed_salad", "shrimp"]),
    ]),
    ingredient!("bread_rolls", "Bread / Rolls", "starch", &[
        ("american", &["steak", "chicken_breast", "chicken_thighs", "pork_chops", "green_beans", "corn", "mixed_salad", "pork_tenderloin"]),
        ("bbq", &["steak", "pork_chops", "drumsticks", "chicken_thighs", "collard_greens", "coleslaw", "baked_beans", "corn"]),
        ("mediterranean", &["chicken_breast", "salmon", "shrimp", "greek_salad", "roasted_tomatoes", "zucchini", "cucumber_tomato", "spinach"]),
    ]),
    ingredient!("garlic_bread", "Garlic Bread", "starch", &[
        ("american", &["chicken_breast", "steak", "pasta", "sausage", "green_beans", "caesar_salad", "mixed_salad", "broccoli"]),
        ("mediterranean", &["pasta", "chicken_breast", "sausage", "shrimp", "roasted_tomatoes", "zucchini", "greek_salad", "spinach"]),
    ]),
    ingredient!("naan", "Naan / Flatbread", "starch", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "salmon", "cucumber_dill", "roasted_tomatoes", "spinach"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "bok_choy", "broccoli", "cabbage_slaw", "cauliflower", "sugar_snap_peas"]),
    ]),
    ingredient!("tortillas", "Tortillas", "starch", &[
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "ground_beef", "ground_turkey", "shrimp", "white_fish", "bell_peppers"]),
    ]),
    ingredient!("cornbread", "Cornbread", "starch", &[
        ("bbq", &["drumsticks", "chicken_breast", "chicken_thighs", "pork_chops", "collard_greens", "baked_beans", "coleslaw", "corn"]),
        ("american", &["drumsticks", "chicken_breast", "chicken_thighs", "ground_beef", "collard_greens", "baked_beans", "green_beans", "corn"]),
        ("latin", &["chicken_breast", "chicken_thighs", "ground_beef", "black_beans_rice", "elote_corn", "avocado_salad", "bell_peppers", "corn"]),
    ]),
    ingredient!("pasta", "Pasta / Penne", "starch", &[
        ("american", &["chicken_breast", "chicken_thighs", "sausage", "shrimp", "ground_beef", "garlic_bread", "green_beans", "caesar_salad"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "sausage", "shrimp", "roasted_tomatoes", "zucchini", "spinach", "greek_salad"]),
    ]),
    ingredient!("egg_noodles", "Egg Noodles", "starch", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "pork_chops", "ground_beef", "mushrooms", "green_beans", "carrots"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "bok_choy", "broccoli", "bean_sprouts", "sugar_snap_peas"]),
    ]),
    ingredient!("couscous", "Couscous", "starch", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "tuna", "white_fish", "roasted_tomatoes", "zucchini"]),
    ]),
    ingredient!("quinoa", "Quinoa", "starch", &[
        ("mediterranean", &["chicken_breast", "salmon", "shrimp", "tuna", "roasted_tomatoes", "zucchini", "spinach", "cucumber_tomato"]),
        ("american", &["chicken_breast", "salmon", "shrimp", "tuna", "asparagus", "broccoli", "green_beans", "avocado_salad"]),
        ("latin", &["chicken_breast", "chicken_thighs", "shrimp", "avocado_salad", "bell_peppers", "corn", "cucumber_tomato", "roasted_tomatoes"]),
    ]),
    ingredient!("mac_cheese", "Mac & Cheese", "starch", &[
        ("american", &["chicken_breast", "chicken_thighs", "drumsticks", "pork_chops", "ground_beef", "broccoli", "green_beans", "corn"]),
        ("bbq", &["drumsticks", "chicken_thighs", "pork_chops", "steak", "collard_greens", "coleslaw", "corn", "baked_beans"]),
    ]),
    ingredient!("yellow_rice", "Yellow Rice", "starch", &[
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "bell_peppers", "corn", "avocado_salad", "elote_corn"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "shrimp", "zucchini", "roasted_tomatoes", "spinach", "cucumber_tomato", "bell_peppers"]),
    ]),
    ingredient!("polenta", "Polenta", "starch", &[
        ("mediterranean", &["sausage", "chicken_thighs", "mushrooms", "roasted_tomatoes", "spinach", "zucchini", "eggplant", "bell_peppers"]),
        ("american", &["sausage", "chicken_thighs", "mushrooms", "green_beans", "spinach", "roasted_tomatoes", "zucchini", "corn"]),
    ]),
    ingredient!("stuffing", "Stuffing", "starch", &[
        ("american", &["chicken_breast", "chicken_thighs", "pork_chops", "green_beans", "corn", "carrots", "mashed_potatoes", "brussels_sprouts"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "pork_chops", "collard_greens", "green_beans", "corn", "mashed_potatoes", "brussels_sprouts"]),
    ]),
    ingredient!("pita", "Pita Bread", "starch", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "shrimp", "greek_salad", "cucumber_dill", "roasted_tomatoes", "cucumber_tomato", "zucchini"]),
    ]),
    ingredient!("biscuits", "Biscuits", "starch", &[
        ("american", &["chicken_breast", "drumsticks", "eggs", "bacon", "sausage", "mashed_potatoes", "green_beans", "corn"]),
        ("bbq", &["drumsticks", "chicken_thighs", "pork_chops", "sausage", "mashed_potatoes", "collard_greens", "baked_beans", "coleslaw"]),
    ]),
    ingredient!("mexican_rice", "Mexican Rice", "starch", &[
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "ground_beef", "ground_turkey", "shrimp", "bell_peppers", "corn"]),
    ]),
    ingredient!("creamed_corn", "Creamed Corn", "starch", &[
        ("american", &["steak", "pork_chops", "mashed_potatoes"]),
        ("bbq", &["cornbread", "baked_beans"]),
    ]),

    // ===== VEGETABLES =====
    ingredient!("broccoli", "Broccoli", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "pasta", "roasted_potatoes", "garlic_bread", "cauliflower"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "fried_rice", "jasmine_rice", "bok_choy", "sugar_snap_peas"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "steak", "salmon", "roasted_potatoes", "corn", "mashed_potatoes", "mac_cheese"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "quinoa", "couscous", "roasted_tomatoes", "zucchini"]),
    ]),
    ingredient!("green_beans", "Green Beans", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "pork_chops", "mashed_potatoes", "roasted_potatoes", "mushrooms", "carrots"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "steak", "pork_chops", "mashed_potatoes", "corn", "collard_greens", "baked_beans"]),
        ("asian", &["chicken_breast", "shrimp", "steak", "fried_rice", "jasmine_rice", "bok_choy", "sugar_snap_peas", "broccoli"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "roasted_potatoes", "roasted_tomatoes", "zucchini", "couscous"]),
    ]),
    ingredient!("mixed_salad", "Mixed Salad", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "shrimp", "garlic_bread", "roasted_potatoes", "quinoa"]),
        ("latin", &["chicken_breast", "steak", "shrimp", "cilantro_lime_rice", "black_beans_rice", "avocado_salad", "corn", "cucumber_tomato"]),
        ("mediterranean", &["chicken_breast", "salmon", "shrimp", "tuna", "pita", "couscous", "cucumber_tomato", "greek_salad"]),
    ]),
    ingredient!("corn", "Corn", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "drumsticks", "pork_chops", "mashed_potatoes", "green_beans", "cornbread", "biscuits"]),
        ("bbq", &["drumsticks", "chicken_breast", "chicken_thighs", "pork_chops", "cornbread", "coleslaw", "baked_beans", "mashed_potatoes"]),
        ("latin", &["chicken_breast", "steak", "shrimp", "tortillas", "black_beans_rice", "elote_corn", "bell_peppers", "avocado_salad"]),
    ]),
    ingredient!("carrots", "Roasted Carrots", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "pork_tenderloin", "roasted_potatoes", "green_beans", "mashed_potatoes", "brussels_sprouts", "peas"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "fried_rice", "jasmine_rice", "bok_choy", "sugar_snap_peas", "broccoli"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "pork_tenderloin", "roasted_potatoes", "green_beans", "corn", "mashed_potatoes", "sweet_potato"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "couscous", "quinoa", "roasted_tomatoes", "zucchini", "spinach"]),
    ]),
    ingredient!("asparagus", "Asparagus", "veg", &[
        ("american", &["salmon", "steak", "chicken_breast", "pork_tenderloin", "roasted_potatoes", "mushrooms", "quinoa", "rice_pilaf"]),
        ("mediterranean", &["salmon", "steak", "chicken_breast", "tuna", "shrimp", "couscous", "roasted_tomatoes", "roasted_potatoes"]),
        ("bbq", &["salmon", "steak", "chicken_breast", "pork_tenderloin", "roasted_potatoes", "grilled_onions", "mixed_salad", "corn"]),
    ]),
    ingredient!("zucchini", "Zucchini", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "couscous", "roasted_tomatoes", "eggplant", "quinoa"]),
        ("american", &["chicken_breast", "chicken_thighs", "ground_turkey", "sausage", "pasta", "roasted_potatoes", "roasted_squash", "mushrooms"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "steak", "salmon", "grilled_onions", "bell_peppers", "roasted_squash", "mixed_salad"]),
        ("latin", &["chicken_breast", "chicken_thighs", "shrimp", "tortillas", "bell_peppers", "corn", "roasted_tomatoes", "avocado_salad"]),
    ]),
    ingredient!("roasted_tomatoes", "Roasted Tomatoes", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "couscous", "zucchini", "eggplant", "spinach"]),
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "tuna", "garlic_bread", "zucchini", "spinach"]),
        ("latin", &["chicken_breast", "steak", "shrimp", "tortillas", "cilantro_lime_rice", "black_beans_rice", "bell_peppers", "avocado_salad"]),
    ]),
    ingredient!("brussels_sprouts", "Brussels Sprouts", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "pork_chops", "roasted_potatoes", "bacon", "sweet_potato"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "steak", "pork_chops", "roasted_potatoes", "mashed_potatoes", "cornbread", "green_beans"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "steak", "salmon", "quinoa", "roasted_tomatoes", "roasted_potatoes", "couscous"]),
    ]),
    ingredient!("cauliflower", "Roasted Cauliflower", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "roasted_potatoes", "broccoli", "carrots", "green_beans"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "couscous", "quinoa", "roasted_tomatoes", "spinach"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "fried_rice", "jasmine_rice", "broccoli", "bok_choy", "naan"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "steak", "pork_tenderloin", "roasted_potatoes", "green_beans", "corn", "mashed_potatoes"]),
    ]),
    ingredient!("spinach", "Sautéed Spinach", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "tuna", "couscous", "garlic_bread", "quinoa"]),
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "steak", "pasta", "garlic_bread", "mushrooms", "eggs"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "salmon", "fried_rice", "jasmine_rice", "eggs", "bok_choy"]),
    ]),
    ingredient!("mushrooms", "Sautéed Mushrooms", "veg", &[
        ("american", &["steak", "chicken_breast", "chicken_thighs", "pork_chops", "mashed_potatoes", "green_beans", "roasted_potatoes", "asparagus"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "fried_rice", "bok_choy", "jasmine_rice", "egg_noodles"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "steak", "pork_tenderloin", "polenta", "spinach", "zucchini", "roasted_tomatoes"]),
    ]),
    ingredient!("eggplant", "Roasted Eggplant", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "couscous", "zucchini", "roasted_tomatoes", "polenta"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "jasmine_rice", "bok_choy", "fried_rice", "bell_peppers", "bean_sprouts"]),
    ]),
    ingredient!("bell_peppers", "Bell Peppers & Onions", "veg", &[
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "ground_beef", "tortillas", "cilantro_lime_rice", "black_beans_rice", "corn"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "fried_rice", "jasmine_rice", "bok_choy", "egg_noodles"]),
        ("american", &["chicken_breast", "chicken_thighs", "sausage", "steak", "rice_pilaf", "zucchini", "roasted_potatoes", "pasta"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "sausage", "shrimp", "couscous", "zucchini", "roasted_tomatoes", "eggplant"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "steak", "sausage", "grilled_onions", "corn", "mashed_potatoes", "bread_rolls"]),
    ]),
    ingredient!("elote_corn", "Mexican Street Corn", "veg", &[
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "ground_beef", "ground_turkey", "cilantro_lime_rice", "tortillas", "black_beans_rice"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "drumsticks", "steak", "cornbread", "coleslaw", "mashed_potatoes", "green_beans"]),
    ]),
    ingredient!("bok_choy", "Bok Choy", "veg", &[
        ("asian", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "steak", "tuna", "fried_rice", "jasmine_rice"]),
    ]),
    ingredient!("sugar_snap_peas", "Sugar Snap Peas", "veg", &[
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "salmon", "steak", "tuna", "fried_rice", "jasmine_rice"]),
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "roasted_potatoes", "carrots", "mushrooms", "rice_pilaf"]),
    ]),
    ingredient!("edamame", "Edamame", "veg", &[
        ("asian", &["salmon", "chicken_breast", "chicken_thighs", "shrimp", "tuna", "fried_rice", "jasmine_rice", "bok_choy"]),
    ]),
    ingredient!("bean_sprouts", "Bean Sprouts", "veg", &[
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "pork_chops", "fried_rice", "egg_noodles", "bok_choy"]),
    ]),
    ingredient!("cabbage_slaw", "Asian Cabbage Slaw", "veg", &[
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "salmon", "steak", "tuna", "white_fish", "fried_rice"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "drumsticks", "pork_chops", "steak", "cornbread", "mashed_potatoes", "baked_beans"]),
    ]),
    ingredient!("cucumber_dill", "Cucumber Dill Salad", "veg", &[
        ("mediterranean", &["tuna", "salmon", "shrimp", "chicken_breast", "white_fish", "pork_tenderloin", "pita", "couscous"]),
        ("american", &["salmon", "tuna", "chicken_breast", "shrimp", "mixed_salad", "roasted_potatoes", "quinoa", "garden_salad"]),
    ]),
    ingredient!("greek_salad", "Greek Salad", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "tuna", "white_fish", "steak", "pita"]),
    ]),
    ingredient!("coleslaw", "Coleslaw", "veg", &[
        ("bbq", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "shrimp", "white_fish", "cornbread", "baked_beans"]),
        ("american", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "fries", "corn", "mashed_potatoes", "biscuits"]),
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "white_fish", "tortillas", "cilantro_lime_rice", "black_beans_rice"]),
    ]),
    ingredient!("collard_greens", "Collard Greens", "veg", &[
        ("bbq", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "sausage", "pork_tenderloin", "cornbread", "baked_beans"]),
        ("american", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "sausage", "mashed_potatoes", "cornbread", "bacon"]),
    ]),
    ingredient!("baked_beans", "Baked Beans", "veg", &[
        ("bbq", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "sausage", "cornbread", "coleslaw", "mashed_potatoes"]),
        ("american", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "sausage", "cornbread", "coleslaw", "biscuits"]),
    ]),
    ingredient!("kale", "Sautéed Kale", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "steak", "pork_tenderloin", "garlic_bread", "mashed_potatoes", "bacon"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "couscous", "quinoa", "roasted_tomatoes", "cucumber_tomato"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "pork_chops", "steak", "cornbread", "mashed_potatoes", "collard_greens", "corn"]),
    ]),
    ingredient!("peas", "Green Peas", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "pork_chops", "mashed_potatoes", "carrots", "pasta", "rice_pilaf"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "fried_rice", "jasmine_rice", "bok_choy", "egg_noodles", "sugar_snap_peas"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "pasta", "couscous", "spinach", "roasted_tomatoes"]),
    ]),
    ingredient!("cucumber_tomato", "Cucumber Tomato Salad", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "tuna", "white_fish", "pita", "couscous"]),
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "cilantro_lime_rice", "black_beans_rice", "tortillas", "avocado_salad"]),
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "steak", "mixed_salad", "garlic_bread", "quinoa", "garden_salad"]),
    ]),
    ingredient!("roasted_squash", "Roasted Squash", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "pork_chops", "roasted_potatoes", "brussels_sprouts", "sweet_potato"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "quinoa", "zucchini", "roasted_tomatoes", "spinach"]),
        ("bbq", &["chicken_breast", "chicken_thighs", "pork_chops", "steak", "sweet_potato", "green_beans", "corn", "mashed_potatoes"]),
    ]),
    ingredient!("grilled_onions", "Grilled Onions", "veg", &[
        ("american", &["steak", "chicken_breast", "chicken_thighs", "pork_chops", "sausage", "mashed_potatoes", "green_beans", "bread_rolls"]),
        ("bbq", &["steak", "chicken_breast", "chicken_thighs", "pork_chops", "sausage", "cornbread", "mashed_potatoes", "baked_beans"]),
        ("latin", &["steak", "chicken_breast", "chicken_thighs", "tortillas", "bell_peppers", "cilantro_lime_rice", "black_beans_rice", "avocado_salad"]),
        ("mediterranean", &["steak", "chicken_breast", "chicken_thighs", "sausage", "zucchini", "roasted_tomatoes", "couscous", "bell_peppers"]),
    ]),
    ingredient!("roasted_beets", "Roasted Beets", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "pork_tenderloin", "mixed_salad", "roasted_potatoes", "quinoa", "garden_salad"]),
        ("mediterranean", &["chicken_breast", "salmon", "tuna", "white_fish", "couscous", "quinoa", "spinach", "cucumber_tomato"]),
    ]),
    ingredient!("sauteed_cabbage", "Sautéed Cabbage", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "pork_chops", "sausage", "drumsticks", "mashed_potatoes", "roasted_potatoes", "bacon"]),
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "fried_rice", "jasmine_rice", "egg_noodles", "bean_sprouts"]),
    ]),
    ingredient!("pickled_vegetables", "Pickled Vegetables", "veg", &[
        ("asian", &["chicken_breast", "chicken_thighs", "shrimp", "steak", "fried_rice", "jasmine_rice", "bok_choy", "tuna"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "pita", "couscous", "cucumber_tomato", "greek_salad"]),
    ]),
    ingredient!("avocado_salad", "Avocado Salad", "veg", &[
        ("latin", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "tortillas", "cilantro_lime_rice", "black_beans_rice", "corn"]),
        ("american", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "mixed_salad", "quinoa", "cucumber_tomato", "garden_salad"]),
    ]),
    ingredient!("black_eyed_peas", "Black-Eyed Peas", "veg", &[
        ("bbq", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "cornbread", "collard_greens", "mashed_potatoes", "bacon"]),
        ("american", &["drumsticks", "pork_chops", "chicken_breast", "chicken_thighs", "cornbread", "collard_greens", "mashed_potatoes", "bacon"]),
    ]),
    ingredient!("succotash", "Succotash", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "drumsticks", "pork_chops", "salmon", "cornbread", "biscuits", "green_beans"]),
        ("bbq", &["drumsticks", "chicken_breast", "chicken_thighs", "pork_chops", "cornbread", "baked_beans", "coleslaw", "mashed_potatoes"]),
    ]),
    ingredient!("ratatouille", "Ratatouille", "veg", &[
        ("mediterranean", &["chicken_breast", "chicken_thighs", "salmon", "shrimp", "sausage", "couscous", "quinoa", "garlic_bread"]),
    ]),
    ingredient!("caesar_salad", "Caesar Salad", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "shrimp", "salmon", "garlic_bread", "pasta", "bread_rolls"]),
        ("mediterranean", &["chicken_breast", "chicken_thighs", "shrimp", "salmon", "pita", "couscous", "roasted_tomatoes", "cucumber_tomato"]),
    ]),
    ingredient!("garden_salad", "Garden Salad", "veg", &[
        ("american", &["chicken_breast", "chicken_thighs", "steak", "salmon", "shrimp", "bread_rolls", "roasted_potatoes", "quinoa"]),
        ("latin", &["chicken_breast", "steak", "shrimp", "tortillas", "cilantro_lime_rice", "avocado_salad", "corn", "black_beans_rice"]),
        ("mediterranean", &["chicken_breast", "salmon", "shrimp", "tuna", "pita", "couscous", "cucumber_tomato", "greek_salad"]),
    ]),
];

pub fn get_proteins() -> Vec<&'static Ingredient> {
    INGREDIENTS.iter().filter(|i| i.category == "protein").collect()
}

pub fn get_starches() -> Vec<&'static Ingredient> {
    INGREDIENTS.iter().filter(|i| i.category == "starch").collect()
}

pub fn get_vegs() -> Vec<&'static Ingredient> {
    INGREDIENTS.iter().filter(|i| i.category == "veg").collect()
}
