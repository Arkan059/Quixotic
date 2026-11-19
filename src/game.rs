use std::collections::{BTreeMap, BTreeSet};
use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::Color;

use crate::achievements::{self, AchievementCondition, AchievementContext, AchievementDef};
use crate::audio::{stage_for_progress, AudioSystem};
use crate::buildings::{self, BuildingDef};
use crate::dialogue::DialogueBox;
use crate::format;
use crate::pat_animation::PatAnimation;
use crate::save::{self, SaveData, SaveManager, SaveStats, Settings, SAVE_VERSION};
use crate::ui::UiState;
use crate::upgrades::{self, UnlockContext, UpgradeDef, UpgradeEffect};

const OFFLINE_CAP_SECONDS: f64 = 8.0 * 3600.0;

const IDLE_LINES: [&str; 12] = [
    "The windmill looked at me strangely today. I looked back with great dignity.",
    "Sancho says I should drink water. I told Sancho I am hydrated by glory. The resulting sigh was legendary.",
    "If I am very still, do you think Dulcinea can see how brave I am?",
    "A knight must remain vigilant. Also warm. And ideally near a hand for patting.",
    "I have named the clouds. That one is Sancho Two. Sancho Two owes me a horse.",
    "Some days the quest is grand. Today the quest is a nice quiet sit. Both are noble.",
    "Do not fear the giants, friend. Fear forgetting to pat your local knight.",
    "I practiced my heroic pose for three hours. My arm has gone to sleep with honor.",
    "Every windmill is a giant until proven otherwise in a court of law.",
    "The castle is imaginary, but the affection is extremely real.",
    "I have written a new ballad. It rhymes mostly with itself. It is brilliant.",
    "Sancho hid my helmet again. The official reason is my own good. A mystery!",
];

const CLICK_LINES: [&str; 10] = [
    "Ah! That is the spot! The very spot of destiny!",
    "A pat of true chivalry! I shall remember this for several minutes!",
    "I can feel the affection rustling my helmet. Magnificent!",
    "Yes! Continue! The quest requires exactly this many warm fuzzies!",
    "Sancho, write this down! A most excellent pat has occurred!",
    "Careful, friend. My helmet is newly polished. Both of us are gleaming.",
    "This is the kind of heroism that never makes it into the ballads. A shame.",
    "My heart is full! My helmet is slightly crooked! What a day!",
    "Such kindness! The windmills are jealous. I can tell.",
    "One more pat and I shall declare you an honorary squire of the highest order!",
];

const FAST_CLICK_LINES: [&str; 5] = [
    "Slow down, brave fingers! Even a hero needs a moment to glisten!",
    "Ha! Such speed! The windmills tremble before your gentle hands!",
    "Your hand is a blur of pure affection! Sancho cannot keep up!",
    "Is this a hurricane? No! It is friendship at high speed!",
    "Steady, mighty patter! My helmet is rattling with joy!",
];

const BUILDING_LINES: [&str; 5] = [
    "{name}! A magnificent addition to the realm! Sancho, add it to the map!",
    "We have acquired a {name}! The quest grows ever more comfortable.",
    "Behold the {name}! It is even more heroic than the drawing.",
    "The chronicles welcome the {name}! This is a glorious day for paperwork.",
    "A new {name}! I shall give it a tour of the imaginary grounds.",
];

const UPGRADE_LINES: [&str; 5] = [
    "Aha! {name}! I feel more heroic already. And slightly taller?",
    "The chronicle says {name} is now ours. The chronicle is very wise.",
    "With {name}, not even the windmills can ignore my excellence!",
    "A fine improvement! {name} shall accompany me on every noble errand.",
    "Sancho is impressed. That means {name} is either excellent or very shiny.",
];

const ACHIEVEMENT_LINES: [&str; 4] = [
    "A new title! \"{name}\"! The bards will need a longer chorus!",
    "I have been honored with \"{name}\"! I shall wear it with humility and a small cape.",
    "The chronicler records a new deed: \"{name}\"! Esteemed company indeed!",
    "An award! \"{name}\"! Sancho, fetch my acceptance speech and my second-best helmet.",
];

