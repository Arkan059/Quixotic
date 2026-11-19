use std::collections::VecDeque;
use std::io::Cursor;

use image::{AnimationDecoder, ImageFormat};
use macroquad::prelude::*;

use crate::font;

const IDLE_BYTES: &[u8] = include_bytes!("../don_quixote.png");
const ANIMATION_BYTES: &[u8] = include_bytes!("../pet.gif");
const CLICK_WINDOW: f64 = 1.0;
const ACTIVE_SECONDS: f32 = 1.05;

pub struct AnimationFrame {
    texture: Texture2D,
    delay_ms: f32,
}

pub struct PatAnimation {
    idle: Option<Texture2D>,
    frames: Vec<AnimationFrame>,
    current_frame: usize,
    frame_progress_ms: f32,
    speed: f32,
    active: bool,
    active_timer: f32,
    squash: f32,
    clock: f64,
    click_stamps: VecDeque<f64>,
    current_cps: f64,
    failed_assets: bool,
    animation_bob: f32,
}

fn load_idle() -> Option<Texture2D> {
    let image = image::load_from_memory_with_format(IDLE_BYTES, ImageFormat::Png)
        .ok()?
        .to_rgba8();
    let width = image.width();
    let height = image.height();
    if width == 0 || height == 0 {
        return None;
    }
    let target = 256u32;
    let resized = image::imageops::resize(
        &image,
        target,
        target,
        image::imageops::FilterType::Lanczos3,
    );
    let texture = Texture2D::from_rgba8(target as u16, target as u16, resized.as_raw());
    texture.set_filter(FilterMode::Linear);
    Some(texture)
}

fn load_frames() -> Vec<AnimationFrame> {
    let mut frames = Vec::new();
    let decoder = match image::codecs::gif::GifDecoder::new(Cursor::new(ANIMATION_BYTES)) {
        Ok(decoder) => decoder,
        Err(_) => return frames,
    };
    let decoded = match decoder.into_frames().collect_frames() {
        Ok(frames) => frames,
        Err(_) => return frames,
    };
    for frame in decoded {
        let (numerator, denominator) = frame.delay().numer_denom_ms();
        let mut delay = numerator as f32 / denominator.max(1) as f32;
        if !delay.is_finite() || delay <= 5.0 {
            delay = 80.0;
        }
        let buffer = frame.into_buffer();
        let width = buffer.width();
        let height = buffer.height();
        if width == 0 || height == 0 {
            continue;
        }
        let texture = Texture2D::from_rgba8(width as u16, height as u16, buffer.as_raw());
        texture.set_filter(FilterMode::Nearest);
        frames.push(AnimationFrame {
            texture,
            delay_ms: delay,
        });
    }
    frames
}

impl PatAnimation {
    pub fn load() -> Self {
        let idle = load_idle();
        let frames = load_frames();
        let failed_assets = idle.is_none() && frames.is_empty();
        PatAnimation {
            idle,
            frames,
            current_frame: 0,
            frame_progress_ms: 0.0,
            speed: 0.5,
            active: false,
            active_timer: 0.0,
            squash: 0.0,
            clock: 0.0,
            click_stamps: VecDeque::new(),
            current_cps: 0.0,
            failed_assets,
            animation_bob: 0.0,
        }
    }

