use std::sync::OnceLock;

use crate::buildings::building_defs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AchievementCategory {
    Clicking,
    Wealth,
    Buildings,
    Upgrades,
    Speed,
    Time,
    Offline,
    Secrets,
}

impl AchievementCategory {
    pub fn label(self) -> &'static str {
        match self {
            AchievementCategory::Clicking => "PATTING",
            AchievementCategory::Wealth => "AFFECTION",
            AchievementCategory::Buildings => "BUILDINGS",
            AchievementCategory::Upgrades => "UPGRADES",
            AchievementCategory::Speed => "PAT SPEED",
            AchievementCategory::Time => "PLAYTIME",
            AchievementCategory::Offline => "REST",
            AchievementCategory::Secrets => "SECRET",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AchievementCondition {
    TotalClicks(u64),
    TotalEarned(f64),
    BuildingOwned(usize, u64),
    TotalBuildings(u64),
    UpgradesBought(u64),
    PeakCps(f64),
    Playtime(f64),
    OfflineSeconds(f64),
    ManualSaves(u64),
    OfflineEarned(f64),
    BestBulkPurchase(u64),
}

#[derive(Clone, Debug)]
pub struct AchievementDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub condition: AchievementCondition,
    pub hidden: bool,
}

pub struct AchievementContext<'a> {
    pub total_clicks: u64,
    pub total_earned: f64,
    pub building_counts: &'a [u64],
    pub total_buildings: u64,
    pub upgrades_bought: u64,
    pub peak_cps: f64,
    pub playtime: f64,
    pub offline_seconds: f64,
    pub manual_saves: u64,
    pub offline_earned: f64,
    pub best_bulk_purchase: u64,
}

const CLICK_NAMES: [&str; 8] = [
    "First Contact",
    "A Promising Squire",
    "Dedicated Patter",
    "Knight of the Open Palm",
    "The Gentle Tempest",
    "Hero of Hands",
    "Legendary Patting Master",
    "The Impossible Pat",
];

const CLICK_DESCRIPTIONS: [&str; 8] = [
    "Pat Don Quixote once. Every great quest begins with a single pat.",
    "Pat her 25 times. She has already composed a ballad about you.",
    "Pat her 100 times. Sancho is taking notes for the official chronicle.",
    "Pat her 500 times. Your palm has been knighted by sheer repetition.",
    "Pat her 2,500 times. The windmills are starting to feel left out.",
    "Pat her 10,000 times. This is no longer a hobby. This is destiny.",
    "Pat her 50,000 times. The chronicles require a bigger shelf.",
    "Pat her 250,000 times. Even the impossible is a little impressed.",
];

const CLICK_THRESHOLDS: [u64; 8] = [1, 25, 100, 500, 2500, 10000, 50000, 250000];

const EARNED_NAMES: [&str; 10] = [
    "A Modest Treasure",
    "Pocketful of Dreams",
    "Chivalry Pays",
    "Squire's Retirement Fund",
    "A Very Real Castle",
    "Dulcinea's Dowry",
    "Windmill Baron",
    "Golden Age",
    "The Invisible Empire",
    "A Fortune of Dreams",
];

const EARNED_DESCRIPTIONS: [&str; 10] = [
    "Earn 100 affection. Enough for a snack, or one very sincere compliment.",
    "Earn 1,000 affection. The imaginary treasury is coming along nicely.",
    "Earn 10,000 affection. Good deeds do pay, especially with excellent accounting.",
    "Earn 100,000 affection. Sancho suggests a modest retirement. Nonsense.",
    "Earn 1,000,000 affection. The castle is now legally imaginary and fully paid for.",
    "Earn 10,000,000 affection. A dowry of pure admiration and excellent handwriting.",
    "Earn 100,000,000 affection. Every windmill salutes when you pass.",
    "Earn 1,000,000,000 affection. Historians will call this the affectionate age.",
    "Earn 1,000,000,000,000 affection. The empire is invisible and quite comfortable.",
    "Earn 1,000,000,000,000,000 affection. A fortune built entirely on being patted.",
];

const EARNED_THRESHOLDS: [f64; 10] = [
    100.0,
    1_000.0,
    10_000.0,
    100_000.0,
    1_000_000.0,
    10_000_000.0,
    100_000_000.0,
    1_000_000_000.0,
    1_000_000_000_000.0,
    1_000_000_000_000_000.0,
];