const SAVE_SUCCESS_LINES: [&str; 3] = [
    "Our deeds are written! The chronicler thanks you. Now, back to the quest!",
    "The sacred notebook has been updated! Future historians will swoon.",
    "Saved! Even the imaginary clerks are celebrating with tiny cups of tea.",
];

pub struct Notice {
    pub text: String,
    pub timer: f32,
    pub error: bool,
}

pub struct Toast {
    pub name: String,
    pub timer: f32,
    pub duration: f32,
}

pub struct Floater {
    pub x: f32,
    pub y: f32,
    pub velocity_y: f32,
    pub life: f32,
    pub max_life: f32,
    pub text: String,
}

pub struct HeartParticle {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub life: f32,
    pub max_life: f32,
    pub size: i32,
    pub color: Color,
}

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0x1234_5678_9ABC_DEF0);
        Rng {
            state: nanos ^ 0xA076_1D64_78BD_642F ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.state = value;
        value
    }

    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }

    pub fn range(&mut self, minimum: f64, maximum: f64) -> f64 {
        minimum + (maximum - minimum) * self.unit()
    }

    pub fn index(&mut self, length: usize) -> usize {
        if length == 0 {
            0
        } else {
            (self.next_u64() % length as u64) as usize
        }
    }
}

impl Default for Rng {
    fn default() -> Self {
        Rng::new()
    }
}

struct Economy {
    click_flat: f64,
    click_multiplier: f64,
    production_multiplier: f64,
    global_multiplier: f64,
    click_production_share: f64,
    auto_pat_rate: f64,
    offline_multiplier: f64,
    building_multipliers: Vec<f64>,
}

pub struct Game {
    pub buildings: &'static [BuildingDef],
    pub upgrades: &'static [UpgradeDef],
    pub achievements: &'static [AchievementDef],
    pub affection: f64,
    pub total_earned: f64,
    pub total_clicks: u64,
    pub building_counts: Vec<u64>,
    pub owned_upgrades: BTreeSet<String>,
    pub unlocked_achievements: BTreeSet<String>,
    pub revealed_buildings: BTreeSet<String>,
    pub revealed_upgrades: BTreeSet<String>,
    pub stats: SaveStats,
    pub settings: Settings,
    pub save_manager: SaveManager,
    pub audio: AudioSystem,
    pub pat: PatAnimation,
    pub dialogue: DialogueBox,
    pub ui: UiState,
    pub floaters: Vec<Floater>,
    pub hearts: Vec<HeartParticle>,
    pub toasts: Vec<Toast>,
    pub notice: Option<Notice>,
    pub rng: Rng,
    pub clock: f64,
    autosave_timer: f64,
    reaction_cooldown: f64,
    last_dialogue_at: f64,
}

impl Game {
    pub async fn new() -> Self {
        let mut save_manager = SaveManager::new();
        let (save_data, load_warning, had_save) = match save_manager.load() {
            Ok(Some(data)) => (data, None, true),
            Ok(None) => (SaveData::default(), None, false),
            Err(error) => (SaveData::default(), Some(error), false),
        };
        let initial_last_saved = save_data.last_saved;
        let settings = save_data.settings.clone();
        let audio = AudioSystem::new(&settings).await;
        let pat = PatAnimation::load();
        let mut game = Game::from_save_data(save_data, save_manager, audio, pat);
        game.begin_session(had_save, initial_last_saved, load_warning);
        game
    }

