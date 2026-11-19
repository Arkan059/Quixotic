use macroquad::prelude::*;

use crate::achievements::condition_label;
use crate::font;
use crate::format;
use crate::game::Game;
use crate::theme;

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;

const CONTENT_X: f32 = 326.0;
const CONTENT_W: f32 = 290.0;
const CONTENT_TOP: f32 = 82.0;
const TAB_Y: f32 = 48.0;
const TAB_H: f32 = 20.0;
const BUILD_LIST_TOP: f32 = 110.0;
const BUILD_ROW_H: f32 = 31.0;
const BUILD_ROW_GAP: f32 = 2.0;
const BUILD_VISIBLE_ROWS: usize = 5;
const UPGRADE_LIST_TOP: f32 = 100.0;
const UPGRADE_ROW_H: f32 = 33.0;
const UPGRADE_ROW_GAP: f32 = 2.0;
const UPGRADE_VISIBLE_ROWS: usize = 5;
const ACH_LIST_TOP: f32 = 99.0;
const ACH_ROW_H: f32 = 22.0;
const ACH_ROW_GAP: f32 = 1.0;
const ACH_VISIBLE_ROWS: usize = 8;
const TOOLTIP_TOP: f32 = 279.0;
const TOOLTIP_HEIGHT: f32 = 95.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Buildings,
    Upgrades,
    Achievements,
    Stats,
    Settings,
}

impl Tab {
    pub const ALL: [Tab; 5] = [
        Tab::Buildings,
        Tab::Upgrades,
        Tab::Achievements,
        Tab::Stats,
        Tab::Settings,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Tab::Buildings => "BUILD",
            Tab::Upgrades => "UPGRADES",
            Tab::Achievements => "AWARDS",
            Tab::Stats => "STATS",
            Tab::Settings => "SETTINGS",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Tab::Buildings => "BUILDINGS",
            Tab::Upgrades => "UPGRADES",
            Tab::Achievements => "AWARDS",
            Tab::Stats => "STATISTICS",
            Tab::Settings => "OPTIONS",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FocusId {
    Tab(Tab),
    Buy(usize),
    Building(usize),
    Upgrade(usize),
    Achievement(usize),
    Setting(usize),
}

pub struct UiState {
    pub tab: Tab,
    pub buy_index: usize,
    pub focus: FocusId,
    pub scroll_buildings: usize,
    pub scroll_upgrades: usize,
    pub scroll_achievements: usize,
    pub reset_modal: bool,
    pub reset_choice: usize,
    pub dragging_slider: Option<usize>,
    last_mouse: Option<Vec2>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            tab: Tab::Buildings,
            buy_index: 0,
            focus: FocusId::Tab(Tab::Buildings),
            scroll_buildings: 0,
            scroll_upgrades: 0,
            scroll_achievements: 0,
            reset_modal: false,
            reset_choice: 0,
            dragging_slider: None,
            last_mouse: None,
        }
    }

    pub fn buy_amount(&self) -> u64 {
        match self.buy_index {
            0 => 1,
            1 => 10,
            2 => 100,
            _ => u64::MAX,
        }
    }
}

#[derive(Clone, Copy)]
struct FocusEntry {
    id: FocusId,
    rect: Rect,
    visible: bool,
}

pub fn screen_viewport() -> (Rect, f32) {
    let width = screen_width().max(1.0);
    let height = screen_height().max(1.0);
    let scale = (width / SCREEN_WIDTH)
        .min(height / SCREEN_HEIGHT)
        .max(0.001);
    let draw_width = SCREEN_WIDTH * scale;
    let draw_height = SCREEN_HEIGHT * scale;
    (
        Rect::new(
            (width - draw_width) * 0.5,
            (height - draw_height) * 0.5,
            draw_width,
            draw_height,
        ),
        scale,
    )
}

fn game_mouse_position() -> Option<Vec2> {
    let (viewport, scale) = screen_viewport();
    let (mouse_x, mouse_y) = mouse_position();
    if mouse_x < viewport.x
        || mouse_y < viewport.y
        || mouse_x > viewport.x + viewport.w
        || mouse_y > viewport.y + viewport.h
    {
        return None;
    }
    Some(vec2(
        (mouse_x - viewport.x) / scale,
        (mouse_y - viewport.y) / scale,
    ))
}

fn left_panel() -> Rect {
    Rect::new(10.0, 38.0, 298.0, 348.0)
}

fn right_panel() -> Rect {
    Rect::new(314.0, 38.0, 316.0, 348.0)
}

fn dialogue_rect() -> Rect {
    Rect::new(10.0, 392.0, 620.0, 78.0)
}

fn character_rect() -> Rect {
    Rect::new(46.0, 112.0, 226.0, 226.0)
}

fn tab_rects() -> Vec<(Tab, Rect)> {
    let mut result = Vec::with_capacity(5);
    let mut x = 326.0;
    for tab in Tab::ALL {
        let width = font::measure_text(tab.label(), 1) as f32 + 18.0;
        result.push((tab, Rect::new(x, TAB_Y, width, TAB_H)));
        x += width + 2.0;
    }
    result
}

fn buy_rects() -> Vec<Rect> {
    let mut result = Vec::with_capacity(4);
    let width = 66.0;
    let gap = 6.0;
    let mut x = 332.0;
    for _ in 0..4 {
        result.push(Rect::new(x, CONTENT_TOP, width, 20.0));
        x += width + gap;
    }
    result
}

fn settings_rects() -> Vec<Rect> {
    vec![
        Rect::new(CONTENT_X, CONTENT_TOP, CONTENT_W, 42.0),
        Rect::new(CONTENT_X, 130.0, CONTENT_W, 42.0),
        Rect::new(CONTENT_X, 178.0, CONTENT_W, 28.0),
        Rect::new(CONTENT_X, 212.0, CONTENT_W, 28.0),
        Rect::new(CONTENT_X, 246.0, CONTENT_W, 28.0),
        Rect::new(CONTENT_X, 280.0, CONTENT_W, 28.0),
    ]
}

fn slider_track(rect: Rect) -> Rect {
    Rect::new(rect.x + 16.0, rect.y + 27.0, rect.w - 32.0, 6.0)
}

fn revealed_building_indices(game: &Game) -> Vec<usize> {
    game.buildings
        .iter()
        .enumerate()
        .filter(|(_, building)| game.revealed_buildings.contains(building.id))
        .map(|(index, _)| index)
        .collect()
}

fn revealed_upgrade_indices(game: &Game) -> Vec<usize> {
    game.upgrades
        .iter()
        .enumerate()
        .filter(|(_, upgrade)| game.revealed_upgrades.contains(upgrade.id.as_str()))
        .map(|(index, _)| index)
        .collect()
}

fn building_row_rect(position: usize, scroll: usize) -> Rect {
    let offset = position as f32 - scroll as f32;
    Rect::new(
        CONTENT_X,
        BUILD_LIST_TOP + offset * (BUILD_ROW_H + BUILD_ROW_GAP),
        CONTENT_W,
        BUILD_ROW_H,
    )
}

fn upgrade_row_rect(position: usize, scroll: usize) -> Rect {
    let offset = position as f32 - scroll as f32;
    Rect::new(
        CONTENT_X,
        UPGRADE_LIST_TOP + offset * (UPGRADE_ROW_H + UPGRADE_ROW_GAP),
        CONTENT_W,
        UPGRADE_ROW_H,
    )
}

fn achievement_row_rect(position: usize, scroll: usize) -> Rect {
    let offset = position as f32 - scroll as f32;
    Rect::new(
        CONTENT_X,
        ACH_LIST_TOP + offset * (ACH_ROW_H + ACH_ROW_GAP),
        CONTENT_W,
        ACH_ROW_H,
    )
}

