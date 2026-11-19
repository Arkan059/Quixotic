use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub const SAVE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Settings {
    pub music_volume: f32,
    pub effects_volume: f32,
    pub muted: bool,
    pub dark_theme: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            music_volume: 0.6,
            effects_volume: 0.8,
            muted: false,
            dark_theme: false,
        }
    }
}

impl Settings {
    pub fn normalize(&mut self) {
        if !self.music_volume.is_finite() {
            self.music_volume = 0.6;
        }
        if !self.effects_volume.is_finite() {
            self.effects_volume = 0.8;
        }
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.effects_volume = self.effects_volume.clamp(0.0, 1.0);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct SaveStats {
    pub playtime: f64,
    pub peak_cps: f64,
    pub total_spent: f64,
    pub manual_saves: u64,
    pub last_offline_seconds: f64,
    pub total_offline_earnings: f64,
    pub best_bulk_purchase: u64,
    pub sessions: u64,
}

impl SaveStats {
    pub fn normalize(&mut self) {
        if !self.playtime.is_finite() || self.playtime < 0.0 {
            self.playtime = 0.0;
        }
        if !self.peak_cps.is_finite() || self.peak_cps < 0.0 {
            self.peak_cps = 0.0;
        }
        if !self.total_spent.is_finite() || self.total_spent < 0.0 {
            self.total_spent = 0.0;
        }
        if !self.last_offline_seconds.is_finite() || self.last_offline_seconds < 0.0 {
            self.last_offline_seconds = 0.0;
        }
        if !self.total_offline_earnings.is_finite() || self.total_offline_earnings < 0.0 {
            self.total_offline_earnings = 0.0;
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct SaveData {
    pub version: u32,
    pub affection: f64,
    pub total_earned: f64,
    pub total_clicks: u64,
    pub buildings: BTreeMap<String, u64>,
    pub upgrades: BTreeSet<String>,
    pub achievements: BTreeSet<String>,
    pub revealed_buildings: BTreeSet<String>,
    pub revealed_upgrades: BTreeSet<String>,
    pub stats: SaveStats,
    pub settings: Settings,
    pub last_saved: i64,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            version: SAVE_VERSION,
            affection: 0.0,
            total_earned: 0.0,
            total_clicks: 0,
            buildings: BTreeMap::new(),
            upgrades: BTreeSet::new(),
            achievements: BTreeSet::new(),
            revealed_buildings: BTreeSet::new(),
            revealed_upgrades: BTreeSet::new(),
            stats: SaveStats::default(),
            settings: Settings::default(),
            last_saved: unix_now(),
        }
    }
}

impl SaveData {
    pub fn normalize(&mut self) {
        if !self.affection.is_finite() || self.affection < 0.0 {
            self.affection = 0.0;
        }
        if !self.total_earned.is_finite() || self.total_earned < 0.0 {
            self.total_earned = 0.0;
        }
        if self.total_earned < self.affection {
            self.total_earned = self.affection;
        }
        self.stats.normalize();
        self.settings.normalize();
        if self.version > SAVE_VERSION {
            self.version = SAVE_VERSION;
        }
        self.buildings.retain(|_, count| *count > 0);
    }
}

pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

pub struct SaveManager {
    candidates: Vec<PathBuf>,
    active_index: usize,
}

fn add_candidate(candidates: &mut Vec<PathBuf>, path: PathBuf) {
    if !candidates.iter().any(|existing| existing == &path) {
        candidates.push(path);
    }
}

fn save_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(project) = ProjectDirs::from("com", "arkanquixote", "Quixotic") {
        add_candidate(&mut candidates, project.data_dir().join("save.json"));
        add_candidate(&mut candidates, project.data_local_dir().join("save.json"));
    }
    if let Ok(directory) = std::env::current_dir() {
        add_candidate(&mut candidates, directory.join("quixotic_save.json"));
    }
    if let Ok(executable) = std::env::current_exe() {
        if let Some(directory) = executable.parent() {
            add_candidate(&mut candidates, directory.join("quixotic_save.json"));
        }
    }
    add_candidate(
        &mut candidates,
        std::env::temp_dir().join("Quixotic").join("save.json"),
    );
    candidates
}

fn rename_atomic(source: &Path, target: &Path) -> Result<(), String> {
    if !target.exists() {
        return fs::rename(source, target)
            .map_err(|_| "the save file could not be moved into place".to_string());
    }
    if fs::rename(source, target).is_ok() {
        return Ok(());
    }
    let backup = target.with_extension("bak");
    let _ = fs::remove_file(&backup);
    fs::rename(target, &backup)
        .map_err(|_| "the old save file could not be set aside".to_string())?;
    match fs::rename(source, target) {
        Ok(()) => {
            let _ = fs::remove_file(backup);
            Ok(())
        }
        Err(_) => {
            let _ = fs::rename(backup, target);
            Err("the save file could not be replaced safely".to_string())
        }
    }
}

fn load_from(path: &Path) -> Result<Option<SaveData>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let bytes = fs::read(path).map_err(|_| "the save file could not be read".to_string())?;
    match serde_json::from_slice::<SaveData>(&bytes) {
        Ok(mut data) => {
            data.normalize();
            Ok(Some(data))
        }
        Err(_) => {
            let corrupt = path.with_extension("corrupt.json");
            let _ = fs::rename(path, &corrupt);
            Err("the save file was unreadable and was set aside as a damaged copy".to_string())
        }
    }
}

fn save_to(path: &Path, data: &SaveData) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|directory| !directory.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|_| "the save folder could not be created".to_string())?;
    let temp = path.with_extension("tmp");
    let bytes = serde_json::to_vec_pretty(data)
        .map_err(|_| "the save data could not be encoded".to_string())?;
    {
        let mut file = fs::File::create(&temp)
            .map_err(|_| "the temporary save file could not be created".to_string())?;
        file.write_all(&bytes)
            .map_err(|_| "the temporary save file could not be written".to_string())?;
        file.sync_all()
            .map_err(|_| "the temporary save file could not be flushed".to_string())?;
    }
    rename_atomic(&temp, path)
}

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {
            candidates: save_candidates(),
            active_index: 0,
        }
    }

    pub fn load(&mut self) -> Result<Option<SaveData>, String> {
        let mut first_error = None;
        let mut newest: Option<(usize, SaveData)> = None;
        for index in 0..self.candidates.len() {
            let path = self.candidates[index].clone();
            if !path.exists() {
                continue;
            }
            match load_from(&path) {
                Ok(Some(data)) => {
                    let replace = newest
                        .as_ref()
                        .map(|(_, current)| data.last_saved > current.last_saved)
                        .unwrap_or(true);
                    if replace {
                        newest = Some((index, data));
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    if first_error.is_none() {
                        first_error = Some(error);
                    }
                }
            }
        }
        if let Some((index, data)) = newest {
            self.active_index = index;
            return Ok(Some(data));
        }
        match first_error {
            Some(error) => Err(error),
            None => Ok(None),
        }
    }

    pub fn save(&mut self, data: &SaveData) -> Result<(), String> {
        let active = self.active_index;
        let order = std::iter::once(active)
            .chain((0..self.candidates.len()).filter(|index| *index != active));
        for index in order {
            if save_to(&self.candidates[index], data).is_ok() {
                self.active_index = index;
                return Ok(());
            }
        }
        Err("the save file could not be written in any available save location".to_string())
    }

    pub fn reset(&mut self) -> Result<(), String> {
        let mut first_error = None;
        for base in self.candidates.clone() {
            for path in [
                base.clone(),
                base.with_extension("tmp"),
                base.with_extension("bak"),
                base.with_extension("corrupt.json"),
            ] {
                if path.exists() {
                    if fs::remove_file(&path).is_err() && first_error.is_none() {
                        first_error = Some("the save file could not be removed".to_string());
                    }
                }
            }
        }
        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    pub fn display_path(&self) -> String {
        self.candidates
            .get(self.active_index)
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "the save file".to_string())
    }

    pub fn using_fallback(&self) -> bool {
        self.active_index > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manager() -> SaveManager {
        let directory = std::env::temp_dir().join(format!(
            "quixotic-save-test-{}-{}",
            std::process::id(),
            unix_now()
        ));
        SaveManager {
            candidates: vec![directory.join("save.json")],
            active_index: 0,
        }
    }

    fn test_data() -> SaveData {
        let mut data = SaveData::default();
        data.affection = 1234.5;
        data.total_earned = 9876.0;
        data.total_clicks = 42;
        data.buildings.insert("windmill".to_string(), 7);
        data.upgrades.insert("click_00".to_string());
        data.achievements.insert("clicks_00".to_string());
        data.revealed_buildings.insert("windmill".to_string());
        data.revealed_upgrades.insert("click_00".to_string());
        data.stats.manual_saves = 2;
        data.stats.playtime = 3600.0;
        data.settings.music_volume = 0.22;
        data.settings.effects_volume = 0.77;
        data.settings.muted = true;
        data.settings.dark_theme = true;
        data
    }

    #[test]
    fn save_roundtrip_is_atomic_and_persistent() {
        let mut manager = test_manager();
        let data = test_data();
        manager.save(&data).expect("first save should succeed");
        let loaded = manager
            .load()
            .expect("load should work")
            .expect("save should exist");
        assert!((loaded.affection - 1234.5).abs() < 1e-9);
        assert_eq!(loaded.total_clicks, 42);
        assert_eq!(loaded.buildings.get("windmill").copied(), Some(7));
        assert!(loaded.upgrades.contains("click_00"));
        assert!(loaded.settings.muted);
        assert!(loaded.settings.dark_theme);
        assert!((loaded.settings.music_volume - 0.22).abs() < 1e-6);
        let mut updated = loaded;
        updated.affection = 9999.0;
        manager.save(&updated).expect("overwrite should succeed");
        let reloaded = manager
            .load()
            .expect("load should work")
            .expect("save should exist");
        assert!((reloaded.affection - 9999.0).abs() < 1e-9);
        manager.reset().expect("reset should succeed");
        assert!(manager.load().expect("load should work").is_none());
        if let Some(parent) = manager.candidates.first().and_then(|path| path.parent()) {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn save_falls_back_when_primary_is_unavailable() {
        let directory = std::env::temp_dir().join(format!(
            "quixotic-save-fallback-test-{}-{}",
            std::process::id(),
            unix_now()
        ));
        let blocker = directory.join("blocked");
        fs::create_dir_all(&directory).expect("test folder should be created");
        fs::write(&blocker, b"not a directory").expect("blocking file should be written");
        let mut manager = SaveManager {
            candidates: vec![
                blocker.join("save.json"),
                directory.join("fallback").join("save.json"),
            ],
            active_index: 0,
        };
        manager
            .save(&test_data())
            .expect("fallback save should succeed");
        assert_eq!(manager.active_index, 1);
        assert!(manager.using_fallback());
        assert!(manager.candidates[1].exists());
        assert!(manager.load().expect("fallback load should work").is_some());
        manager.reset().expect("fallback reset should succeed");
        let _ = fs::remove_dir_all(&directory);
    }

    #[test]
    fn load_prefers_the_most_recent_save_copy() {
        let directory = std::env::temp_dir().join(format!(
            "quixotic-save-newest-test-{}-{}",
            std::process::id(),
            unix_now()
        ));
        let primary = directory.join("primary").join("save.json");
        let fallback = directory.join("fallback").join("save.json");
        let mut manager = SaveManager {
            candidates: vec![primary, fallback.clone()],
            active_index: 0,
        };
        let mut older = test_data();
        older.last_saved = 100;
        older.affection = 1.0;
        manager.save(&older).expect("older save should succeed");
        let mut newer = test_data();
        newer.last_saved = 200;
        newer.affection = 2.0;
        manager.active_index = 1;
        manager
            .save(&newer)
            .expect("newer fallback save should succeed");
        manager.active_index = 0;
        let loaded = manager
            .load()
            .expect("load should work")
            .expect("a save should exist");
        assert_eq!(manager.active_index, 1);
        assert!((loaded.affection - 2.0).abs() < 1e-9);
        let _ = fs::remove_dir_all(&directory);
    }
}