    fn from_save_data(
        data: SaveData,
        save_manager: SaveManager,
        audio: AudioSystem,
        pat: PatAnimation,
    ) -> Self {
        let building_defs = buildings::building_defs();
        let upgrade_defs = upgrades::upgrade_defs();
        let achievement_defs = achievements::achievement_defs();
        let mut building_counts = vec![0u64; building_defs.len()];
        for (id, count) in data.buildings {
            if let Some(index) = buildings::building_index(&id) {
                building_counts[index] = count;
            }
        }
        let mut owned_upgrades: BTreeSet<String> = data
            .upgrades
            .into_iter()
            .filter(|id| upgrades::upgrade_index(id).is_some())
            .collect();
        let mut unlocked_achievements: BTreeSet<String> = data
            .achievements
            .into_iter()
            .filter(|id| achievements::achievement_index(id).is_some())
            .collect();
        let mut revealed_buildings: BTreeSet<String> = data
            .revealed_buildings
            .into_iter()
            .filter(|id| buildings::building_index(id).is_some())
            .collect();
        let mut revealed_upgrades: BTreeSet<String> = data
            .revealed_upgrades
            .into_iter()
            .filter(|id| upgrades::upgrade_index(id).is_some())
            .collect();
        if building_defs.is_empty() {
            building_counts.clear();
        }
        owned_upgrades.retain(|id| upgrades::upgrade_index(id).is_some());
        unlocked_achievements.retain(|id| achievements::achievement_index(id).is_some());
        revealed_buildings.retain(|id| buildings::building_index(id).is_some());
        revealed_upgrades.retain(|id| upgrades::upgrade_index(id).is_some());
        Game {
            buildings: building_defs,
            upgrades: upgrade_defs,
            achievements: achievement_defs,
            affection: data.affection,
            total_earned: data.total_earned,
            total_clicks: data.total_clicks,
            building_counts,
            owned_upgrades,
            unlocked_achievements,
            revealed_buildings,
            revealed_upgrades,
            stats: data.stats,
            settings: data.settings,
            save_manager,
            audio,
            pat,
            dialogue: DialogueBox::new(),
            ui: UiState::new(),
            floaters: Vec::new(),
            hearts: Vec::new(),
            toasts: Vec::new(),
            notice: None,
            rng: Rng::new(),
            clock: 0.0,
            autosave_timer: 0.0,
            reaction_cooldown: 0.0,
            last_dialogue_at: 0.0,
        }
    }

    fn begin_session(&mut self, had_save: bool, last_saved: i64, load_warning: Option<String>) {
        self.settings.normalize();
        self.stats.sessions = self.stats.sessions.saturating_add(1);
        self.refresh_reveals();
        let now = save::unix_now();
        let mut offline_line: Option<String> = None;
        if had_save {
            let away = (now - last_saved).max(0) as f64;
            let away = if away.is_finite() { away } else { 0.0 };
            self.stats.last_offline_seconds = away;
            let capped = away.min(OFFLINE_CAP_SECONDS);
            let rate = self.production_per_second();
            let earned = (rate * capped * (1.0 + self.offline_multiplier())).max(0.0);
            if earned.is_finite() && earned > 0.0 {
                self.affection += earned;
                self.total_earned += earned;
                self.stats.total_offline_earnings += earned;
            }
            if earned > 0.0 {
                let mut line = format!(
                    "You were away for {}. I collected {} Affection while you were gone! Sancho counted it twice to be sure.",
                    format::duration(away),
                    format::number(earned)
                );
                if away > OFFLINE_CAP_SECONDS {
                    line.push_str(
                        " I only remember the first eight hours. The rest was a beautiful dream.",
                    );
                }
                offline_line = Some(line);
            } else {
                offline_line = Some(
                    "Welcome back! I guarded the castle while you were away. I collected zero Affection, but a truly heroic amount of determination."
                        .to_string(),
                );
            }
        }
        self.refresh_reveals();
        self.check_achievements();
        if let Some(error) = load_warning {
            self.notice = Some(Notice {
                text: format!("OLD SAVE WAS DAMAGED: {}", error),
                timer: 12.0,
                error: true,
            });
            self.set_dialogue(
                "The old chronicle was damaged, so we began a fresh page. A copy of the old scroll was kept, just in case. Now then, where were we?",
            );
        } else if let Some(line) = offline_line {
            self.set_dialogue(line);
        } else if had_save {
            self.set_dialogue("Ah, my friend returns! The windmills have been quiet without you. Shall we continue the quest?");
        } else {
            self.set_dialogue(
                "Ah! A new face at the castle gate! I am Don Quixote, knight, dreamer, and professional receiver of pats. Please pat me, brave friend!",
            );
        }
        self.flush_autosave();
    }