fn clamp_scrolls(game: &mut Game) {
    let building_count = revealed_building_indices(game).len();
    let upgrade_count = revealed_upgrade_indices(game).len();
    let achievement_count = game.achievements.len();
    game.ui.scroll_buildings = game
        .ui
        .scroll_buildings
        .min(building_count.saturating_sub(BUILD_VISIBLE_ROWS));
    game.ui.scroll_upgrades = game
        .ui
        .scroll_upgrades
        .min(upgrade_count.saturating_sub(UPGRADE_VISIBLE_ROWS));
    game.ui.scroll_achievements = game
        .ui
        .scroll_achievements
        .min(achievement_count.saturating_sub(ACH_VISIBLE_ROWS));
}

fn build_focus_entries(game: &Game) -> Vec<FocusEntry> {
    let mut entries = Vec::new();
    for (tab, rect) in tab_rects() {
        entries.push(FocusEntry {
            id: FocusId::Tab(tab),
            rect,
            visible: true,
        });
    }
    match game.ui.tab {
        Tab::Buildings => {
            for (index, rect) in buy_rects().into_iter().enumerate() {
                entries.push(FocusEntry {
                    id: FocusId::Buy(index),
                    rect,
                    visible: true,
                });
            }
            for (position, index) in revealed_building_indices(game).into_iter().enumerate() {
                entries.push(FocusEntry {
                    id: FocusId::Building(index),
                    rect: building_row_rect(position, game.ui.scroll_buildings),
                    visible: position >= game.ui.scroll_buildings
                        && position < game.ui.scroll_buildings + BUILD_VISIBLE_ROWS,
                });
            }
        }
        Tab::Upgrades => {
            for (position, index) in revealed_upgrade_indices(game).into_iter().enumerate() {
                entries.push(FocusEntry {
                    id: FocusId::Upgrade(index),
                    rect: upgrade_row_rect(position, game.ui.scroll_upgrades),
                    visible: position >= game.ui.scroll_upgrades
                        && position < game.ui.scroll_upgrades + UPGRADE_VISIBLE_ROWS,
                });
            }
        }
        Tab::Achievements => {
            for index in 0..game.achievements.len() {
                entries.push(FocusEntry {
                    id: FocusId::Achievement(index),
                    rect: achievement_row_rect(index, game.ui.scroll_achievements),
                    visible: index >= game.ui.scroll_achievements
                        && index < game.ui.scroll_achievements + ACH_VISIBLE_ROWS,
                });
            }
        }
        Tab::Stats => {}
        Tab::Settings => {
            for (index, rect) in settings_rects().into_iter().enumerate() {
                entries.push(FocusEntry {
                    id: FocusId::Setting(index),
                    rect,
                    visible: true,
                });
            }
        }
    }
    entries
}

fn entry_at(game: &Game, mouse: Vec2) -> Option<FocusEntry> {
    build_focus_entries(game)
        .into_iter()
        .find(|entry| entry.visible && entry.rect.contains(mouse))
}

fn ensure_focus_valid(game: &mut Game) {
    let entries = build_focus_entries(game);
    if !entries.iter().any(|entry| entry.id == game.ui.focus) {
        game.ui.focus = FocusId::Tab(game.ui.tab);
    }
}

fn ensure_focus_visible(game: &mut Game) {
    match game.ui.focus {
        FocusId::Building(index) => {
            if let Some(position) = revealed_building_indices(game)
                .iter()
                .position(|candidate| *candidate == index)
            {
                if position < game.ui.scroll_buildings {
                    game.ui.scroll_buildings = position;
                } else if position >= game.ui.scroll_buildings + BUILD_VISIBLE_ROWS {
                    game.ui.scroll_buildings = position + 1 - BUILD_VISIBLE_ROWS;
                }
            }
        }
        FocusId::Upgrade(index) => {
            if let Some(position) = revealed_upgrade_indices(game)
                .iter()
                .position(|candidate| *candidate == index)
            {
                if position < game.ui.scroll_upgrades {
                    game.ui.scroll_upgrades = position;
                } else if position >= game.ui.scroll_upgrades + UPGRADE_VISIBLE_ROWS {
                    game.ui.scroll_upgrades = position + 1 - UPGRADE_VISIBLE_ROWS;
                }
            }
        }
        FocusId::Achievement(index) => {
            if game.ui.scroll_achievements > index {
                game.ui.scroll_achievements = index;
            } else if index >= game.ui.scroll_achievements + ACH_VISIBLE_ROWS {
                game.ui.scroll_achievements = index + 1 - ACH_VISIBLE_ROWS;
            }
        }
        _ => {}
    }
    clamp_scrolls(game);
}

fn navigate_focus(game: &mut Game, delta: isize) {
    let entries = build_focus_entries(game);
    if entries.is_empty() {
        return;
    }
    let current = entries
        .iter()
        .position(|entry| entry.id == game.ui.focus)
        .unwrap_or(0);
    let count = entries.len() as isize;
    let next = (current as isize + delta).rem_euclid(count) as usize;
    game.ui.focus = entries[next].id;
    ensure_focus_visible(game);
    game.audio.play_cursor();
}

fn set_slider_from_x(game: &mut Game, index: usize, mouse_x: f32) {
    let rects = settings_rects();
    let Some(rect) = rects.get(index) else {
        return;
    };
    let track = slider_track(*rect);
    let value = ((mouse_x - track.x) / track.w.max(1.0)).clamp(0.0, 1.0);
    if index == 0 {
        game.settings.music_volume = value;
    } else if index == 1 {
        game.settings.effects_volume = value;
    }
}

fn adjust_slider(game: &mut Game, index: usize, direction: f32) {
    let step = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        0.10
    } else {
        0.05
    };
    if index == 0 {
        game.settings.music_volume =
            (game.settings.music_volume + direction * step).clamp(0.0, 1.0);
    } else if index == 1 {
        game.settings.effects_volume =
            (game.settings.effects_volume + direction * step).clamp(0.0, 1.0);
    }
    game.audio.play_cursor();
}

fn activate(game: &mut Game, id: FocusId) {
    match id {
        FocusId::Tab(tab) => {
            game.ui.tab = tab;
            game.ui.focus = FocusId::Tab(tab);
            game.audio.play_cursor();
        }
        FocusId::Buy(index) => {
            game.ui.buy_index = index.min(3);
            game.ui.focus = FocusId::Buy(index.min(3));
            game.audio.play_cursor();
        }
        FocusId::Building(index) => {
            let amount = game.ui.buy_amount();
            game.try_buy_building(index, amount);
        }
        FocusId::Upgrade(index) => {
            game.try_buy_upgrade(index);
        }
        FocusId::Achievement(_) => {}
        FocusId::Setting(0) | FocusId::Setting(1) => {}
        FocusId::Setting(2) => {
            game.settings.muted = !game.settings.muted;
            game.audio.play_cursor();
        }
        FocusId::Setting(3) => {
            game.settings.dark_theme = !game.settings.dark_theme;
            game.audio.play_cursor();
        }
        FocusId::Setting(4) => {
            game.manual_save();
        }
        FocusId::Setting(5) => {
            game.ui.reset_modal = true;
            game.ui.reset_choice = 0;
            game.audio.play_cursor();
        }
        FocusId::Setting(_) => {}
    }
}

fn scroll_active_tab(game: &mut Game, direction: isize) -> bool {
    let before = (
        game.ui.scroll_buildings,
        game.ui.scroll_upgrades,
        game.ui.scroll_achievements,
    );
    match game.ui.tab {
        Tab::Buildings => {
            let max = revealed_building_indices(game)
                .len()
                .saturating_sub(BUILD_VISIBLE_ROWS);
            game.ui.scroll_buildings =
                (game.ui.scroll_buildings as isize + direction).clamp(0, max as isize) as usize;
        }
        Tab::Upgrades => {
            let max = revealed_upgrade_indices(game)
                .len()
                .saturating_sub(UPGRADE_VISIBLE_ROWS);
            game.ui.scroll_upgrades =
                (game.ui.scroll_upgrades as isize + direction).clamp(0, max as isize) as usize;
        }
        Tab::Achievements => {
            let max = game.achievements.len().saturating_sub(ACH_VISIBLE_ROWS);
            game.ui.scroll_achievements =
                (game.ui.scroll_achievements as isize + direction).clamp(0, max as isize) as usize;
        }
        _ => {}
    }
    before
        != (
            game.ui.scroll_buildings,
            game.ui.scroll_upgrades,
            game.ui.scroll_achievements,
        )
}