const BUILDING_NAMES: [&str; 26] = [
    "Windmill Whispers",
    "Sancho's Approval",
    "A Lance Among Friends",
    "Stable Beginnings",
    "Castle in the Mind",
    "Letter of Importance",
    "Sheepish Devotion",
    "Basin of Bravery",
    "Oathbound",
    "Donkey Diplomacy",
    "A Candle for Courage",
    "Club Membership",
    "Quest Accepted",
    "By the Book",
    "A Chatty Windmill",
    "Squire School Spirit",
    "Postmarked Dulcinea",
    "Cloud Nine",
    "Giant Evidence",
    "Woeful Membership",
    "Lionhearted",
    "Golden Shine",
    "Island Governance",
    "Horsepower of Belief",
    "Constellation Flour",
    "Cosmic Pat",
];

const TOTAL_BUILDING_NAMES: [&str; 8] = [
    "A Humble Estate",
    "Small Kingdom, Big Heart",
    "A Proper Realm",
    "Urban Chivalry",
    "Windmill City",
    "Industrial-sized Honor",
    "A Kingdom of Whirring",
    "The Infinite Manor",
];

const TOTAL_BUILDING_DESCRIPTIONS: [&str; 8] = [
    "Own 1 building. Every kingdom starts with a questionable investment.",
    "Own 10 buildings. The neighbors are beginning to ask questions.",
    "Own 25 buildings. Sancho has prepared a very long to-do list.",
    "Own 50 buildings. The quest now requires proper roads and a map.",
    "Own 100 buildings. The skyline is becoming delightfully improbable.",
    "Own 250 buildings. Chivalry has never been this well-staffed.",
    "Own 500 buildings. The windmills have formed a committee.",
    "Own 1,000 buildings. You have made imagination into infrastructure.",
];

const TOTAL_BUILDING_THRESHOLDS: [u64; 8] = [1, 10, 25, 50, 100, 250, 500, 1000];

const UPGRADE_NAMES: [&str; 7] = [
    "First Improvement",
    "Tuned for Adventure",
    "A Well-Equipped Knight",
    "Doctrine of Patting",
    "Walking Arsenal of Kindness",
    "A Knight of Many Talents",
    "The Complete Quixote",
];

const UPGRADE_DESCRIPTIONS: [&str; 7] = [
    "Buy 1 upgrade. An old helmet needs the occasional polish.",
    "Buy 5 upgrades. The quest is now slightly more efficient and much more stylish.",
    "Buy 10 upgrades. Every buckle has a purpose and every purpose is noble.",
    "Buy 20 upgrades. The sacred art of patting has developed a curriculum.",
    "Buy 35 upgrades. Kindness is now a loadout, and it is fully optimized.",
    "Buy 50 upgrades. Armor, etiquette, ballads, and snacks all in one knight.",
    "Buy all 70 upgrades. The complete knight. The final form of affectionate heroism.",
];

const UPGRADE_THRESHOLDS: [u64; 7] = [1, 5, 10, 20, 35, 50, 70];

const SPEED_NAMES: [&str; 5] = [
    "Quick Paws",
    "Squire Speed",
    "Finger of the Wind",
    "The Blur of Kindness",
    "Unstoppable Affection",
];

const SPEED_DESCRIPTIONS: [&str; 5] = [
    "Reach 2 clicks per second. Sancho approves of this brisk technique.",
    "Reach 4 clicks per second. The lance salute is getting hasty.",
    "Reach 8 clicks per second. Your hand moves faster than the windmill.",
    "Reach 12 clicks per second. The plush is nearly airborne.",
    "Reach 16 clicks per second. Even the impossible says take a breath.",
];

const SPEED_THRESHOLDS: [f64; 5] = [2.0, 4.0, 8.0, 12.0, 16.0];

const TIME_NAMES: [&str; 5] = [
    "A Short Quest",
    "A Good Long Sit",
    "The Afternoon Adventure",
    "Dedicated Squire",
    "A Day of Chivalry",
];

const TIME_DESCRIPTIONS: [&str; 5] = [
    "Play for 1 minute. Enough time for one heroic opening speech.",
    "Play for 10 minutes. The quest is officially underway.",
    "Play for 1 hour. A knight must pace herself, and so must a dedicated patter.",
    "Play for 5 hours. The chronicle is now several chapters long.",
    "Play for 24 hours. Even imaginary heroes need a water break.",
];

const TIME_THRESHOLDS: [f64; 5] = [60.0, 600.0, 3600.0, 18000.0, 86400.0];