    fn flush_autosave(&mut self) {
        if let Err(error) = self.write_current_save() {
            self.notice = Some(Notice {
                text: format!("SAVE FAILED: {}", error),
                timer: 8.0,
                error: true,
            });
        }
    }

    pub fn update(&mut self, delta: f64) {
        let dt = delta.clamp(0.0, 0.25);
        self.clock += dt;
        let rate = self.production_per_second();
        if rate > 0.0 {
            let gain = rate * dt;
            self.affection += gain;
            self.total_earned += gain;
        }
        self.stats.playtime += dt;
        self.pat.update(dt as f32);
        if self.pat.current_cps() > self.stats.peak_cps {
            self.stats.peak_cps = self.pat.current_cps();
        }
        if self.dialogue.update(dt as f32) {
            self.audio.play_blip();
        }
        self.reaction_cooldown = (self.reaction_cooldown - dt).max(0.0);
        self.update_effects(dt as f32);
        if let Some(toast) = self.toasts.first_mut() {
            toast.timer -= dt as f32;
        }
        if self
            .toasts
            .first()
            .map(|toast| toast.timer <= 0.0)
            .unwrap_or(false)
        {
            self.toasts.remove(0);
        }
        if let Some(notice) = &mut self.notice {
            notice.timer -= dt as f32;
        }
        if self
            .notice
            .as_ref()
            .map(|notice| notice.timer <= 0.0)
            .unwrap_or(false)
        {
            self.notice = None;
        }
        if self.clock - self.last_dialogue_at > 16.0 && self.dialogue.is_complete() {
            let index = self.rng.index(IDLE_LINES.len());
            self.set_dialogue(IDLE_LINES[index]);
        }
        self.refresh_reveals();
        self.check_achievements();
        let stage = stage_for_progress(self.total_earned, self.total_buildings());
        self.audio.update(dt, &self.settings, stage);
        self.autosave_timer += dt;
        if self.autosave_timer >= 30.0 {
            self.autosave();
        }
    }

    fn update_effects(&mut self, dt: f32) {
        for floater in &mut self.floaters {
            floater.life -= dt;
            floater.y += floater.velocity_y * dt;
            floater.velocity_y += 24.0 * dt;
        }
        self.floaters.retain(|floater| floater.life > 0.0);
        for heart in &mut self.hearts {
            heart.life -= dt;
            heart.x += heart.velocity_x * dt;
            heart.y += heart.velocity_y * dt;
            heart.velocity_y += 18.0 * dt;
            heart.velocity_x *= 0.99;
        }
        self.hearts.retain(|heart| heart.life > 0.0);
    }

    fn set_dialogue(&mut self, text: impl Into<String>) {
        self.dialogue.say(text);
        self.last_dialogue_at = self.clock;
    }