fn modal_buttons() -> [Rect; 2] {
    [
        Rect::new(132.0, 276.0, 178.0, 32.0),
        Rect::new(330.0, 276.0, 178.0, 32.0),
    ]
}

fn handle_modal(game: &mut Game, mouse: Option<Vec2>, moved: bool, pressed: bool) {
    let buttons = modal_buttons();
    if moved {
        if let Some(position) = mouse {
            if buttons[0].contains(position) && game.ui.reset_choice != 0 {
                game.ui.reset_choice = 0;
                game.audio.play_cursor();
            } else if buttons[1].contains(position) && game.ui.reset_choice != 1 {
                game.ui.reset_choice = 1;
                game.audio.play_cursor();
            }
        }
    }
    if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Up) {
        if game.ui.reset_choice != 0 {
            game.ui.reset_choice = 0;
            game.audio.play_cursor();
        }
    }
    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Down) {
        if game.ui.reset_choice != 1 {
            game.ui.reset_choice = 1;
            game.audio.play_cursor();
        }
    }
    let mut confirm = false;
    let mut cancel = is_key_pressed(KeyCode::Escape);
    if is_key_pressed(KeyCode::Enter)
        || is_key_pressed(KeyCode::KpEnter)
        || is_key_pressed(KeyCode::Z)
    {
        confirm = true;
    }
    if pressed {
        if let Some(position) = mouse {
            if buttons[0].contains(position) {
                game.ui.reset_choice = 0;
                cancel = true;
            } else if buttons[1].contains(position) {
                game.ui.reset_choice = 1;
                confirm = true;
            }
        }
    }
    if cancel {
        game.ui.reset_modal = false;
        game.ui.dragging_slider = None;
        game.ui.focus = FocusId::Setting(5);
        game.audio.play_cursor();
        return;
    }
    if confirm {
        game.ui.reset_modal = false;
        if game.ui.reset_choice == 1 {
            game.reset_progress();
            game.ui.tab = Tab::Buildings;
            game.ui.focus = FocusId::Tab(Tab::Buildings);
            game.audio.play_cursor();
        } else {
            game.ui.focus = FocusId::Setting(5);
            game.audio.play_cursor();
        }
    }
}

pub fn update(game: &mut Game) {
    show_mouse(false);
    clamp_scrolls(game);
    let mouse = game_mouse_position();
    let moved = match (mouse, game.ui.last_mouse) {
        (Some(current), Some(previous)) => current.distance(previous) > 0.4,
        (Some(_), None) => true,
        _ => false,
    };
    game.ui.last_mouse = mouse;
    let pressed = is_mouse_button_pressed(MouseButton::Left);
    let released = is_mouse_button_released(MouseButton::Left);
    if released {
        game.ui.dragging_slider = None;
    }
    if game.ui.reset_modal {
        handle_modal(game, mouse, moved, pressed);
        return;
    }
    if let Some(index) = game.ui.dragging_slider {
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(position) = mouse {
                set_slider_from_x(game, index, position.x);
            }
        } else {
            game.ui.dragging_slider = None;
        }
    }
    if moved && game.ui.dragging_slider.is_none() {
        if let Some(position) = mouse {
            if let Some(entry) = entry_at(game, position) {
                game.ui.focus = entry.id;
            }
        }
    }
    let wheel = mouse_wheel().1;
    if wheel != 0.0 {
        if let Some(position) = mouse {
            if right_panel().contains(position) {
                let direction = if wheel > 0.0 { -1 } else { 1 };
                if scroll_active_tab(game, direction) {
                    game.audio.play_cursor();
                }
            }
        }
    }
    let mut consumed = false;
    if pressed {
        if let Some(position) = mouse {
            if let Some(entry) = entry_at(game, position) {
                game.ui.focus = entry.id;
                match entry.id {
                    FocusId::Setting(index @ 0..=1) => {
                        set_slider_from_x(game, index, position.x);
                        game.ui.dragging_slider = Some(index);
                    }
                    _ => activate(game, entry.id),
                }
                consumed = true;
            } else if character_rect().contains(position) {
                game.pat(position.x, position.y);
                consumed = true;
            } else if dialogue_rect().contains(position) {
                game.dialogue.finish();
                consumed = true;
            }
        }
    }
    if !consumed {
        let left = is_key_pressed(KeyCode::Left);
        let right = is_key_pressed(KeyCode::Right);
        let up = is_key_pressed(KeyCode::Up);
        let down = is_key_pressed(KeyCode::Down);
        let mut nav_delta = 0isize;
        if let FocusId::Setting(index @ 0..=1) = game.ui.focus {
            if left || right {
                adjust_slider(game, index, if left { -1.0 } else { 1.0 });
            } else if up {
                nav_delta = -1;
            } else if down {
                nav_delta = 1;
            }
        } else {
            if up || left {
                nav_delta = -1;
            }
            if down || right {
                nav_delta = 1;
            }
        }
        if nav_delta != 0 {
            navigate_focus(game, nav_delta);
        }
        if is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::KpEnter)
            || is_key_pressed(KeyCode::Z)
        {
            let focus = game.ui.focus;
            activate(game, focus);
        }
        if is_key_pressed(KeyCode::Space) {
            let area = character_rect();
            game.pat(area.x + area.w * 0.5, area.y + area.h * 0.55);
        }
    }
    ensure_focus_valid(game);
    clamp_scrolls(game);
}

pub fn draw(game: &Game) {
    theme::apply(&game.settings);
    show_mouse(false);
    draw_header(game);
    draw_left_panel(game);
    draw_right_panel(game);
    draw_particles(game);
    game.dialogue.draw(dialogue_rect());
    draw_notice(game);
    draw_toast(game);
    if game.ui.reset_modal {
        draw_reset_modal(game);
    }
    draw_mouse_cursor();
}

fn draw_header(game: &Game) {
    font::draw_text("QUIXOTIC", 14, 7, 2, theme::ink());
    if game.notice.is_none() && game.toasts.is_empty() {
        let text = format!(
            "AWARDS {} / {}",
            game.achievement_count(),
            game.max_achievements()
        );
        font::draw_text_right(&text, 628, 16, 1, theme::faint_ink());
    }
    draw_rectangle(10.0, 33.0, 620.0, 1.0, theme::rule());
}

fn draw_left_panel(game: &Game) {
    let panel = left_panel();
    draw_panel(panel);
    font::draw_text_centered("AFFECTION", 159, 48, 1, theme::soft_ink());
    let affection_text = format::number(game.affection);
    let affection_scale = font::fit_scale(&affection_text, 236, 4);
    let affection_width = font::measure_text(&affection_text, affection_scale);
    font::draw_text_centered(&affection_text, 159, 58, affection_scale, theme::ink());
    let heart_scale = if affection_scale >= 3 { 2 } else { 1 };
    let heart_height = 7 * heart_scale;
    let value_height = font::GLYPH_HEIGHT * affection_scale;
    let heart_y = 58 + (value_height - heart_height) / 2;
    let heart_x = 159 - affection_width / 2 - 14 - heart_scale * 7;
    font::draw_heart(heart_x.max(22), heart_y, heart_scale, theme::gold());
    let rate_text = format!(
        "+{} PER SECOND",
        format::number(game.production_per_second())
    );
    let rate_scale = font::fit_scale(&rate_text, 260, 1);
    font::draw_text_centered(&rate_text, 159, 91, rate_scale, theme::pink());
    let click_text = format!("+{} PER PAT", format::number(game.click_value()));
    let click_scale = font::fit_scale(&click_text, 260, 1);
    font::draw_text_centered(&click_text, 159, 101, click_scale, theme::soft_ink());
    let area = character_rect();
    game.pat.draw(area);
    font::draw_text_centered("CLICK TO PAT  -  SPACE TO PAT", 159, 347, 1, theme::ink());
    let speed_text = format!(
        "PAT SPEED {:.1} PATS/SEC  /  PLAYBACK {:.1}x",
        game.pat.current_cps(),
        game.pat.speed_multiplier()
    );
    let speed_scale = font::fit_scale(&speed_text, 270, 1);
    font::draw_text_centered(&speed_text, 159, 359, speed_scale, theme::soft_ink());
    draw_rectangle(40.0, 338.0, 8.0, 2.0, theme::ink());
    draw_rectangle(272.0, 338.0, 8.0, 2.0, theme::ink());
    draw_rectangle(80.0, 338.0, 158.0, 2.0, theme::rule());
}

