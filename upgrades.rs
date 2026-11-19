use std::sync::OnceLock;

use crate::buildings::building_defs;
use crate::format;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpgradeCategory {
    ClickPower,
    ClickMultiplier,
    BuildingBoost,
    Global,
    Special,
}

impl UpgradeCategory {
    pub fn label(self) -> &'static str {
        match self {
            UpgradeCategory::ClickPower => "CLICK POWER",
            UpgradeCategory::ClickMultiplier => "CLICK MULTIPLIER",
            UpgradeCategory::BuildingBoost => "BUILDING BOOST",
            UpgradeCategory::Global => "GLOBAL",
            UpgradeCategory::Special => "QUIXOTIC SPECIAL",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            UpgradeCategory::ClickPower => "CLICK",
            UpgradeCategory::ClickMultiplier => "MULT",
            UpgradeCategory::BuildingBoost => "BUILD",
            UpgradeCategory::Global => "GLOBAL",
            UpgradeCategory::Special => "SPECIAL",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnlockReq {
    Always,
    TotalClicks(u64),
    TotalEarned(f64),
    TotalBuildings(u64),
    Building(usize, u64),
    Upgrades(u64),
    Achievements(u64),
}

#[derive(Clone, Copy, Debug)]
pub enum UpgradeEffect {
    ClickFlat(f64),
    ClickMultiplier(f64),
    ProductionMultiplier(f64),
    GlobalMultiplier(f64),
    BuildingMultiplier(usize, f64),
    ClickProductionShare(f64),
    AutoPatRate(f64),
    OfflineBonus(f64),
}

#[derive(Clone, Debug)]
pub struct UpgradeDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: UpgradeCategory,
    pub chain_index: usize,
    pub cost: f64,
    pub effect: UpgradeEffect,
    pub unlock: UnlockReq,
}

pub struct UnlockContext<'a> {
    pub total_clicks: u64,
    pub total_earned: f64,
    pub total_buildings: u64,
    pub upgrades_bought: u64,
    pub achievements_unlocked: u64,
    pub building_counts: &'a [u64],
}

const CLICK_POWER_NAMES: [&str; 14] = [
    "A Gentle Pat",
    "A Determined Pat",
    "The Two-Handed Pat",
    "Pat of the Woeful Countenance",
    "Knightly Knuckle",
    "Squire-Approved Pat",
    "Helmet-Polished Pat",
    "Windmill-Wrestling Pat",
    "Lance-Accurate Pat",
    "Dulcinea-Devoted Pat",
    "Unstoppable Patting Technique",
    "Order of the Open Palm",
    "Legendary Squire Slap",
    "The Impossibly Tender Blow",
];

const CLICK_POWER_DESCRIPTIONS: [&str; 14] = [
    "A pat so gentle that even the windmill leans closer.",
    "You mean it this time. Both hands, and a small bow afterward.",
    "A two-handed technique passed down by extremely enthusiastic squires.",
    "Delivered with the full sorrow and dignity of a woeful knight.",
    "Knuckles are for knocking on doors, and occasionally for patting.",
    "Sancho approved this method. The form was signed with a hoofprint.",
    "Polish the helmet first, then pat the knight. Priorities matter.",
    "Wrestle a windmill, then pat a hero. A balanced training regimen.",
    "Aim true, strike softly, and never poke anyone with the lance.",
    "Every pat now includes a tiny imagined serenade from Dulcinea.",
    "A wholly original technique involving confidence, kindness, and three pats at once.",
    "The order gathers to observe the finest patting in the land.",
    "A legendary slap that somehow feels exactly like a warm hug.",
    "The blow is impossible to land, yet affection appears anyway. Heroic nonsense!",
];

const CLICK_POWER_UNLOCKS: [u64; 14] = [
    0, 10, 30, 80, 200, 500, 1200, 3000, 8000, 20000, 50000, 120000, 300000, 750000,
];

const CLICK_MULT_NAMES: [&str; 10] = [
    "Soulful Touch",
    "Heart of a Knight",
    "Tender Valor",
    "Ballad-Worthy Pat",
    "Chivalry Overdrive",
    "The Golden Gauntlet",
    "Dulcinea's Handkerchief",
    "Ode to Affection",
    "Heroic Tenderness",
    "The Infinite Pat",
];