fn building_achievement_description(index: usize, name: &str) -> String {
    match index % 3 {
        0 => format!(
            "Own your first {}. It has already started practicing a heroic pose.",
            name
        ),
        1 => format!(
            "One {} is enough to make any imaginary kingdom feel official.",
            name
        ),
        _ => format!(
            "The {} is here! Sancho wrote it down in the important ledger of important things.",
            name
        ),
    }
}

fn build_achievements() -> Vec<AchievementDef> {
    let mut achievements = Vec::with_capacity(75);
    for index in 0..CLICK_NAMES.len() {
        achievements.push(AchievementDef {
            id: format!("clicks_{:02}", index),
            name: CLICK_NAMES[index].to_string(),
            description: CLICK_DESCRIPTIONS[index].to_string(),
            category: AchievementCategory::Clicking,
            condition: AchievementCondition::TotalClicks(CLICK_THRESHOLDS[index]),
            hidden: false,
        });
    }
    for index in 0..EARNED_NAMES.len() {
        achievements.push(AchievementDef {
            id: format!("earned_{:02}", index),
            name: EARNED_NAMES[index].to_string(),
            description: EARNED_DESCRIPTIONS[index].to_string(),
            category: AchievementCategory::Wealth,
            condition: AchievementCondition::TotalEarned(EARNED_THRESHOLDS[index]),
            hidden: false,
        });
    }
    for (index, building) in building_defs().iter().enumerate() {
        achievements.push(AchievementDef {
            id: format!("own_{}", building.id),
            name: BUILDING_NAMES[index].to_string(),
            description: building_achievement_description(index, building.name),
            category: AchievementCategory::Buildings,
            condition: AchievementCondition::BuildingOwned(index, 1),
            hidden: false,
        });
    }
    for index in 0..TOTAL_BUILDING_NAMES.len() {
        achievements.push(AchievementDef {
            id: format!("total_buildings_{:02}", index),
            name: TOTAL_BUILDING_NAMES[index].to_string(),
            description: TOTAL_BUILDING_DESCRIPTIONS[index].to_string(),
            category: AchievementCategory::Buildings,
            condition: AchievementCondition::TotalBuildings(TOTAL_BUILDING_THRESHOLDS[index]),
            hidden: false,
        });
    }
    for index in 0..UPGRADE_NAMES.len() {
        achievements.push(AchievementDef {
            id: format!("upgrades_{:02}", index),
            name: UPGRADE_NAMES[index].to_string(),
            description: UPGRADE_DESCRIPTIONS[index].to_string(),
            category: AchievementCategory::Upgrades,
            condition: AchievementCondition::UpgradesBought(UPGRADE_THRESHOLDS[index]),
            hidden: false,
        });
    }
    for index in 0..SPEED_NAMES.len() {
        achievements.push(AchievementDef {
            id: format!("speed_{:02}", index),
            name: SPEED_NAMES[index].to_string(),
            description: SPEED_DESCRIPTIONS[index].to_string(),
            category: AchievementCategory::Speed,
            condition: AchievementCondition::PeakCps(SPEED_THRESHOLDS[index]),
            hidden: false,
        });
    }
    achievements.push(AchievementDef {
        id: "secret_speed".to_string(),
        name: "Finger of the Hurricane".to_string(),
        description: "Reach 25 clicks per second. The windmill is taking notes.".to_string(),
        category: AchievementCategory::Speed,
        condition: AchievementCondition::PeakCps(25.0),
        hidden: true,
    });
    for index in 0..TIME_NAMES.len() {
        achievements.push(AchievementDef {
            id: format!("playtime_{:02}", index),
            name: TIME_NAMES[index].to_string(),
            description: TIME_DESCRIPTIONS[index].to_string(),
            category: AchievementCategory::Time,
            condition: AchievementCondition::Playtime(TIME_THRESHOLDS[index]),
            hidden: false,
        });
    }
    achievements.push(AchievementDef {
        id: "away_short".to_string(),
        name: "A Brief Nap".to_string(),
        description: "Return after being away for at least 5 minutes. Heroes need beauty sleep."
            .to_string(),
        category: AchievementCategory::Offline,
        condition: AchievementCondition::OfflineSeconds(300.0),
        hidden: false,
    });
    achievements.push(AchievementDef {
        id: "away_long".to_string(),
        name: "A Hero's Rest".to_string(),
        description: "Return after being away for at least 1 hour. The quest kept itself tidy."
            .to_string(),
        category: AchievementCategory::Offline,
        condition: AchievementCondition::OfflineSeconds(3600.0),
        hidden: false,
    });
    achievements.push(AchievementDef {
        id: "secret_saves".to_string(),
        name: "The Pen Is Mighty".to_string(),
        description: "Save the game 3 times. A knight must record her deeds before napping."
            .to_string(),
        category: AchievementCategory::Secrets,
        condition: AchievementCondition::ManualSaves(3),
        hidden: true,
    });
    achievements.push(AchievementDef {
        id: "secret_bulk".to_string(),
        name: "Wholesale Chivalry".to_string(),
        description: "Buy 100 buildings in one purchase. Sancho needs a much bigger cart."
            .to_string(),
        category: AchievementCategory::Secrets,
        condition: AchievementCondition::BestBulkPurchase(100),
        hidden: true,
    });
    achievements.push(AchievementDef {
        id: "secret_offline".to_string(),
        name: "Sweet Dreams, Knight".to_string(),
        description: "Collect 100,000 affection while away. Even a nap can be heroic.".to_string(),
        category: AchievementCategory::Secrets,
        condition: AchievementCondition::OfflineEarned(100_000.0),
        hidden: true,
    });
    achievements
}