fn draw_right_panel(game: &Game) {
    let panel = right_panel();
    draw_panel(panel);
    for (tab, rect) in tab_rects() {
        let focused = game.ui.focus == FocusId::Tab(tab);
        let active = game.ui.tab == tab;
        let fill = if active {
            theme::ink()
        } else if focused {
            theme::pale_gold()
        } else {
            theme::panel()
        };
        draw_button_surface(rect, fill, focused);
        if focused {
            draw_focus_heart(
                rect,
                if active { theme::gold() } else { theme::pink() },
                game.clock,
            );
        }
        let color = if active {
            theme::panel()
        } else if focused {
            theme::ink()
        } else {
            theme::soft_ink()
        };
        font::draw_text(tab.label(), rect.x as i32 + 16, rect.y as i32 + 7, 1, color);
    }
    draw_rectangle(324.0, 73.0, 296.0, 1.0, theme::rule());
    match game.ui.tab {
        Tab::Buildings => draw_buildings_tab(game),
        Tab::Upgrades => draw_upgrades_tab(game),
        Tab::Achievements => draw_achievements_tab(game),
        Tab::Stats => draw_stats_tab(game),
        Tab::Settings => draw_settings_tab(game),
    }
}

fn draw_buildings_tab(game: &Game) {
    let labels = ["x1", "x10", "x100", "MAX"];
    for (index, rect) in buy_rects().into_iter().enumerate() {
        let focused = game.ui.focus == FocusId::Buy(index);
        let selected = game.ui.buy_index == index;
        let fill = if focused {
            theme::pale_gold()
        } else if selected {
            theme::pale_pink()
        } else {
            theme::panel()
        };
        draw_button_surface(rect, fill, focused);
        if focused {
            draw_focus_heart(rect, theme::pink(), game.clock);
        }
        font::draw_text_centered(
            labels[index],
            (rect.x + rect.w / 2.0) as i32,
            rect.y as i32 + 7,
            1,
            theme::ink(),
        );
    }
    let items = revealed_building_indices(game);
    let visible_end = (game.ui.scroll_buildings + BUILD_VISIBLE_ROWS).min(items.len());
    let requested = game.ui.buy_amount();
    for position in game.ui.scroll_buildings..visible_end {
        let index = items[position];
        let building = &game.buildings[index];
        let rect = building_row_rect(position, game.ui.scroll_buildings);
        let focused = game.ui.focus == FocusId::Building(index);
        let affordable = game.can_buy_building(index, requested);
        let fill = if focused {
            theme::pale_gold()
        } else {
            theme::panel()
        };
        draw_button_surface(rect, fill, focused);
        if focused {
            draw_focus_heart(rect, theme::pink(), game.clock);
        }
        let text_color = if affordable {
            theme::ink()
        } else {
            theme::dim()
        };
        let max_name =
            font::measure_text(&format::number(game.building_counts[index] as f64), 1) + 18;
        let name = truncate_text(building.name, CONTENT_W as i32 - 16 - max_name, 1);
        font::draw_text(&name, rect.x as i32 + 16, rect.y as i32 + 5, 1, text_color);
        let owned = format!("x{}", format::number(game.building_counts[index] as f64));
        font::draw_text_right(
            &owned,
            (rect.x + rect.w) as i32 - 9,
            rect.y as i32 + 5,
            1,
            if game.building_counts[index] > 0 {
                theme::gold()
            } else {
                theme::dim()
            },
        );
        let (amount, cost) = game.building_purchase_preview(index, requested);
        let purchase_text = if amount == 0 {
            format!(
                "NEXT: {}",
                format::number(crate::buildings::next_cost(
                    building,
                    game.building_counts[index]
                ))
            )
        } else if requested == u64::MAX {
            format!(
                "BUY {} FOR {}",
                format::number(amount as f64),
                format::number(cost)
            )
        } else {
            format!(
                "BUY {} FOR {}",
                format::number(amount as f64),
                format::number(cost)
            )
        };
        let second = truncate_text(&purchase_text, CONTENT_W as i32 - 24, 1);
        font::draw_text(
            &second,
            rect.x as i32 + 16,
            rect.y as i32 + 18,
            1,
            if affordable {
                theme::soft_ink()
            } else {
                theme::dim()
            },
        );
    }
    if items.is_empty() {
        font::draw_text_centered(
            "NO BUILDINGS YET",
            (CONTENT_X + CONTENT_W / 2.0) as i32,
            180,
            1,
            theme::faint_ink(),
        );
    }
    draw_scrollbar(
        Rect::new(
            CONTENT_X,
            BUILD_LIST_TOP,
            CONTENT_W,
            5.0 * BUILD_ROW_H + 4.0 * BUILD_ROW_GAP,
        ),
        items.len(),
        BUILD_VISIBLE_ROWS,
        game.ui.scroll_buildings,
    );
    draw_list_scroll_hints(
        Rect::new(
            CONTENT_X,
            BUILD_LIST_TOP,
            CONTENT_W,
            5.0 * BUILD_ROW_H + 4.0 * BUILD_ROW_GAP,
        ),
        items.len(),
        BUILD_VISIBLE_ROWS,
        game.ui.scroll_buildings,
    );
    draw_building_tooltip(game, requested);
}

fn draw_building_tooltip(game: &Game, requested: u64) {
    let rect = Rect::new(CONTENT_X, TOOLTIP_TOP, CONTENT_W, TOOLTIP_HEIGHT);
    draw_small_panel(rect);
    let selected = match game.ui.focus {
        FocusId::Building(index) => Some(index),
        _ => None,
    };
    if let Some(index) = selected {
        let Some(building) = game.buildings.get(index) else {
            return;
        };
        let owned = game.building_counts.get(index).copied().unwrap_or(0);
        let name = truncate_text(building.name, 210, 1);
        font::draw_text(&name, rect.x as i32 + 8, rect.y as i32 + 6, 1, theme::ink());
        font::draw_text_right(
            &format!("OWNED x{}", format::number(owned as f64)),
            (rect.x + rect.w) as i32 - 8,
            rect.y as i32 + 6,
            1,
            theme::gold(),
        );
        font::draw_wrapped_limited(
            building.description,
            rect.x as i32 + 8,
            rect.y as i32 + 18,
            (rect.w - 16.0) as i32,
            1,
            3,
            theme::soft_ink(),
        );
        let (amount, cost) = game.building_purchase_preview(index, requested);
        let cost_line = if amount == 0 {
            format!(
                "NEXT UNIT: {}",
                format::number(crate::buildings::next_cost(building, owned))
            )
        } else {
            format!(
                "BUY {}: {} AFFECTION",
                format::number(amount as f64),
                format::number(cost)
            )
        };
        font::draw_text(
            &truncate_text(&cost_line, (rect.w - 16.0) as i32, 1),
            rect.x as i32 + 8,
            rect.y as i32 + 48,
            1,
            theme::ink(),
        );
        let unit = format!(
            "EACH: +{} / SEC",
            format::number(game.building_unit_output(index))
        );
        font::draw_text(
            &truncate_text(&unit, (rect.w - 16.0) as i32, 1),
            rect.x as i32 + 8,
            rect.y as i32 + 61,
            1,
            theme::pink(),
        );
        let total = format!(
            "TOTAL: +{} / SEC",
            format::number(game.building_unit_output(index) * owned as f64)
        );
        font::draw_text(
            &truncate_text(&total, (rect.w - 16.0) as i32, 1),
            rect.x as i32 + 8,
            rect.y as i32 + 74,
            1,
            theme::soft_ink(),
        );
    } else {
        font::draw_text(
            Tab::Buildings.title(),
            rect.x as i32 + 8,
            rect.y as i32 + 6,
            1,
            theme::ink(),
        );
        font::draw_wrapped_limited(
            "Loyal helpers that produce affection every second. Use the amount buttons, then select a building to buy it.",
            rect.x as i32 + 8,
            rect.y as i32 + 19,
            (rect.w - 16.0) as i32,
            1,
            3,
            theme::soft_ink(),
        );
        font::draw_text(
            "A WELL-FUNDED QUEST IS A HAPPY QUEST.",
            rect.x as i32 + 8,
            rect.y as i32 + 61,
            1,
            theme::faint_ink(),
        );
        font::draw_text(
            "ARROWS CHOOSE - ENTER BUYS",
            rect.x as i32 + 8,
            rect.y as i32 + 77,
            1,
            theme::pink(),
        );
    }
}