const CLICK_MULT_DESCRIPTIONS: [&str; 10] = [
    "A pat that lands directly on the soul. The soul says thank you.",
    "Beneath every helmet is a heart, and mine is extremely receptive.",
    "Valor is loud, but tenderness carries farther. Also it gives more affection.",
    "Bards will write songs about this pat. The songs will be slightly too long.",
    "Chivalry has a speed setting, and that setting is now maximum.",
    "A gauntlet made of gold works surprisingly well for gentle patting.",
    "She lent me a handkerchief. I have framed it. It makes me stronger.",
    "An entire ode in one motion. Please hold your applause until the second pat.",
    "A hero can be fierce, fearless, and also very good at gentle head pats.",
    "The pat repeats forever in the imaginary hall of heroic achievements.",
];

const CLICK_MULT_UNLOCKS: [f64; 10] = [
    100.0,
    1000.0,
    7500.0,
    50000.0,
    300000.0,
    2_000_000.0,
    15_000_000.0,
    100_000_000.0,
    1_000_000_000.0,
    10_000_000_000.0,
];

const GLOBAL_NAMES: [&str; 10] = [
    "Knightly Inspiration",
    "The Power of Friendship",
    "A Thousand Windmills",
    "Squirely Excellence",
    "Tales of the Impossible",
    "Chivalric Resonance",
    "Grand Order of Patting",
    "Legend of the Kind Knight",
    "The Dreaming Giant",
    "Crown of the Woeful",
];

const GLOBAL_DESCRIPTIONS: [&str; 10] = [
    "One heroic speech, delivered to a mirror, improves absolutely everything.",
    "Sancho believes in me. That counts as infrastructure.",
    "Somewhere, a thousand windmills begin to turn in perfect agreement.",
    "Excellence is contagious. Sancho caught it first and now it is everywhere.",
    "The impossible listens to a good story and decides to cooperate.",
    "All my oaths, ballads, and snacks are finally vibrating in harmony.",
    "A grand order devoted to patting, heroism, and proper snack distribution.",
    "Kindness is the mightiest blade. Also it has excellent long-term returns.",
    "The giant sleeps in the hills, and its dreams smell faintly of affection.",
    "A crown woven from windmill grass. Surprisingly comfortable. Extremely regal.",
];

const GLOBAL_UNLOCK_UPGRADES: [u64; 10] = [0, 2, 5, 9, 15, 25, 38, 50, 60, 70];

const SPECIAL_NAMES: [&str; 10] = [
    "Squire's Helping Hand",
    "A Thousand Enthusiastic Squires",
    "A Storm of Tiny Hands",
    "Dulcinea's Favor",
    "Windmill Windfall",
    "Barber's Brilliant Basin",
    "Golden Age of Chivalry",
    "A Legend in My Own Mind",
    "Impossible Dream Dividend",
    "The Final, Glorious Pat",
];

const SPECIAL_DESCRIPTIONS: [&str; 10] = [
    "Sancho pats my shoulder on a reliable schedule. Extremely professional.",
    "An entire crowd of imaginary squires has learned the gentle art of patting.",
    "A swirling storm of tiny hands, all here to support the cause.",
    "Dulcinea smiled in a letter. The letter is now part of my permanent strength.",
    "Every windmill in the land owes me a favor, and favors can be turned into affection.",
    "The barber's basin is now a sacred relic. The barber has stopped asking questions.",
    "Good deeds, brave knights, and well-fed squires everywhere are doing their part.",
    "Who needs a chronicler when one's imagination is this reliable?",
    "Eight hours of questing worth of affection is now remembered even while I nap.",
    "The last pat. The greatest pat. The one that makes the universe feel a little softer.",
];

const SPECIAL_UNLOCK_ACHIEVEMENTS: [u64; 10] = [1, 5, 12, 0, 0, 25, 40, 0, 0, 60];

fn make_boost_descriptions() -> Vec<String> {
    let mut result = Vec::with_capacity(26);
    let mut index = 0;
    for building in building_defs() {
        let text = match index % 3 {
            0 => format!(
                "I gave the {} a medal and a speech. It now works twice as hard!",
                building.name
            ),
            1 => format!(
                "The {} has sworn a sacred oath. Its output doubled on the spot.",
                building.name
            ),
            _ => format!(
                "Sancho polished the {}. Miraculously, it now produces twice as much.",
                building.name
            ),
        };
        result.push(text);
        index += 1;
    }
    result
}

