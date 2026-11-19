use std::cell::RefCell;

use macroquad::prelude::Color;

use crate::save::Settings;

#[derive(Clone, Copy)]
struct Palette {
    background: Color,
    panel: Color,
    ink: Color,
    soft_ink: Color,
    faint_ink: Color,
    pink: Color,
    deep_pink: Color,
    gold: Color,
    pale_gold: Color,
    pale_pink: Color,
    pale_blue: Color,
    rule: Color,
    shadow: Color,
    dim: Color,
    red: Color,
    owned_fill: Color,
    unlocked_fill: Color,
    heart_outline: Color,
}

impl Palette {
    fn light() -> Self {
        Palette {
            background: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            panel: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            ink: Color {
                r: 0.05,
                g: 0.05,
                b: 0.08,
                a: 1.0,
            },
            soft_ink: Color {
                r: 0.34,
                g: 0.34,
                b: 0.38,
                a: 1.0,
            },
            faint_ink: Color {
                r: 0.60,
                g: 0.60,
                b: 0.64,
                a: 1.0,
            },
            pink: Color {
                r: 0.93,
                g: 0.20,
                b: 0.34,
                a: 1.0,
            },
            deep_pink: Color {
                r: 0.72,
                g: 0.06,
                b: 0.19,
                a: 1.0,
            },
            gold: Color {
                r: 0.91,
                g: 0.63,
                b: 0.10,
                a: 1.0,
            },
            pale_gold: Color {
                r: 1.0,
                g: 0.96,
                b: 0.80,
                a: 1.0,
            },
            pale_pink: Color {
                r: 1.0,
                g: 0.89,
                b: 0.92,
                a: 1.0,
            },
            pale_blue: Color {
                r: 0.95,
                g: 0.97,
                b: 1.0,
                a: 1.0,
            },
            rule: Color {
                r: 0.85,
                g: 0.85,
                b: 0.87,
                a: 1.0,
            },
            shadow: Color {
                r: 0.74,
                g: 0.74,
                b: 0.77,
                a: 0.55,
            },
            dim: Color {
                r: 0.52,
                g: 0.52,
                b: 0.57,
                a: 1.0,
            },
            red: Color {
                r: 0.75,
                g: 0.10,
                b: 0.18,
                a: 1.0,
            },
            owned_fill: Color {
                r: 0.96,
                g: 0.96,
                b: 0.96,
                a: 1.0,
            },
            unlocked_fill: Color {
                r: 0.99,
                g: 0.97,
                b: 0.88,
                a: 1.0,
            },
            heart_outline: Color {
                r: 0.05,
                g: 0.05,
                b: 0.08,
                a: 1.0,
            },
        }
    }

    fn dark() -> Self {
        Palette {
            background: Color {
                r: 0.035,
                g: 0.035,
                b: 0.055,
                a: 1.0,
            },
            panel: Color {
                r: 0.085,
                g: 0.085,
                b: 0.115,
                a: 1.0,
            },
            ink: Color {
                r: 0.94,
                g: 0.94,
                b: 0.97,
                a: 1.0,
            },
            soft_ink: Color {
                r: 0.70,
                g: 0.70,
                b: 0.76,
                a: 1.0,
            },
            faint_ink: Color {
                r: 0.48,
                g: 0.48,
                b: 0.56,
                a: 1.0,
            },
            pink: Color {
                r: 1.0,
                g: 0.38,
                b: 0.53,
                a: 1.0,
            },
            deep_pink: Color {
                r: 1.0,
                g: 0.66,
                b: 0.76,
                a: 1.0,
            },
            gold: Color {
                r: 0.97,
                g: 0.73,
                b: 0.30,
                a: 1.0,
            },
            pale_gold: Color {
                r: 0.22,
                g: 0.18,
                b: 0.08,
                a: 1.0,
            },
            pale_pink: Color {
                r: 0.25,
                g: 0.11,
                b: 0.17,
                a: 1.0,
            },
            pale_blue: Color {
                r: 0.10,
                g: 0.13,
                b: 0.19,
                a: 1.0,
            },
            rule: Color {
                r: 0.24,
                g: 0.24,
                b: 0.30,
                a: 1.0,
            },
            shadow: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.55,
            },
            dim: Color {
                r: 0.46,
                g: 0.46,
                b: 0.54,
                a: 1.0,
            },
            red: Color {
                r: 1.0,
                g: 0.37,
                b: 0.43,
                a: 1.0,
            },
            owned_fill: Color {
                r: 0.12,
                g: 0.12,
                b: 0.15,
                a: 1.0,
            },
            unlocked_fill: Color {
                r: 0.17,
                g: 0.15,
                b: 0.08,
                a: 1.0,
            },
            heart_outline: Color {
                r: 0.015,
                g: 0.015,
                b: 0.025,
                a: 1.0,
            },
        }
    }
}

fn palette_for(settings: &Settings) -> Palette {
    if settings.dark_theme {
        Palette::dark()
    } else {
        Palette::light()
    }
}

thread_local! {
    static ACTIVE: RefCell<Palette> = RefCell::new(Palette::light());
}

pub fn apply(settings: &Settings) {
    ACTIVE.with(|active| {
        *active.borrow_mut() = palette_for(settings);
    });
}

pub fn background(settings: &Settings) -> Color {
    palette_for(settings).background
}

fn current() -> Palette {
    ACTIVE.with(|active| *active.borrow())
}

pub fn panel() -> Color {
    current().panel
}

pub fn ink() -> Color {
    current().ink
}

pub fn soft_ink() -> Color {
    current().soft_ink
}

pub fn faint_ink() -> Color {
    current().faint_ink
}

pub fn pink() -> Color {
    current().pink
}

pub fn deep_pink() -> Color {
    current().deep_pink
}

pub fn gold() -> Color {
    current().gold
}

pub fn pale_gold() -> Color {
    current().pale_gold
}

pub fn pale_pink() -> Color {
    current().pale_pink
}

pub fn pale_blue() -> Color {
    current().pale_blue
}

pub fn rule() -> Color {
    current().rule
}

pub fn shadow() -> Color {
    current().shadow
}

pub fn dim() -> Color {
    current().dim
}

pub fn red() -> Color {
    current().red
}

pub fn owned_fill() -> Color {
    current().owned_fill
}

pub fn unlocked_fill() -> Color {
    current().unlocked_fill
}

pub fn heart_outline() -> Color {
    current().heart_outline
}