    pub fn note_click(&mut self) {
        if self.active_timer <= 0.0 {
            self.current_frame = 0;
            self.frame_progress_ms = 0.0;
        }
        self.active = true;
        self.active_timer = ACTIVE_SECONDS;
        self.squash = 1.0;
        self.click_stamps.push_back(self.clock);
        while self
            .click_stamps
            .front()
            .map(|stamp| self.clock - *stamp > CLICK_WINDOW)
            .unwrap_or(false)
        {
            self.click_stamps.pop_front();
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.clock += dt as f64;
        while self
            .click_stamps
            .front()
            .map(|stamp| self.clock - *stamp > CLICK_WINDOW)
            .unwrap_or(false)
        {
            self.click_stamps.pop_front();
        }
        self.current_cps = self.click_stamps.len() as f64;
        let cps = self.current_cps as f32;
        let target_speed = if self.active {
            0.5 + ((cps - 1.0).max(0.0) / 9.0).clamp(0.0, 1.0) * 3.5
        } else {
            0.5
        };
        let blend = 1.0 - (-dt * 6.0).exp();
        self.speed += (target_speed - self.speed) * blend;
        if self.active_timer > 0.0 {
            self.active_timer -= dt;
            if self.active_timer <= 0.0 {
                self.active = false;
                self.active_timer = 0.0;
                self.current_frame = 0;
                self.frame_progress_ms = 0.0;
            }
        }
        if self.active && !self.frames.is_empty() {
            self.frame_progress_ms += dt * 1000.0 * self.speed.max(0.05);
            let mut guard = 0;
            while guard < self.frames.len() * 4 {
                let delay = self.frames[self.current_frame].delay_ms.max(1.0);
                if self.frame_progress_ms < delay {
                    break;
                }
                self.frame_progress_ms -= delay;
                self.current_frame = (self.current_frame + 1) % self.frames.len();
                guard += 1;
            }
        }
        self.squash *= (-dt * 9.0).exp();
        if self.squash < 0.001 {
            self.squash = 0.0;
        }
        if self.active {
            self.animation_bob = (self.clock as f32 * 18.0).sin() * 1.2 * self.speed.min(2.0);
        } else {
            self.animation_bob *= (1.0 - dt * 8.0).clamp(0.0, 1.0);
        }
    }

    pub fn current_cps(&self) -> f64 {
        self.current_cps
    }

    pub fn speed_multiplier(&self) -> f32 {
        self.speed
    }

    pub fn is_animated(&self) -> bool {
        self.active && !self.frames.is_empty()
    }

    pub fn draw(&self, area: Rect) {
        if self.failed_assets {
            draw_rectangle(area.x, area.y, area.w, area.h, WHITE);
            draw_rectangle(
                area.x,
                area.y,
                area.w,
                3.0,
                Color::new(0.05, 0.05, 0.08, 1.0),
            );
            draw_rectangle(
                area.x,
                area.y + area.h - 3.0,
                area.w,
                3.0,
                Color::new(0.05, 0.05, 0.08, 1.0),
            );
            draw_rectangle(
                area.x,
                area.y,
                3.0,
                area.h,
                Color::new(0.05, 0.05, 0.08, 1.0),
            );
            draw_rectangle(
                area.x + area.w - 3.0,
                area.y,
                3.0,
                area.h,
                Color::new(0.05, 0.05, 0.08, 1.0),
            );
            draw_rectangle(
                area.x + 24.0,
                area.y + area.h - 40.0,
                area.w - 48.0,
                2.0,
                Color::new(0.8, 0.2, 0.3, 1.0),
            );
            font::draw_text_centered(
                "DON QUIXOTE",
                (area.x + area.w / 2.0) as i32,
                (area.y + area.h / 2.0 - 18.0) as i32,
                2,
                Color::new(0.05, 0.05, 0.08, 1.0),
            );
            font::draw_text_centered(
                "PORTRAIT MISSING",
                (area.x + area.w / 2.0) as i32,
                (area.y + area.h / 2.0 + 8.0) as i32,
                1,
                Color::new(0.55, 0.55, 0.55, 1.0),
            );
            return;
        }
        let texture = if self.is_animated() {
            self.frames
                .get(self.current_frame)
                .map(|frame| &frame.texture)
        } else {
            self.idle.as_ref()
        };
        let Some(texture) = texture.or_else(|| self.frames.first().map(|frame| &frame.texture))
        else {
            return;
        };
        let squash_x = 1.0 + self.squash * 0.075;
        let squash_y = 1.0 - self.squash * 0.05;
        let draw_width = area.w * squash_x;
        let draw_height = area.h * squash_y;
        let x = area.x + (area.w - draw_width) / 2.0;
        let y = area.y + area.h - draw_height + self.animation_bob;
        draw_texture_ex(
            texture,
            x.round(),
            y.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    draw_width.round().max(1.0),
                    draw_height.round().max(1.0),
                )),
                ..Default::default()
            },
        );
    }
}