    pub fn pat(&mut self, x: f32, y: f32) {
        let amount = self.click_value();
        self.affection += amount;
        self.total_earned += amount;
        self.total_clicks = self.total_clicks.saturating_add(1);
        self.pat.note_click();
        self.audio.play_pat();
        self.floaters.push(Floater {
            x,
            y,
            velocity_y: -34.0,
            life: 1.05,
            max_life: 1.05,
            text: format!("+{}", format::number(amount)),
        });
        let heart_count = 3 + (self.rng.index(3));
        for _ in 0..heart_count {
            let angle = self.rng.range(-2.6, -0.5);
            let speed = self.rng.range(22.0, 58.0);
            self.hearts.push(HeartParticle {
                x: x + self.rng.range(-13.0, 13.0) as f32,
                y: y + self.rng.range(-8.0, 8.0) as f32,
                velocity_x: angle.cos() as f32 * speed as f32,
                velocity_y: angle.sin() as f32 * speed as f32,
                life: self.rng.range(0.65, 1.15) as f32,
                max_life: 1.15,
                size: if self.rng.unit() > 0.7 { 2 } else { 1 },
                color: if self.rng.unit() > 0.55 {
                    Color::new(0.93, 0.20, 0.34, 1.0)
                } else {
                    Color::new(0.98, 0.64, 0.16, 1.0)
                },
            });
        }
        let clicks = self.total_clicks;
        if clicks == 10 || clicks == 100 || clicks == 1000 || clicks == 10000 || clicks == 100000 {
            self.set_dialogue(format!(
                "{} pats! Sancho says that is a lot. I say it is the beginning of a proper heroic tradition!",
                format::number(clicks as f64)
            ));
            self.reaction_cooldown = 3.0;
        } else if self.reaction_cooldown <= 0.0 {
            let cps = self.pat.current_cps();
            let pool: &[&str] = if cps >= 7.0 {
                &FAST_CLICK_LINES
            } else {
                &CLICK_LINES
            };
            let line = pool[self.rng.index(pool.len())];
            self.set_dialogue(line);
            self.reaction_cooldown = 4.5;
        }
        self.refresh_reveals();
        self.check_achievements();
    }

    pub fn click_value(&self) -> f64 {
        let economy = self.economy();
        let achievement_bonus = self.achievement_bonus();
        let global = economy.global_multiplier * achievement_bonus;
        let base = (1.0 + economy.click_flat) * economy.click_multiplier * global;
        let production = self.production_per_second();
        let share = production * economy.click_production_share;
        (base + share).max(0.0)
    }

    pub fn production_per_second(&self) -> f64 {
        let economy = self.economy();
        let achievement_bonus = self.achievement_bonus();
        let building_multiplier =
            economy.production_multiplier * economy.global_multiplier * achievement_bonus;
        let mut total = 0.0;
        for (index, building) in self.buildings.iter().enumerate() {
            let count = self.building_counts.get(index).copied().unwrap_or(0) as f64;
            let specific = economy
                .building_multipliers
                .get(index)
                .copied()
                .unwrap_or(1.0);
            total += count * building.base_rate * specific * building_multiplier;
        }
        let automatic = economy.auto_pat_rate * economy.global_multiplier * achievement_bonus;
        (total + automatic).max(0.0)
    }

    pub fn building_unit_output(&self, index: usize) -> f64 {
        let Some(building) = self.buildings.get(index) else {
            return 0.0;
        };
        let economy = self.economy();
        let achievement_bonus = self.achievement_bonus();
        let specific = economy
            .building_multipliers
            .get(index)
            .copied()
            .unwrap_or(1.0);
        building.base_rate
            * specific
            * economy.production_multiplier
            * economy.global_multiplier
            * achievement_bonus
    }

    fn economy(&self) -> Economy {
        let mut economy = Economy {
            click_flat: 0.0,
            click_multiplier: 1.0,
            production_multiplier: 1.0,
            global_multiplier: 1.0,
            click_production_share: 0.0,
            auto_pat_rate: 0.0,
            offline_multiplier: 0.0,
            building_multipliers: vec![1.0; self.buildings.len()],
        };
        for upgrade in self.upgrades {
            if !self.owned_upgrades.contains(upgrade.id.as_str()) {
                continue;
            }
            match upgrade.effect {
                UpgradeEffect::ClickFlat(amount) => economy.click_flat += amount,
                UpgradeEffect::ClickMultiplier(amount) => economy.click_multiplier *= amount,
                UpgradeEffect::ProductionMultiplier(amount) => {
                    economy.production_multiplier *= amount
                }
                UpgradeEffect::GlobalMultiplier(amount) => economy.global_multiplier *= amount,
                UpgradeEffect::BuildingMultiplier(index, amount) => {
                    if let Some(multiplier) = economy.building_multipliers.get_mut(index) {
                        *multiplier *= amount;
                    }
                }
                UpgradeEffect::ClickProductionShare(amount) => {
                    economy.click_production_share += amount
                }
                UpgradeEffect::AutoPatRate(amount) => economy.auto_pat_rate += amount,
                UpgradeEffect::OfflineBonus(amount) => economy.offline_multiplier += amount,
            }
        }
        economy
    }

