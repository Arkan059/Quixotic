use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct BuildingDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub base_cost: f64,
    pub growth: f64,
    pub base_rate: f64,
}

const NAMES: [&str; 26] = [
    "Windmill",
    "Squire Sancho",
    "Rusty Lance",
    "Trusty Steed",
    "Imaginary Castle",
    "Dulcinea's Letter",
    "Loyal Sheep",
    "Barber's Basin",
    "Knightly Oath",
    "Dapple Donkey",
    "Candle of Chivalry",
    "Windmill Fan Club",
    "Errant Quest Board",
    "Chivalry Manual",
    "Enchanted Windmill",
    "Squire School",
    "Dulcinea Fan Mail",
    "Castle in the Clouds",
    "Giant Windmill",
    "Order of the Woeful",
    "Knight of Lions",
    "Golden Helmet",
    "Island of Barataria",
    "Flying Wooden Horse",
    "Starry Windmill",
    "Cosmic Windmill",
];

const IDS: [&str; 26] = [
    "windmill",
    "squire_sancho",
    "rusty_lance",
    "trusty_steed",
    "imaginary_castle",
    "dulcinea_letter",
    "loyal_sheep",
    "barbers_basin",
    "knightly_oath",
    "dapple_donkey",
    "candle_of_chivalry",
    "windmill_fan_club",
    "errant_quest_board",
    "chivalry_manual",
    "enchanted_windmill",
    "squire_school",
    "dulcinea_fan_mail",
    "castle_in_the_clouds",
    "giant_windmill",
    "order_of_the_woeful",
    "knight_of_lions",
    "golden_helmet",
    "barataria_island",
    "flying_wooden_horse",
    "starry_windmill",
    "cosmic_windmill",
];

const DESCRIPTIONS: [&str; 26] = [
    "A perfectly ordinary windmill. I have never once mistaken it for a giant. Probably.",
    "My loyal squire brings snacks, warnings, and occasional common sense. Mostly snacks.",
    "The classic lance: lightly rusted, heavily heroic, and excellent for pointing at things.",
    "A noble steed of exceptional character. Please do not ask about the smell.",
    "Its towers are made of hope and its moat is a very real puddle. Majestic.",
    "Her words are worth a thousand gold. I read them aloud to everyone nearby.",
    "Baa. A most chivalrous flock. They follow me into battle and into the hay.",
    "A helmet of unmatched shine! The barber insists it is for shaving. Nonsense.",
    "A solemn vow repeated daily. Affection grows every time I announce it to the sky.",
    "Sancho's trusty donkey. It carries the snacks and judges absolutely no one.",
    "A tiny flame that refuses to go out, even in a stiff breeze or a logical argument.",
    "Our first meeting was a disaster. Our second meeting had much better snacks.",
    "Quest one: pat the knight. Quest two: pat the knight again. Payment: joy.",
    "Chapter one: be brave. Chapter two: be kind. Chapter three: do not fight furniture.",
    "This one talks. It mostly asks for more wind and occasional compliments.",
    "A whole classroom of tiny Sanchos, all saluting at once. Extremely efficient.",
    "The letters arrive in bundles, each scented with roses and questionable poetry.",
    "Difficult to reach, impossible to clean, and absolutely worth the climb.",
    "Now THIS one is definitely a giant. I have measured its shadow twice.",
    "Membership card included. The sad face is part of the uniform. Mine is radiant.",
    "The lions were very brave. I was also very brave about the lions.",
    "A helmet so golden that birds attempt to pay rent on it.",
    "A whole island devoted to good governance and excellent snacks. Sancho runs it.",
    "No wings, no engine, no problem. Belief is a perfectly valid fuel source.",
    "It turns constellations into flour. The stars are surprisingly cooperative.",
    "The final giant. It spins the universe itself, and every turn produces a gentle pat.",
];

pub fn building_defs() -> &'static [BuildingDef] {
    static DEFS: OnceLock<Vec<BuildingDef>> = OnceLock::new();
    DEFS.get_or_init(|| {
        IDS.iter()
            .enumerate()
            .map(|(index, id)| {
                let tier = index as f64;
                BuildingDef {
                    id,
                    name: NAMES[index],
                    description: DESCRIPTIONS[index],
                    base_cost: 15.0 * 8.4f64.powf(tier),
                    growth: 1.15,
                    base_rate: 0.1 * 7.4f64.powf(tier),
                }
            })
            .collect()
    })
    .as_slice()
}

pub fn building_index(id: &str) -> Option<usize> {
    building_defs()
        .iter()
        .position(|building| building.id == id)
}

pub fn next_cost(def: &BuildingDef, owned: u64) -> f64 {
    def.base_cost * def.growth.powf(owned as f64)
}

pub fn bulk_cost(def: &BuildingDef, owned: u64, amount: u64) -> f64 {
    if amount == 0 {
        return 0.0;
    }
    let growth = def.growth;
    def.base_cost * growth.powf(owned as f64) * (growth.powf(amount as f64) - 1.0) / (growth - 1.0)
}

pub fn max_affordable(def: &BuildingDef, owned: u64, currency: f64) -> u64 {
    const LIMIT: u64 = 1_000_000;
    if currency < next_cost(def, owned) {
        return 0;
    }
    let numerator =
        1.0 + currency * (def.growth - 1.0) / (def.base_cost * def.growth.powf(owned as f64));
    let mut amount = if numerator > 0.0 {
        (numerator.ln() / def.growth.ln()).floor() as u64
    } else {
        0
    };
    amount = amount.clamp(0, LIMIT);
    let mut guard = 0;
    while amount > 0 && bulk_cost(def, owned, amount) > currency && guard < 128 {
        amount -= 1;
        guard += 1;
    }
    guard = 0;
    while amount < LIMIT && bulk_cost(def, owned, amount + 1) <= currency && guard < 128 {
        amount += 1;
        guard += 1;
    }
    amount
}

pub fn purchase_cost(def: &BuildingDef, owned: u64, requested: u64, currency: f64) -> (u64, f64) {
    if requested == u64::MAX || requested == 0 {
        let amount = max_affordable(def, owned, currency);
        (amount, bulk_cost(def, owned, amount))
    } else {
        (requested, bulk_cost(def, owned, requested))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn building_table_is_complete() {
        let defs = building_defs();
        assert_eq!(defs.len(), 26);
        let mut ids = BTreeSet::new();
        for def in defs {
            assert!(ids.insert(def.id));
            assert!((def.growth - 1.15).abs() < 1e-9);
            assert!(def.base_cost.is_finite() && def.base_cost > 0.0);
            assert!(def.base_rate.is_finite() && def.base_rate > 0.0);
        }
    }

    #[test]
    fn bulk_costs_and_affordability_agree() {
        let def = &building_defs()[0];
        let one = next_cost(def, 0);
        assert!((bulk_cost(def, 0, 1) - one).abs() < 1e-6);
        assert_eq!(max_affordable(def, 0, one - 1.0), 0);
        assert!(max_affordable(def, 0, one) >= 1);
        let amount = max_affordable(def, 0, one * 100.0);
        assert!(bulk_cost(def, 0, amount) <= one * 100.0);
        assert!(bulk_cost(def, 0, amount + 1) > one * 100.0);
    }
}
