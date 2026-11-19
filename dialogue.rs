use macroquad::prelude::*;

use crate::font;
use crate::theme;

pub const TEXT_WIDTH: i32 = 566;

pub struct DialogueBox {
    lines: Vec<String>,
    flat: Vec<char>,
    revealed: usize,
    accumulator: f32,
    char_interval: f32,
    line_spacing: i32,
    blink_clock: f32,
}

impl DialogueBox {
    pub fn new() -> Self {
        let mut dialogue = DialogueBox {
            lines: Vec::new(),
            flat: Vec::new(),
            revealed: 0,
            accumulator: 0.0,
            char_interval: 0.025,
            line_spacing: 9,
            blink_clock: 0.0,
        };
        dialogue.say("Ah! A visitor! Come closer, brave friend. My helmet is polished and my heart is ready!");
        dialogue
    }

    pub fn say(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.lines = font::wrap_text(&text, TEXT_WIDTH, 1);
        self.flat = self.lines.iter().flat_map(|line| line.chars()).collect();
        self.revealed = 0;
        self.accumulator = 0.0;
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.blink_clock += dt;
        if self.revealed >= self.flat.len() {
            return false;
        }
        self.accumulator += dt;
        let mut blip = false;
        let mut revealed_this_frame = 0;
        while self.accumulator >= self.char_interval
            && self.revealed < self.flat.len()
            && revealed_this_frame < 8
        {
            self.accumulator -= self.char_interval;
            let character = self.flat[self.revealed];
            self.revealed += 1;
            revealed_this_frame += 1;
            if !character.is_whitespace() {
                blip = true;
            }
        }
        blip
    }

    pub fn finish(&mut self) {
        self.revealed = self.flat.len();
        self.accumulator = 0.0;
    }

    pub fn is_complete(&self) -> bool {
        self.revealed >= self.flat.len()
    }

    pub fn draw(&self, rect: Rect) {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::panel());
        draw_rectangle(rect.x, rect.y, rect.w, 3.0, theme::ink());
        draw_rectangle(rect.x, rect.y + rect.h - 3.0, rect.w, 3.0, theme::ink());
        draw_rectangle(rect.x, rect.y, 3.0, rect.h, theme::ink());
        draw_rectangle(rect.x + rect.w - 3.0, rect.y, 3.0, rect.h, theme::ink());
        draw_rectangle(
            rect.x + 7.0,
            rect.y + 7.0,
            rect.w - 14.0,
            1.0,
            theme::rule(),
        );
        let label_x = rect.x as i32 + 14;
        let label_y = rect.y as i32 + 9;
        draw_rectangle(
            label_x as f32 - 5.0,
            label_y as f32 - 3.0,
            83.0,
            14.0,
            theme::ink(),
        );
        font::draw_text("DON QUIXOTE", label_x, label_y, 1, theme::panel());
        font::draw_text(
            "ARROWS NAVIGATE   ENTER CONFIRM   SPACE PATS",
            rect.x as i32 + 136,
            label_y,
            1,
            theme::soft_ink(),
        );
        let mut consumed = 0usize;
        for (line_index, line) in self.lines.iter().enumerate() {
            if consumed >= self.revealed {
                break;
            }
            let line_length = line.chars().count();
            let take = (self.revealed - consumed).min(line_length);
            let shown: String = line.chars().take(take).collect();
            font::draw_text(
                &shown,
                rect.x as i32 + 18,
                rect.y as i32 + 28 + line_index as i32 * self.line_spacing,
                1,
                theme::ink(),
            );
            consumed += line_length;
        }
        if self.is_complete() && (self.blink_clock * 2.0).sin() > -0.25 {
            font::draw_heart(
                rect.x as i32 + rect.w as i32 - 24,
                rect.y as i32 + rect.h as i32 - 17,
                1,
                theme::pink(),
            );
        }
    }
}