    fn offline_multiplier(&self) -> f64 {
        self.economy().offline_multiplier
    }

    pub fn achievement_bonus(&self) -> f64 {
        achievements::achievement_bonus(self.unlocked_achievements.len() as u64)
    }

    pub fn total_buildings(&self) -> u64 {
        self.building_counts
            .iter()
            .fold(0u64, |total, count| total.saturating_add(*count))
    }

    pub fn max_achievements(&self) -> usize {
        self.achievements.len()
    }

    pub fn achievement_count(&self) -> usize {
        self.unlocked_achievements.len()
    }

    fn unlock_context(&self) -> UnlockContext<'_> {
        UnlockContext {
            total_clicks: self.total_clicks,
            total_earned: self.total_earned,
            total_buildings: self.total_buildings(),
            upgrades_bought: self.owned_upgrades.len() as u64,
            achievements_unlocked: self.unlocked_achievements.len() as u64,
            building_counts: &self.building_counts,
        }
    }

    pub fn upgrade_is_unlocked(&self, def: &UpgradeDef) -> bool {
        upgrades::is_unlocked(def, &self.unlock_context())
    }

    pub fn building_purchase_preview(&self, index: usize, requested: u64) -> (u64, f64) {
        let Some(def) = self.buildings.get(index) else {
            return (0, f64::INFINITY);
        };
        let owned = self.building_counts.get(index).copied().unwrap_or(0);
        buildings::purchase_cost(def, owned, requested, self.affection)
    }

    pub fn can_buy_building(&self, index: usize, requested: u64) -> bool {
        let (amount, cost) = self.building_purchase_preview(index, requested);
        amount > 0 && cost.is_finite() && cost <= self.affection
    }

    pub fn try_buy_building(&mut self, index: usize, requested: u64) -> bool {
        let Some(def) = self.buildings.get(index) else {
            self.audio.play_error();
            return false;
        };
        if !self.revealed_buildings.contains(def.id) {
            return false;
        }
        let (amount, cost) = self.building_purchase_preview(index, requested);
        if amount == 0 || !cost.is_finite() || cost > self.affection {
            self.audio.play_error();
            if self.reaction_cooldown <= 0.0 {
                self.set_dialogue("Alas! Not enough affection. The treasury is mostly lint and one very loyal button.");
                self.reaction_cooldown = 4.0;
            }
            return false;
        }
        self.affection -= cost;
        self.building_counts[index] = self.building_counts[index].saturating_add(amount);
        self.stats.total_spent += cost;
        self.stats.best_bulk_purchase = self.stats.best_bulk_purchase.max(amount);
        self.audio.play_purchase();
        let line = BUILDING_LINES[self.rng.index(BUILDING_LINES.len())].replace("{name}", def.name);
        self.set_dialogue(line);
        self.reaction_cooldown = 2.0;
        self.refresh_reveals();
        self.check_achievements();
        true
    }

    pub fn try_buy_upgrade(&mut self, index: usize) -> bool {
        let Some(def) = self.upgrades.get(index) else {
            self.audio.play_error();
            return false;
        };
        if self.owned_upgrades.contains(def.id.as_str()) {
            return false;
        }
        if !self.revealed_upgrades.contains(def.id.as_str()) {
            self.audio.play_error();
            return false;
        }
        if !self.upgrade_is_unlocked(def) || def.cost > self.affection {
            self.audio.play_error();
            return false;
        }
        self.affection -= def.cost;
        self.owned_upgrades.insert(def.id.clone());
        self.stats.total_spent += def.cost;
        self.audio.play_purchase();
        let line = UPGRADE_LINES[self.rng.index(UPGRADE_LINES.len())].replace("{name}", &def.name);
        self.set_dialogue(line);
        self.reaction_cooldown = 2.0;
        self.refresh_reveals();
        self.check_achievements();
        true
    }

    pub fn refresh_reveals(&mut self) {
        for (index, def) in self.buildings.iter().enumerate() {
            if self.revealed_buildings.contains(def.id) {
                continue;
            }
            let owned = self.building_counts.get(index).copied().unwrap_or(0);
            let previous = if index == 0 {
                true
            } else {
                self.building_counts.get(index - 1).copied().unwrap_or(0) > 0
            };
            let affordable = buildings::next_cost(def, owned) <= self.affection;
            if owned > 0 || previous || affordable {
                self.revealed_buildings.insert(def.id.to_string());
            }
        }
        for def in self.upgrades {
            if self.revealed_upgrades.contains(def.id.as_str()) {
                continue;
            }
            let starter = def.chain_index == 0;
            let previous_purchased = self.upgrades.iter().any(|other| {
                other.category == def.category
                    && other.chain_index + 1 == def.chain_index
                    && self.owned_upgrades.contains(other.id.as_str())
            });
            let affordable = def.cost <= self.affection;
            if starter || previous_purchased || affordable {
                self.revealed_upgrades.insert(def.id.clone());
            }
        }
    }

    pub fn achievement_progress(&self, condition: AchievementCondition) -> (f64, f64) {
        let context = AchievementContext {
            total_clicks: self.total_clicks,
            total_earned: self.total_earned,
            building_counts: &self.building_counts,
            total_buildings: self.total_buildings(),
            upgrades_bought: self.owned_upgrades.len() as u64,
            peak_cps: self.stats.peak_cps,
            playtime: self.stats.playtime,
            offline_seconds: self.stats.last_offline_seconds,
            manual_saves: self.stats.manual_saves,
            offline_earned: self.stats.total_offline_earnings,
            best_bulk_purchase: self.stats.best_bulk_purchase,
        };
        achievements::progress(condition, &context)
    }

    pub fn check_achievements(&mut self) {
        let newly_unlocked: Vec<(String, String)> = {
            let context = AchievementContext {
                total_clicks: self.total_clicks,
                total_earned: self.total_earned,
                building_counts: &self.building_counts,
                total_buildings: self.total_buildings(),
                upgrades_bought: self.owned_upgrades.len() as u64,
                peak_cps: self.stats.peak_cps,
                playtime: self.stats.playtime,
                offline_seconds: self.stats.last_offline_seconds,
                manual_saves: self.stats.manual_saves,
                offline_earned: self.stats.total_offline_earnings,
                best_bulk_purchase: self.stats.best_bulk_purchase,
            };
            self.achievements
                .iter()
                .filter(|def| !self.unlocked_achievements.contains(def.id.as_str()))
                .filter(|def| achievements::is_met(def.condition, &context))
                .map(|def| (def.id.clone(), def.name.clone()))
                .collect()
        };
        if newly_unlocked.is_empty() {
            return;
        }
        for (id, name) in &newly_unlocked {
            self.unlocked_achievements.insert(id.clone());
            self.toasts.push(Toast {
                name: name.clone(),
                timer: 2.0,
                duration: 2.0,
            });
        }
        self.audio.play_achievement();
        if let Some((_, name)) = newly_unlocked.first() {
            let line =
                ACHIEVEMENT_LINES[self.rng.index(ACHIEVEMENT_LINES.len())].replace("{name}", name);
            self.set_dialogue(line);
        }
        self.refresh_reveals();
    }

    fn snapshot_save(&self) -> SaveData {
        let mut building_map = BTreeMap::new();
        for (index, count) in self.building_counts.iter().enumerate() {
            if *count > 0 {
                if let Some(def) = self.buildings.get(index) {
                    building_map.insert(def.id.to_string(), *count);
                }
            }
        }
        SaveData {
            version: SAVE_VERSION,
            affection: self.affection,
            total_earned: self.total_earned,
            total_clicks: self.total_clicks,
            buildings: building_map,
            upgrades: self.owned_upgrades.clone(),
            achievements: self.unlocked_achievements.clone(),
            revealed_buildings: self.revealed_buildings.clone(),
            revealed_upgrades: self.revealed_upgrades.clone(),
            stats: self.stats.clone(),
            settings: self.settings.clone(),
            last_saved: save::unix_now(),
        }
    }

    fn write_current_save(&mut self) -> Result<(), String> {
        let data = self.snapshot_save();
        self.save_manager.save(&data)
    }

    pub fn manual_save(&mut self) {
        self.stats.manual_saves = self.stats.manual_saves.saturating_add(1);
        self.check_achievements();
        match self.write_current_save() {
            Ok(()) => {
                self.autosave_timer = 0.0;
                let line = SAVE_SUCCESS_LINES[self.rng.index(SAVE_SUCCESS_LINES.len())];
                self.set_dialogue(line);
                self.notice = Some(Notice {
                    text: "JOURNAL SAVED".to_string(),
                    timer: 2.2,
                    error: false,
                });
            }
            Err(error) => {
                self.notice = Some(Notice {
                    text: format!("SAVE FAILED: {}", error),
                    timer: 8.0,
                    error: true,
                });
                self.audio.play_error();
            }
        }
    }

    fn autosave(&mut self) {
        self.autosave_timer = 0.0;
        self.check_achievements();
        match self.write_current_save() {
            Ok(()) => {
                if self.notice.is_none() {
                    self.notice = Some(Notice {
                        text: "AUTOSAVED".to_string(),
                        timer: 1.4,
                        error: false,
                    });
                }
            }
            Err(error) => {
                self.notice = Some(Notice {
                    text: format!("AUTOSAVE FAILED: {}", error),
                    timer: 8.0,
                    error: true,
                });
            }
        }
    }

    pub fn save_on_exit(&mut self) {
        self.check_achievements();
        let _ = self.write_current_save();
    }

    pub fn reset_progress(&mut self) {
        if let Err(error) = self.save_manager.reset() {
            self.notice = Some(Notice {
                text: format!("RESET FAILED: {}", error),
                timer: 8.0,
                error: true,
            });
        }
        self.affection = 0.0;
        self.total_earned = 0.0;
        self.total_clicks = 0;
        for count in &mut self.building_counts {
            *count = 0;
        }
        self.owned_upgrades.clear();
        self.unlocked_achievements.clear();
        self.revealed_buildings.clear();
        self.revealed_upgrades.clear();
        self.stats = SaveStats::default();
        self.stats.sessions = 1;
        self.floaters.clear();
        self.hearts.clear();
        self.toasts.clear();
        self.notice = None;
        self.ui = UiState::new();
        self.autosave_timer = 0.0;
        self.reaction_cooldown = 0.0;
        self.dialogue = DialogueBox::new();
        self.refresh_reveals();
        self.set_dialogue(
            "A fresh page! A new quest! The same glorious knight, naturally. Please pat me, brave friend, and let us begin again!",
        );
        match self.write_current_save() {
            Ok(()) => {
                self.notice = Some(Notice {
                    text: "A NEW QUEST BEGINS".to_string(),
                    timer: 3.0,
                    error: false,
                });
            }
            Err(error) => {
                self.notice = Some(Notice {
                    text: format!("SAVE FAILED: {}", error),
                    timer: 8.0,
                    error: true,
                });
            }
        }
    }

    pub fn upgrade_owned(&self, def: &UpgradeDef) -> bool {
        self.owned_upgrades.contains(def.id.as_str())
    }

    pub fn save_path_display(&self) -> String {
        self.save_manager.display_path()
    }

    pub fn save_using_fallback(&self) -> bool {
        self.save_manager.using_fallback()
    }

    pub fn current_stage(&self) -> usize {
        stage_for_progress(self.total_earned, self.total_buildings())
    }
}