fn build_upgrades() -> Vec<UpgradeDef> {
    let mut upgrades = Vec::with_capacity(70);
    for index in 0..CLICK_POWER_NAMES.len() {
        let tier = index as f64;
        upgrades.push(UpgradeDef {
            id: format!("click_{:02}", index),
            name: CLICK_POWER_NAMES[index].to_string(),
            description: CLICK_POWER_DESCRIPTIONS[index].to_string(),
            category: UpgradeCategory::ClickPower,
            chain_index: index,
            cost: 10.0 * 3.2f64.powf(tier),
            effect: UpgradeEffect::ClickFlat(1.0 * 1.8f64.powf(tier)),
            unlock: if index == 0 {
                UnlockReq::Always
            } else {
                UnlockReq::TotalClicks(CLICK_POWER_UNLOCKS[index])
            },
        });
    }
    for index in 0..CLICK_MULT_NAMES.len() {
        let tier = index as f64;
        upgrades.push(UpgradeDef {
            id: format!("mult_{:02}", index),
            name: CLICK_MULT_NAMES[index].to_string(),
            description: CLICK_MULT_DESCRIPTIONS[index].to_string(),
            category: UpgradeCategory::ClickMultiplier,
            chain_index: index,
            cost: 750.0 * 9.0f64.powf(tier),
            effect: UpgradeEffect::ClickMultiplier(2.0),
            unlock: UnlockReq::TotalEarned(CLICK_MULT_UNLOCKS[index]),
        });
    }
    let boost_descriptions = make_boost_descriptions();
    for (index, building) in building_defs().iter().enumerate() {
        upgrades.push(UpgradeDef {
            id: format!("boost_{:02}", index),
            name: format!("{} Appreciation", building.name),
            description: boost_descriptions[index].clone(),
            category: UpgradeCategory::BuildingBoost,
            chain_index: index,
            cost: building.base_cost * 4.5,
            effect: UpgradeEffect::BuildingMultiplier(index, 2.0),
            unlock: UnlockReq::Building(index, 1),
        });
    }
    for index in 0..GLOBAL_NAMES.len() {
        upgrades.push(UpgradeDef {
            id: format!("global_{:02}", index),
            name: GLOBAL_NAMES[index].to_string(),
            description: GLOBAL_DESCRIPTIONS[index].to_string(),
            category: UpgradeCategory::Global,
            chain_index: index,
            cost: 25_000.0 * 10.0f64.powi(index as i32),
            effect: UpgradeEffect::GlobalMultiplier(1.5),
            unlock: UnlockReq::Upgrades(GLOBAL_UNLOCK_UPGRADES[index]),
        });
    }
    for index in 0..SPECIAL_NAMES.len() {
        let effect = match index {
            0 => UpgradeEffect::AutoPatRate(1.0),
            1 => UpgradeEffect::AutoPatRate(8.0),
            2 => UpgradeEffect::AutoPatRate(40.0),
            3 => UpgradeEffect::ClickProductionShare(0.01),
            4 => UpgradeEffect::ProductionMultiplier(2.5),
            5 => UpgradeEffect::GlobalMultiplier(1.8),
            6 => UpgradeEffect::GlobalMultiplier(2.5),
            7 => UpgradeEffect::ClickMultiplier(5.0),
            8 => UpgradeEffect::OfflineBonus(0.25),
            _ => UpgradeEffect::GlobalMultiplier(5.0),
        };
        let unlock = match index {
            0 => UnlockReq::Achievements(1),
            1 => UnlockReq::Achievements(5),
            2 => UnlockReq::Achievements(12),
            3 => UnlockReq::TotalEarned(25_000.0),
            4 => UnlockReq::TotalBuildings(30),
            5 => UnlockReq::Achievements(25),
            6 => UnlockReq::Achievements(40),
            7 => UnlockReq::TotalClicks(50_000),
            8 => UnlockReq::TotalEarned(1_000_000.0),
            _ => UnlockReq::Achievements(SPECIAL_UNLOCK_ACHIEVEMENTS[9]),
        };
        upgrades.push(UpgradeDef {
            id: format!("special_{:02}", index),
            name: SPECIAL_NAMES[index].to_string(),
            description: SPECIAL_DESCRIPTIONS[index].to_string(),
            category: UpgradeCategory::Special,
            chain_index: index,
            cost: 2_500.0 * 18.0f64.powi(index as i32),
            effect,
            unlock,
        });
    }
    upgrades
}

