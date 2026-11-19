use macroquad::prelude::*;

pub const GLYPH_WIDTH: i32 = 5;
pub const GLYPH_HEIGHT: i32 = 7;
pub const LETTER_SPACING: i32 = 1;
pub const HEART: char = '\u{2665}';

pub fn line_height(scale: i32) -> i32 {
    (GLYPH_HEIGHT + 2) * scale.max(1)
}

pub fn glyph(c: char) -> [u8; 7] {
    match c {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        'a' => [
            0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111,
        ],
        'b' => [
            0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'c' => [
            0b00000, 0b00000, 0b01110, 0b10001, 0b10000, 0b10001, 0b01110,
        ],
        'd' => [
            0b00001, 0b00001, 0b01111, 0b10001, 0b10001, 0b10001, 0b01111,
        ],
        'e' => [
            0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110,
        ],
        'f' => [
            0b00110, 0b01000, 0b11110, 0b01000, 0b01000, 0b01000, 0b01000,
        ],
        'g' => [
            0b00000, 0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001,
        ],
        'h' => [
            0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b10001,
        ],
        'i' => [
            0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'j' => [
            0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'k' => [
            0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010,
        ],
        'l' => [
            0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00110,
        ],
        'm' => [
            0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10101, 0b10101,
        ],
        'n' => [
            0b00000, 0b00000, 0b11110, 0b10001, 0b10001, 0b10001, 0b10001,
        ],
        'o' => [
            0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'p' => [
            0b00000, 0b00000, 0b11110, 0b10001, 0b10001, 0b11110, 0b10000,
        ],
        'q' => [
            0b00000, 0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001,
        ],
        'r' => [
            0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000,
        ],
        's' => [
            0b00000, 0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110,
        ],
        't' => [
            0b01000, 0b11100, 0b01000, 0b01000, 0b01000, 0b01001, 0b00110,
        ],
        'u' => [
            0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10001, 0b01111,
        ],
        'v' => [
            0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'w' => [
            0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010,
        ],
        'x' => [
            0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001,
        ],
        'y' => [
            0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01111, 0b00001,
        ],
        'z' => [
            0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        '6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100,
        ],
        ',' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b01000,
        ],
        '!' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
        '?' => [
            0b01110, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
        ':' => [
            0b00000, 0b00000, 0b00100, 0b00000, 0b00000, 0b00100, 0b00000,
        ],
        ';' => [
            0b00000, 0b00000, 0b00100, 0b00000, 0b00000, 0b00100, 0b01000,
        ],
        '\'' => [
            0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        '"' => [
            0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '+' => [
            0b00000, 0b00000, 0b00100, 0b11111, 0b00100, 0b00000, 0b00000,
        ],
        '*' => [
            0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000,
        ],
        '/' => [
            0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000,
        ],
        '%' => [
            0b11001, 0b11010, 0b00100, 0b00100, 0b01011, 0b10011, 0b00000,
        ],
        '(' => [
            0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
        ],
        ')' => [
            0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
        ],
        '[' => [
            0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110,
        ],
        ']' => [
            0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110,
        ],
        '<' => [
            0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010,
        ],
        '>' => [
            0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000,
        ],
        '=' => [
            0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000,
        ],
        '&' => [
            0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        HEART => [
            0b01010, 0b11111, 0b11111, 0b11111, 0b01110, 0b00100, 0b00000,
        ],
        _ => [
            0b01110, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
    }
}

pub fn draw_glyph(c: char, x: i32, y: i32, scale: i32, color: Color) {
    if scale <= 0 {
        return;
    }
    let rows = glyph(c);
    for (row, bits) in rows.iter().enumerate() {
        let mut start: Option<i32> = None;
        for column in 0..GLYPH_WIDTH {
            let on = bits & (1 << (GLYPH_WIDTH - 1 - column)) != 0;
            match (on, start) {
                (true, None) => start = Some(column),
                (false, Some(begin)) => {
                    let run = column - begin;
                    draw_rectangle(
                        (x + begin * scale) as f32,
                        (y + row as i32 * scale) as f32,
                        (run * scale) as f32,
                        scale as f32,
                        color,
                    );
                    start = None;
                }
                _ => {}
            }
        }
        if let Some(begin) = start {
            let run = GLYPH_WIDTH - begin;
            draw_rectangle(
                (x + begin * scale) as f32,
                (y + row as i32 * scale) as f32,
                (run * scale) as f32,
                scale as f32,
                color,
            );
        }
    }
}

pub fn measure_line(text: &str, scale: i32) -> i32 {
    let scale = scale.max(1);
    let count = text.chars().count() as i32;
    if count == 0 {
        return 0;
    }
    count * (GLYPH_WIDTH + LETTER_SPACING) * scale - LETTER_SPACING * scale
}

pub fn measure_text(text: &str, scale: i32) -> i32 {
    text.split('\n')
        .map(|line| measure_line(line, scale))
        .max()
        .unwrap_or(0)
}

pub fn draw_text(text: &str, x: i32, y: i32, scale: i32, color: Color) {
    let scale = scale.max(1);
    let mut cursor_y = y;
    for line in text.split('\n') {
        let mut cursor_x = x;
        for c in line.chars() {
            if c != ' ' {
                draw_glyph(c, cursor_x, cursor_y, scale, color);
            }
            cursor_x += (GLYPH_WIDTH + LETTER_SPACING) * scale;
        }
        cursor_y += line_height(scale);
    }
}

pub fn draw_text_centered(text: &str, center_x: i32, y: i32, scale: i32, color: Color) {
    let width = measure_text(text, scale);
    draw_text(text, center_x - width / 2, y, scale, color);
}

pub fn draw_text_right(text: &str, right_x: i32, y: i32, scale: i32, color: Color) {
    let width = measure_text(text, scale);
    draw_text(text, right_x - width, y, scale, color);
}

pub fn fit_scale(text: &str, max_width: i32, max_scale: i32) -> i32 {
    let mut scale = max_scale.max(1);
    while scale > 1 && measure_text(text, scale) > max_width {
        scale -= 1;
    }
    scale
}

fn split_long_word(word: &str, max_width: i32, scale: i32) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for c in word.chars() {
        let mut candidate = current.clone();
        candidate.push(c);
        if !current.is_empty() && measure_line(&candidate, scale) > max_width {
            chunks.push(current);
            current = String::new();
            current.push(c);
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

pub fn wrap_text(text: &str, max_width: i32, scale: i32) -> Vec<String> {
    let scale = scale.max(1);
    let max_width = max_width.max(1);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        if words.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        for word in words {
            if measure_line(word, scale) > max_width {
                if !current.is_empty() {
                    lines.push(current.clone());
                    current.clear();
                }
                let pieces = split_long_word(word, max_width, scale);
                let last = pieces.len().saturating_sub(1);
                for (index, piece) in pieces.into_iter().enumerate() {
                    if index == last {
                        current = piece;
                    } else {
                        lines.push(piece);
                    }
                }
                continue;
            }
            if current.is_empty() {
                current.push_str(word);
            } else {
                let mut candidate = current.clone();
                candidate.push(' ');
                candidate.push_str(word);
                if measure_line(&candidate, scale) <= max_width {
                    current = candidate;
                } else {
                    lines.push(current.clone());
                    current.clear();
                    current.push_str(word);
                }
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

pub fn draw_wrapped(text: &str, x: i32, y: i32, max_width: i32, scale: i32, color: Color) -> i32 {
    let lines = wrap_text(text, max_width, scale);
    let step = line_height(scale);
    for (index, line) in lines.iter().enumerate() {
        draw_text(line, x, y + index as i32 * step, scale, color);
    }
    lines.len() as i32 * step
}

pub fn draw_wrapped_limited(
    text: &str,
    x: i32,
    y: i32,
    max_width: i32,
    scale: i32,
    max_lines: usize,
    color: Color,
) {
    let lines = wrap_text(text, max_width, scale);
    let step = line_height(scale);
    for (index, line) in lines.iter().take(max_lines).enumerate() {
        draw_text(line, x, y + index as i32 * step, scale, color);
    }
}

pub fn draw_heart(x: i32, y: i32, scale: i32, color: Color) {
    const HEART_MASK: [u8; 7] = [
        0b0110110, 0b1111111, 0b1111111, 0b1111111, 0b0111110, 0b0011100, 0b0001000,
    ];
    if scale <= 0 {
        return;
    }
    for (row, bits) in HEART_MASK.iter().enumerate() {
        let mut start: Option<i32> = None;
        for column in 0..7 {
            let on = bits & (1 << (6 - column)) != 0;
            match (on, start) {
                (true, None) => start = Some(column),
                (false, Some(begin)) => {
                    let run = column - begin;
                    draw_rectangle(
                        (x + begin * scale) as f32,
                        (y + row as i32 * scale) as f32,
                        (run * scale) as f32,
                        scale as f32,
                        color,
                    );
                    start = None;
                }
                _ => {}
            }
        }
        if let Some(begin) = start {
            let run = 7 - begin;
            draw_rectangle(
                (x + begin * scale) as f32,
                (y + row as i32 * scale) as f32,
                (run * scale) as f32,
                scale as f32,
                color,
            );
        }
    }
}