fn draw_upgrades_tab(game: &Game) {
    let owned = game
        .upgrades
        .iter()
        .filter(|upgrade| game.upgrade_owned(upgrade))
        .count();
    let header = format!(
        "UPGRADES  {} / {} OWNED",
        format::number(owned as f64),
        game.upgrades.len()
    );
    font::draw_text(&header, CONTENT_X as i32, 84, 1, theme::soft_ink());
    font::draw_text_right(
        "ENTER TO BUY",
        (CONTENT_X + CONTENT_W) as i32,
        84,
        1,
        theme::faint_ink(),
    );
    let items = revealed_upgrade_indices(game);
    let visible_end = (game.ui.scroll_upgrades + UPGRADE_VISIBLE_ROWS).min(items.len());
    for position in game.ui.scroll_upgrades..visible_end {
        let index = items[position];
        let upgrade = &game.upgrades[index];
        let rect = upgrade_row_rect(position, game.ui.scroll_upgrades);
        let focused = game.ui.focus == FocusId::Upgrade(index);
        let owned_upgrade = game.upgrade_owned(upgrade);
        let unlocked = game.upgrade_is_unlocked(upgrade);
        let affordable = upgrade.cost <= game.affection;
        let dim = owned_upgrade || !unlocked || !affordable;
        let fill = if focused {
            theme::pale_gold()
        } else if owned_upgrade {
            theme::owned_fill()
        } else {
            theme::panel()
        };
        draw_button_surface(rect, fill, focused);
        if focused {
            draw_focus_heart(rect, theme::pink(), game.clock);
        }
        let text_color = if dim { theme::dim() } else { theme::ink() };
        let cost_text = if owned_upgrade {
            "OWNED".to_string()
        } else {
            format::number(upgrade.cost)
        };
        let cost_width = font::measure_text(&cost_text, 1) + 18;
        let name = truncate_text(&upgrade.name, CONTENT_W as i32 - 16 - cost_width, 1);
        font::draw_text(&name, rect.x as i32 + 16, rect.y as i32 + 5, 1, text_color);
        font::draw_text_right(
            &cost_text,
            (rect.x + rect.w) as i32 - 9,
            rect.y as i32 + 5,
            1,
            if owned_upgrade {
                theme::gold()
            } else if affordable && unlocked {
                theme::ink()
            } else {
                theme::dim()
            },
        );
        let second_text = if owned_upgrade {
            format!(
                "{} - {}",
                upgrade.category.short_label(),
                crate::upgrades::effect_text(upgrade)
            )
        } else if !unlocked {
            format!("LOCKED - {}", crate::upgrades::unlock_text(upgrade))
        } else {
            crate::upgrades::effect_text(upgrade)
        };
        let second = truncate_text(&second_text, CONTENT_W as i32 - 24, 1);
        font::draw_text(
            &second,
            rect.x as i32 + 16,
            rect.y as i32 + 18,
            1,
            if owned_upgrade || !unlocked {
                theme::dim()
            } else {
                theme::pink()
            },
        );
        draw_rectangle(rect.x, rect.y + rect.h - 1.0, rect.w, 1.0, theme::rule());
    }
    if items.is_empty() {
        font::draw_text_centered(
            "NO UPGRADES REVEALED YET",
            (CONTENT_X + CONTENT_W / 2.0) as i32,
            180,
            1,
            theme::faint_ink(),
        );
    }
    draw_scrollbar(
        Rect::new(
            CONTENT_X,
            UPGRADE_LIST_TOP,
            CONTENT_W,
            5.0 * UPGRADE_ROW_H + 4.0 * UPGRADE_ROW_GAP,
        ),
        items.len(),
        UPGRADE_VISIBLE_ROWS,
        game.ui.scroll_upgrades,
    );
    draw_list_scroll_hints(
        Rect::new(
            CONTENT_X,
            UPGRADE_LIST_TOP,
            CONTENT_W,
            5.0 * UPGRADE_ROW_H + 4.0 * UPGRADE_ROW_GAP,
        ),
        items.len(),
        UPGRADE_VISIBLE_ROWS,
        game.ui.scroll_upgrades,
    );
    draw_upgrade_tooltip(game);
}

fn draw_upgrade_tooltip(game: &Game) {
    let rect = Rect::new(CONTENT_X, TOOLTIP_TOP, CONTENT_W, TOOLTIP_HEIGHT);
    draw_small_panel(rect);
    let selected = match game.ui.focus {
        FocusId::Upgrade(index) => Some(index),
        _ => None,
    };
    if let Some(index) = selected {
        let Some(upgrade) = game.upgrades.get(index) else {
            return;
        };
        let name = truncate_text(&upgrade.name, 210, 1);
        font::draw_text(&name, rect.x as i32 + 8, rect.y as i32 + 5, 1, theme::ink());
        let category = format!(
            "{}  -  {}",
            upgrade.category.label(),
            if game.upgrade_owned(upgrade) {
                "PURCHASED"
            } else {
                "AVAILABLE"
            }
        );
        font::draw_text(
            &truncate_text(&category, (rect.w - 16.0) as i32, 1),
            rect.x as i32 + 8,
            rect.y as i32 + 15,
            1,
            if game.upgrade_owned(upgrade) {
                theme::gold()
            } else {
                theme::soft_ink()
            },
        );
        font::draw_wrapped_limited(
            &upgrade.description,
            rect.x as i32 + 8,
            rect.y as i32 + 26,
            (rect.w - 16.0) as i32,
            1,
            3,
            theme::soft_ink(),
        );
        font::draw_text(
            &truncate_text(
                &format!(
                    "COST  {}",
                    if game.upgrade_owned(upgrade) {
                        "PAID".to_string()
                    } else {
                        format::number(upgrade.cost)
                    }
                ),
                (rect.w - 16.0) as i32,
                1,
            ),
            rect.x as i32 + 8,
            rect.y as i32 + 53,
            1,
            theme::ink(),
        );
        font::draw_text(
            &truncate_text(
                &format!("EFFECT  {}", crate::upgrades::effect_text(upgrade)),
                (rect.w - 16.0) as i32,
                1,
            ),
            rect.x as i32 + 8,
            rect.y as i32 + 65,
            1,
            theme::pink(),
        );
        let unlocked = game.upgrade_is_unlocked(upgrade);
        let unlock = format!("UNLOCK  {}", crate::upgrades::unlock_text(upgrade));
        font::draw_text(
            &truncate_text(&unlock, (rect.w - 16.0) as i32, 1),
            rect.x as i32 + 8,
            rect.y as i32 + 77,
            1,
            if unlocked {
                theme::soft_ink()
            } else {
                theme::red()
            },
        );
    } else {
        font::draw_text(
            Tab::Upgrades.title(),
            rect.x as i32 + 8,
            rect.y as i32 + 6,
            1,
            theme::ink(),
        );
        font::draw_wrapped_limited(
            "Permanent boosts for a brave knight. A few stern requirements must be met before some can be purchased.",
            rect.x as i32 + 8,
            rect.y as i32 + 19,
            (rect.w - 16.0) as i32,
            1,
            3,
            theme::soft_ink(),
        );
        font::draw_text(
            "SELECT ONE TO READ ITS HEROIC SALES PITCH.",
            rect.x as i32 + 8,
            rect.y as i32 + 61,
            1,
            theme::faint_ink(),
        );
        font::draw_text(
            "ARROWS CHOOSE - ENTER BUYS",
            rect.x as i32 + 8,
            rect.y as i32 + 77,
            1,
            theme::pink(),
        );
    }
}