pub fn upgrade_defs() -> &'static [UpgradeDef] {
    static DEFS: OnceLock<Vec<UpgradeDef>> = OnceLock::new();
    DEFS.get_or_init(build_upgrades).as_slice()
}

pub fn upgrade_index(id: &str) -> Option<usize> {
    upgrade_defs().iter().position(|upgrade| upgrade.id == id)
}

pub fn is_unlocked(def: &UpgradeDef, context: &UnlockContext) -> bool {
    match def.unlock {
        UnlockReq::Always => true,
        UnlockReq::TotalClicks(required) => context.total_clicks >= required,
        UnlockReq::TotalEarned(required) => context.total_earned >= required,
        UnlockReq::TotalBuildings(required) => context.total_buildings >= required,
        UnlockReq::Building(index, required) => {
            context.building_counts.get(index).copied().unwrap_or(0) >= required
        }
        UnlockReq::Upgrades(required) => context.upgrades_bought >= required,
        UnlockReq::Achievements(required) => context.achievements_unlocked >= required,
    }
}

pub fn unlock_text(def: &UpgradeDef) -> String {
    match def.unlock {
        UnlockReq::Always => "No requirement".to_string(),
        UnlockReq::TotalClicks(required) => {
            format!("Pat her {} times", format::number(required as f64))
        }
        UnlockReq::TotalEarned(required) => format!("Earn {} affection", format::number(required)),
        UnlockReq::TotalBuildings(required) => {
            format!("Own {} buildings", format::number(required as f64))
        }
        UnlockReq::Building(index, required) => {
            let name = building_defs()
                .get(index)
                .map(|building| building.name)
                .unwrap_or("building");
            format!("Own {} {}", format::number(required as f64), name)
        }
        UnlockReq::Upgrades(required) => {
            format!("Buy {} upgrades", format::number(required as f64))
        }
        UnlockReq::Achievements(required) => {
            format!("Unlock {} awards", format::number(required as f64))
        }
    }
}

pub fn effect_text(def: &UpgradeDef) -> String {
    match def.effect {
        UpgradeEffect::ClickFlat(amount) => format!("+{} per pat", format::number(amount)),
        UpgradeEffect::ClickMultiplier(amount) => format!("Pat power x{}", format::number(amount)),
        UpgradeEffect::ProductionMultiplier(amount) => {
            format!("All buildings x{}", format::number(amount))
        }
        UpgradeEffect::GlobalMultiplier(amount) => {
            format!("Everything x{}", format::number(amount))
        }
        UpgradeEffect::BuildingMultiplier(index, amount) => {
            let name = building_defs()
                .get(index)
                .map(|building| building.name)
                .unwrap_or("Building");
            format!("{} output x{}", name, format::number(amount))
        }
        UpgradeEffect::ClickProductionShare(amount) => {
            format!("+{} of production per pat", format::percent(amount))
        }
        UpgradeEffect::AutoPatRate(amount) => {
            format!("+{}/s automatic patting", format::number(amount))
        }
        UpgradeEffect::OfflineBonus(amount) => {
            format!("+{} offline affection", format::percent(amount))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn upgrade_table_is_complete() {
        let defs = upgrade_defs();
        assert!(defs.len() >= 50);
        let mut ids = BTreeSet::new();
        let mut chains: BTreeMap<UpgradeCategory, Vec<usize>> = BTreeMap::new();
        for def in defs {
            assert!(ids.insert(def.id.clone()));
            assert!(def.cost.is_finite() && def.cost > 0.0);
            assert!(!def.name.is_empty());
            assert!(!def.description.is_empty());
            chains
                .entry(def.category)
                .or_default()
                .push(def.chain_index);
        }
        assert_eq!(ids.len(), defs.len());
        for indexes in chains.values_mut() {
            indexes.sort_unstable();
            for (expected, actual) in indexes.iter().enumerate() {
                assert_eq!(expected, *actual);
            }
        }
    }
}