pub fn achievement_defs() -> &'static [AchievementDef] {
    static DEFS: OnceLock<Vec<AchievementDef>> = OnceLock::new();
    DEFS.get_or_init(build_achievements).as_slice()
}

pub fn achievement_index(id: &str) -> Option<usize> {
    achievement_defs()
        .iter()
        .position(|achievement| achievement.id == id)
}

pub fn achievement_bonus(unlocked: u64) -> f64 {
    1.0 + unlocked as f64 * 0.005
}

pub fn progress(condition: AchievementCondition, context: &AchievementContext) -> (f64, f64) {
    match condition {
        AchievementCondition::TotalClicks(target) => (context.total_clicks as f64, target as f64),
        AchievementCondition::TotalEarned(target) => (context.total_earned, target),
        AchievementCondition::BuildingOwned(index, target) => (
            context.building_counts.get(index).copied().unwrap_or(0) as f64,
            target as f64,
        ),
        AchievementCondition::TotalBuildings(target) => {
            (context.total_buildings as f64, target as f64)
        }
        AchievementCondition::UpgradesBought(target) => {
            (context.upgrades_bought as f64, target as f64)
        }
        AchievementCondition::PeakCps(target) => (context.peak_cps, target),
        AchievementCondition::Playtime(target) => (context.playtime, target),
        AchievementCondition::OfflineSeconds(target) => (context.offline_seconds, target),
        AchievementCondition::ManualSaves(target) => (context.manual_saves as f64, target as f64),
        AchievementCondition::OfflineEarned(target) => (context.offline_earned, target),
        AchievementCondition::BestBulkPurchase(target) => {
            (context.best_bulk_purchase as f64, target as f64)
        }
    }
}

pub fn is_met(condition: AchievementCondition, context: &AchievementContext) -> bool {
    let (value, target) = progress(condition, context);
    value >= target
}

pub fn condition_label(condition: AchievementCondition) -> &'static str {
    match condition {
        AchievementCondition::TotalClicks(_) => "PATS",
        AchievementCondition::TotalEarned(_) => "AFFECTION",
        AchievementCondition::BuildingOwned(_, _) => "OWNED",
        AchievementCondition::TotalBuildings(_) => "BUILDINGS",
        AchievementCondition::UpgradesBought(_) => "UPGRADES",
        AchievementCondition::PeakCps(_) => "PEAK PATS",
        AchievementCondition::Playtime(_) => "PLAYTIME",
        AchievementCondition::OfflineSeconds(_) => "TIME AWAY",
        AchievementCondition::ManualSaves(_) => "MANUAL SAVES",
        AchievementCondition::OfflineEarned(_) => "OFFLINE AFFECTION",
        AchievementCondition::BestBulkPurchase(_) => "ONE PURCHASE",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn achievement_table_is_complete() {
        let defs = achievement_defs();
        assert_eq!(defs.len(), 75);
        let mut ids = BTreeSet::new();
        for def in defs {
            assert!(ids.insert(def.id.clone()));
            assert!(!def.name.is_empty());
            assert!(!def.description.is_empty());
        }
        assert_eq!(ids.len(), 75);
        let hidden = defs.iter().filter(|def| def.hidden).count();
        assert!(hidden >= 3);
    }

    #[test]
    fn bonus_uses_five_tenths_of_a_percent_per_award() {
        assert!((achievement_bonus(0) - 1.0).abs() < 1e-9);
        assert!((achievement_bonus(20) - 1.1).abs() < 1e-9);
    }
}