fn draw_achievements_tab(game: &Game) {
    let header = format!(
        "AWARDS  {} / {}",
        game.achievement_count(),
        game.max_achievements()
    );
    font::draw_text(&header, CONTENT_X as i32, 84, 1, theme::ink());
    let bonus = format!(
        "+{} GLOBAL",
        format::percent(game.achievement_bonus() - 1.0)
    );
    font::draw_text_right(&bonus, (CONTENT_X + CONTENT_W) as i32, 84, 1, theme::gold());
    let visible_end = (game.ui.scroll_achievements + ACH_VISIBLE_ROWS).min(game.achievements.len());
    for position in game.ui.scroll_achievements..visible_end {
        let achievement = &game.achievements[position];
        let unlocked = game.unlocked_achievements.contains(achievement.id.as_str());
        let hidden_locked = achievement.hidden && !unlocked;
        let rect = achievement_row_rect(position, game.ui.scroll_achievements);
        let focused = game.ui.focus == FocusId::Achievement(position);
        let fill = if focused {
            theme::pale_gold()
        } else if unlocked {
            theme::unlocked_fill()
        } else {
            theme::panel()
        };
        draw_button_surface(rect, fill, focused);
        if focused {
            draw_focus_heart(rect, theme::pink(), game.clock);
        }
        font::draw_heart(
            rect.x as i32 + 15,
            rect.y as i32 + 7,
            1,
            if unlocked {
                theme::gold()
            } else {
                theme::rule()
            },
        );
        let name = if hidden_locked {
            "???".to_string()
        } else {
            truncate_text(&achievement.name, 165, 1)
        };
        font::draw_text(
            &name,
            rect.x as i32 + 27,
            rect.y as i32 + 7,
            1,
            if unlocked {
                theme::ink()
            } else if hidden_locked {
                theme::faint_ink()
            } else {
                theme::soft_ink()
            },
        );
        let progress = if unlocked {
            "DONE".to_string()
        } else if hidden_locked {
            "SECRET".to_string()
        } else {
            let (value, target) = game.achievement_progress(achievement.condition);
            format!("{} / {}", format::number(value), format::number(target))
        };
        font::draw_text_right(
            &progress,
            (rect.x + rect.w) as i32 - 8,
            rect.y as i32 + 7,
            1,
            if unlocked {
                theme::gold()
            } else {
                theme::faint_ink()
            },
        );
    }
    draw_scrollbar(
        Rect::new(
            CONTENT_X,
            ACH_LIST_TOP,
            CONTENT_W,
            8.0 * ACH_ROW_H + 7.0 * ACH_ROW_GAP,
        ),
        game.achievements.len(),
        ACH_VISIBLE_ROWS,
        game.ui.scroll_achievements,
    );
    draw_list_scroll_hints(
        Rect::new(
            CONTENT_X,
            ACH_LIST_TOP,
            CONTENT_W,
            8.0 * ACH_ROW_H + 7.0 * ACH_ROW_GAP,
        ),
        game.achievements.len(),
        ACH_VISIBLE_ROWS,
        game.ui.scroll_achievements,
    );
    draw_achievement_details(game);
}

fn draw_achievement_details(game: &Game) {
    let rect = Rect::new(CONTENT_X, 288.0, CONTENT_W, 86.0);
    draw_small_panel(rect);
    let selected = match game.ui.focus {
        FocusId::Achievement(index) => Some(index),
        _ => None,
    };
    if let Some(index) = selected {
        let Some(achievement) = game.achievements.get(index) else {
            return;
        };
        let unlocked = game.unlocked_achievements.contains(achievement.id.as_str());
        if achievement.hidden && !unlocked {
            font::draw_text("???", rect.x as i32 + 8, rect.y as i32 + 6, 1, theme::ink());
            font::draw_text(
                "SECRET AWARD",
                rect.x as i32 + 8,
                rect.y as i32 + 18,
                1,
                theme::faint_ink(),
            );
            font::draw_wrapped_limited(
                "This deed is hidden until it is earned. Keep patting, keep dreaming.",
                rect.x as i32 + 8,
                rect.y as i32 + 32,
                (rect.w - 16.0) as i32,
                1,
                2,
                theme::soft_ink(),
            );
            return;
        }
        let name = truncate_text(&achievement.name, 200, 1);
        font::draw_text(
            &name,
            rect.x as i32 + 8,
            rect.y as i32 + 6,
            1,
            if unlocked {
                theme::ink()
            } else {
                theme::soft_ink()
            },
        );
        font::draw_text_right(
            if unlocked { "UNLOCKED" } else { "LOCKED" },
            (rect.x + rect.w) as i32 - 8,
            rect.y as i32 + 6,
            1,
            if unlocked {
                theme::gold()
            } else {
                theme::faint_ink()
            },
        );
        font::draw_text(
            &format!(
                "{}  -  {}",
                achievement.category.label(),
                condition_label(achievement.condition)
            ),
            rect.x as i32 + 8,
            rect.y as i32 + 17,
            1,
            theme::faint_ink(),
        );
        font::draw_wrapped_limited(
            &achievement.description,
            rect.x as i32 + 8,
            rect.y as i32 + 30,
            (rect.w - 16.0) as i32,
            1,
            2,
            theme::soft_ink(),
        );
        let (value, target) = game.achievement_progress(achievement.condition);
        let fraction = if target > 0.0 {
            (value / target).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let progress_text = format!(
            "PROGRESS  {} / {}",
            format::number(value),
            format::number(target)
        );
        font::draw_text(
            &progress_text,
            rect.x as i32 + 8,
            rect.y as i32 + 52,
            1,
            theme::soft_ink(),
        );
        let bar = Rect::new(rect.x + 8.0, rect.y + 65.0, rect.w - 16.0, 7.0);
        draw_rectangle(bar.x, bar.y, bar.w, bar.h, theme::panel());
        draw_rectangle_lines(bar.x, bar.y, bar.w, bar.h, 1.0, theme::ink());
        if fraction > 0.0 {
            draw_rectangle(
                bar.x + 1.0,
                bar.y + 1.0,
                (bar.w - 2.0) * fraction as f32,
                bar.h - 2.0,
                if unlocked {
                    theme::gold()
                } else {
                    theme::pink()
                },
            );
        }
        font::draw_text(
            if unlocked {
                "THIS DEED IS PART OF YOUR LEGEND."
            } else {
                "EVERY PAT BRINGS IT CLOSER."
            },
            rect.x as i32 + 8,
            rect.y as i32 + 76,
            1,
            if unlocked {
                theme::gold()
            } else {
                theme::faint_ink()
            },
        );
    } else {
        font::draw_text(
            Tab::Achievements.title(),
            rect.x as i32 + 8,
            rect.y as i32 + 6,
            1,
            theme::ink(),
        );
        font::draw_wrapped_limited(
            "Every award adds a permanent +0.5% to everything. Hover an award or use the arrows to inspect it.",
            rect.x as i32 + 8,
            rect.y as i32 + 20,
            (rect.w - 16.0) as i32,
            1,
            2,
            theme::soft_ink(),
        );
        font::draw_text(
            "SELECT AN AWARD TO READ ITS TALE.",
            rect.x as i32 + 8,
            rect.y as i32 + 56,
            1,
            theme::faint_ink(),
        );
    }
}

fn draw_stats_tab(game: &Game) {
    let rate = game.production_per_second();
    let rows: Vec<(&str, String)> = vec![
        ("TOTAL PATS", format::number(game.total_clicks as f64)),
        ("AFFECTION EARNED", format::number(game.total_earned)),
        ("PER SECOND", format!("+{}", format::rate(rate))),
        (
            "PER PAT",
            format!("+{}", format::number(game.click_value())),
        ),
        ("PLAYTIME", format::clock(game.stats.playtime)),
        (
            "BUILDINGS OWNED",
            format::number(game.total_buildings() as f64),
        ),
        (
            "UPGRADES OWNED",
            format!("{} / {}", game.owned_upgrades.len(), game.upgrades.len()),
        ),
        (
            "AWARDS",
            format!("{} / {}", game.achievement_count(), game.max_achievements()),
        ),
        ("TOTAL SPENT", format::number(game.stats.total_spent)),
        ("PEAK PAT SPEED", format!("{:.1} P/S", game.stats.peak_cps)),
        (
            "CURRENT PAT SPEED",
            format!("{:.1} P/S", game.pat.current_cps()),
        ),
        ("SESSIONS", format::number(game.stats.sessions as f64)),
        (
            "LAST REST",
            format::duration(game.stats.last_offline_seconds),
        ),
        (
            "DREAM AFFECTION",
            format::number(game.stats.total_offline_earnings),
        ),
    ];
    font::draw_text("THE KNIGHT'S RECORD", CONTENT_X as i32, 84, 1, theme::ink());
    let mut y = 99.0;
    for (index, (label, value)) in rows.iter().enumerate() {
        let row = Rect::new(CONTENT_X, y, CONTENT_W, 17.0);
        if index % 2 == 0 {
            draw_rectangle(row.x, row.y, row.w, row.h, theme::pale_blue());
        }
        let label_text = truncate_text(label, 150, 1);
        font::draw_text(
            &label_text,
            row.x as i32 + 4,
            row.y as i32 + 5,
            1,
            theme::soft_ink(),
        );
        let value_scale = font::fit_scale(value, 112, 1);
        font::draw_text_right(
            value,
            (row.x + row.w) as i32 - 4,
            row.y as i32 + 5,
            value_scale,
            theme::ink(),
        );
        draw_rectangle(row.x, row.y + row.h - 1.0, row.w, 1.0, theme::rule());
        y += 18.0;
    }
    font::draw_text(
        "PAT SPEED IS MEASURED OVER ONE SECOND.",
        CONTENT_X as i32,
        355,
        1,
        theme::faint_ink(),
    );
    font::draw_text(
        "A KNIGHT'S DEEDS OUTLIVE THE INK.",
        CONTENT_X as i32,
        366,
        1,
        theme::pale_pink(),
    );
}

fn draw_settings_tab(game: &Game) {
    let rects = settings_rects();
    draw_slider_row(
        game,
        rects[0],
        0,
        "MUSIC VOLUME",
        game.settings.music_volume,
    );
    draw_slider_row(
        game,
        rects[1],
        1,
        "EFFECTS VOLUME",
        game.settings.effects_volume,
    );
    draw_setting_button(
        game,
        rects[2],
        2,
        "MUTE ALL AUDIO",
        if game.settings.muted {
            "MUTED"
        } else {
            "SOUND ON"
        },
        if game.settings.muted {
            theme::red()
        } else {
            theme::gold()
        },
    );
    draw_setting_button(
        game,
        rects[3],
        3,
        "DARK THEME",
        if game.settings.dark_theme {
            "ON"
        } else {
            "OFF"
        },
        if game.settings.dark_theme {
            theme::gold()
        } else {
            theme::soft_ink()
        },
    );
    draw_setting_button(
        game,
        rects[4],
        4,
        "SAVE JOURNAL",
        "MANUAL SAVE",
        theme::soft_ink(),
    );
    draw_setting_button(game, rects[5], 5, "RESET SAVE", "DANGER", theme::red());
    draw_rectangle(CONTENT_X, 316.0, CONTENT_W, 1.0, theme::rule());
    font::draw_text(
        "AUDIO AND MUSIC ARE SYNTHESIZED IN CODE.",
        CONTENT_X as i32,
        324,
        1,
        theme::soft_ink(),
    );
    let layers = game.current_stage() + 1;
    font::draw_text(
        &format!("MUSIC LAYERS: {} / 4  -  AUTOSAVE: 30S", layers),
        CONTENT_X as i32,
        335,
        1,
        theme::pink(),
    );
    let label = if game.save_using_fallback() {
        "FALLBACK SAVE FILE"
    } else {
        "SAVE FILE"
    };
    font::draw_text(label, CONTENT_X as i32, 350, 1, theme::soft_ink());
    let path = game.save_path_display();
    font::draw_wrapped_limited(
        &path,
        CONTENT_X as i32,
        360,
        CONTENT_W as i32,
        1,
        2,
        theme::faint_ink(),
    );
}

fn draw_slider_row(game: &Game, rect: Rect, index: usize, label: &str, value: f32) {
    let focused = game.ui.focus == FocusId::Setting(index);
    draw_button_surface(
        rect,
        if focused {
            theme::pale_gold()
        } else {
            theme::panel()
        },
        focused,
    );
    if focused {
        draw_focus_heart(rect, theme::pink(), game.clock);
    }
    font::draw_text(
        label,
        rect.x as i32 + 16,
        rect.y as i32 + 6,
        1,
        theme::ink(),
    );
    font::draw_text_right(
        &format::percent(value as f64),
        (rect.x + rect.w) as i32 - 10,
        rect.y as i32 + 6,
        1,
        theme::pink(),
    );
    let track = slider_track(rect);
    draw_rectangle(track.x, track.y, track.w, track.h, theme::rule());
    draw_rectangle_lines(track.x, track.y, track.w, track.h, 1.0, theme::ink());
    if value > 0.0 {
        draw_rectangle(
            track.x + 1.0,
            track.y + 1.0,
            (track.w - 2.0) * value,
            track.h - 2.0,
            theme::pink(),
        );
    }
    let knob_x = track.x + (track.w - 6.0) * value;
    draw_rectangle(knob_x, track.y - 2.0, 6.0, track.h + 4.0, theme::ink());
    if focused {
        font::draw_text(
            "LEFT/RIGHT ADJUST",
            rect.x as i32 + 16,
            rect.y as i32 + 18,
            1,
            theme::faint_ink(),
        );
    }
}

fn draw_setting_button(
    game: &Game,
    rect: Rect,
    index: usize,
    label: &str,
    right: &str,
    right_color: Color,
) {
    let focused = game.ui.focus == FocusId::Setting(index);
    draw_button_surface(
        rect,
        if focused {
            theme::pale_gold()
        } else {
            theme::panel()
        },
        focused,
    );
    if focused {
        draw_focus_heart(
            rect,
            if index == 5 {
                theme::red()
            } else {
                theme::pink()
            },
            game.clock,
        );
    }
    font::draw_text(
        label,
        rect.x as i32 + 16,
        rect.y as i32 + 10,
        1,
        if index == 5 {
            theme::red()
        } else {
            theme::ink()
        },
    );
    font::draw_text_right(
        right,
        (rect.x + rect.w) as i32 - 10,
        rect.y as i32 + 10,
        1,
        right_color,
    );
}

fn draw_reset_modal(game: &Game) {
    let rect = Rect::new(120.0, 152.0, 400.0, 194.0);
    draw_rectangle(rect.x + 7.0, rect.y + 7.0, rect.w, rect.h, theme::shadow());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::panel());
    outline_rect(rect, 4.0, theme::ink());
    draw_rectangle(
        rect.x + 7.0,
        rect.y + 7.0,
        rect.w - 14.0,
        2.0,
        theme::pink(),
    );
    font::draw_text(
        "RESET THE SAVE?",
        rect.x as i32 + 18,
        rect.y as i32 + 18,
        2,
        theme::deep_pink(),
    );
    font::draw_wrapped(
        "This erases all affection, buildings, upgrades, awards, and statistics. Your audio and display settings will be kept. There is no un-patting history.",
        rect.x as i32 + 18,
        rect.y as i32 + 44,
        (rect.w - 36.0) as i32,
        1,
        theme::soft_ink(),
    );
    font::draw_text(
        "THIS IS AN UNDOABLE QUEST.",
        rect.x as i32 + 18,
        rect.y as i32 + 86,
        1,
        theme::red(),
    );
    let buttons = modal_buttons();
    let labels = ["NO, KEEP PLAYING", "YES, START OVER"];
    for (index, button) in buttons.iter().enumerate() {
        let focused = game.ui.reset_choice == index;
        draw_button_surface(
            *button,
            if focused {
                theme::pale_gold()
            } else {
                theme::panel()
            },
            focused,
        );
        if focused {
            draw_focus_heart(
                *button,
                if index == 1 {
                    theme::red()
                } else {
                    theme::pink()
                },
                game.clock,
            );
        }
        font::draw_text_centered(
            labels[index],
            (button.x + button.w / 2.0) as i32,
            button.y as i32 + 12,
            1,
            if index == 1 {
                theme::red()
            } else {
                theme::ink()
            },
        );
    }
    font::draw_text_centered(
        "ARROWS CHOOSE  -  ENTER CONFIRMS  -  ESC CANCELS",
        (rect.x + rect.w / 2.0) as i32,
        rect.y as i32 + 172,
        1,
        theme::faint_ink(),
    );
}

fn draw_toast(game: &Game) {
    let Some(toast) = game.toasts.first() else {
        return;
    };
    let elapsed = (toast.duration - toast.timer).max(0.0);
    let appear = (elapsed / 0.14).clamp(0.0, 1.0);
    let offset = (1.0 - appear) * -8.0;
    let rect = Rect::new(122.0, 5.0 + offset, 506.0, 21.0);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::panel());
    outline_rect(rect, 2.0, theme::ink());
    draw_rectangle(rect.x, rect.y, 5.0, rect.h, theme::gold());
    let prefix = "ACHIEVEMENT ";
    let text_x = rect.x as i32 + 12;
    let text_y = rect.y as i32 + 7;
    font::draw_text(prefix, text_x, text_y, 1, theme::gold());
    let name_x = text_x + font::measure_text(prefix, 1) + 4;
    let available = (rect.x + rect.w) as i32 - name_x - 8;
    let name = truncate_text(&toast.name, available, 1);
    font::draw_text(&name, name_x, text_y, 1, theme::ink());
}

fn draw_particles(game: &Game) {
    for heart in &game.hearts {
        let alpha = (heart.life / heart.max_life).clamp(0.0, 1.0);
        let color = Color {
            r: heart.color.r,
            g: heart.color.g,
            b: heart.color.b,
            a: alpha,
        };
        font::draw_heart(
            heart.x as i32 - 3 * heart.size,
            heart.y as i32 - 3 * heart.size,
            heart.size,
            color,
        );
    }
    for floater in &game.floaters {
        let alpha = (floater.life / floater.max_life).clamp(0.0, 1.0);
        let outline_base = theme::heart_outline();
        let outline = Color {
            r: outline_base.r,
            g: outline_base.g,
            b: outline_base.b,
            a: alpha,
        };
        let fill_base = theme::pink();
        let fill = Color {
            r: fill_base.r,
            g: fill_base.g,
            b: fill_base.b,
            a: alpha,
        };
        let width = font::measure_text(&floater.text, 1);
        let x = floater.x as i32 - width / 2;
        let y = floater.y as i32;
        font::draw_text(&floater.text, x + 1, y + 1, 1, outline);
        font::draw_text(&floater.text, x, y, 1, fill);
    }
}

fn draw_mouse_cursor() {
    let Some(mouse) = game_mouse_position() else {
        return;
    };
    let x = mouse.x as i32 - 3;
    let y = mouse.y as i32 - 3;
    font::draw_heart(x + 1, y + 1, 1, theme::ink());
    font::draw_heart(x, y, 1, theme::pink());
}

fn draw_notice(game: &Game) {
    let Some(notice) = &game.notice else {
        return;
    };
    let color = if notice.error {
        theme::red()
    } else {
        theme::soft_ink()
    };
    let lines = font::wrap_text(&notice.text, 252, 1);
    for (index, line) in lines.iter().take(2).enumerate() {
        font::draw_text_right(line, 628, 8 + index as i32 * 10, 1, color);
    }
}

fn draw_panel(rect: Rect) {
    draw_rectangle(rect.x + 4.0, rect.y + 4.0, rect.w, rect.h, theme::shadow());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::panel());
    outline_rect(rect, 3.0, theme::ink());
    draw_rectangle(rect.x + 3.0, rect.y + 3.0, rect.w - 6.0, 2.0, theme::rule());
}

fn draw_small_panel(rect: Rect) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::panel());
    outline_rect(rect, 2.0, theme::ink());
}

fn outline_rect(rect: Rect, thickness: f32, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, thickness, color);
    draw_rectangle(
        rect.x,
        rect.y + rect.h - thickness,
        rect.w,
        thickness,
        color,
    );
    draw_rectangle(rect.x, rect.y, thickness, rect.h, color);
    draw_rectangle(
        rect.x + rect.w - thickness,
        rect.y,
        thickness,
        rect.h,
        color,
    );
}

fn draw_button_surface(rect: Rect, fill: Color, focused: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    outline_rect(rect, 2.0, theme::ink());
    if focused {
        draw_rectangle_lines(
            rect.x - 2.0,
            rect.y - 2.0,
            rect.w + 4.0,
            rect.h + 4.0,
            2.0,
            theme::pink(),
        );
    }
}

fn draw_focus_heart(rect: Rect, color: Color, clock: f64) {
    let bob = ((clock * 5.0).sin() * 1.0) as i32;
    let x = rect.x as i32 + 5;
    let y = (rect.y + rect.h / 2.0) as i32 - 3 + bob;
    font::draw_heart(x + 1, y + 1, 1, theme::ink());
    font::draw_heart(x, y, 1, color);
}

fn draw_scrollbar(area: Rect, total: usize, visible: usize, scroll: usize) {
    if total <= visible {
        return;
    }
    let x = (CONTENT_X + CONTENT_W + 5.0) as i32;
    let top = area.y as i32;
    let height = area.h as i32;
    draw_rectangle(x as f32, top as f32, 2.0, height as f32, theme::rule());
    let thumb_height = ((visible as f32 / total as f32) * height as f32).max(10.0);
    let max_scroll = total.saturating_sub(visible).max(1);
    let thumb_top = top
        + (((scroll as f32 / max_scroll as f32) * (height as f32 - thumb_height)).round() as i32);
    draw_rectangle(x as f32, thumb_top as f32, 2.0, thumb_height, theme::ink());
}

fn draw_list_scroll_hints(area: Rect, total: usize, visible: usize, scroll: usize) {
    if total <= visible {
        return;
    }
    if scroll > 0 {
        let x = (area.x + area.w - 14.0) as i32;
        let y = area.y as i32 + 2;
        draw_rectangle(x as f32, y as f32, 7.0, 1.0, theme::ink());
        draw_rectangle(x as f32 + 1.0, y as f32 + 1.0, 5.0, 1.0, theme::ink());
        draw_rectangle(x as f32 + 2.0, y as f32 + 2.0, 3.0, 1.0, theme::ink());
        draw_rectangle(x as f32 + 3.0, y as f32 + 3.0, 1.0, 1.0, theme::ink());
    }
    if scroll + visible < total {
        let x = (area.x + area.w - 14.0) as i32;
        let y = (area.y + area.h - 5.0) as i32;
        draw_rectangle(x as f32, y as f32 + 4.0, 7.0, 1.0, theme::ink());
        draw_rectangle(x as f32 + 1.0, y as f32 + 3.0, 5.0, 1.0, theme::ink());
        draw_rectangle(x as f32 + 2.0, y as f32 + 2.0, 3.0, 1.0, theme::ink());
        draw_rectangle(x as f32 + 3.0, y as f32 + 1.0, 1.0, 1.0, theme::ink());
    }
}

fn truncate_text(text: &str, max_width: i32, scale: i32) -> String {
    if font::measure_text(text, scale) <= max_width {
        return text.to_string();
    }
    let mut result = String::new();
    for character in text.chars() {
        let mut candidate = result.clone();
        candidate.push(character);
        candidate.push_str("...");
        if font::measure_text(&candidate, scale) > max_width {
            break;
        }
        result.push(character);
    }
    result.push_str("...");
    result
}
