#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod achievements;
mod audio;
mod buildings;
mod dialogue;
mod font;
mod format;
mod game;
mod pat_animation;
mod save;
mod theme;
mod ui;
mod upgrades;

use image::ImageFormat;
use macroquad::prelude::*;

const ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

fn window_icon() -> Option<macroquad::miniquad::conf::Icon> {
    let image = image::load_from_memory_with_format(ICON_BYTES, ImageFormat::Ico)
        .ok()?
        .to_rgba8();
    let small = image::imageops::resize(&image, 16, 16, image::imageops::FilterType::Lanczos3);
    let medium = image::imageops::resize(&image, 32, 32, image::imageops::FilterType::Lanczos3);
    let big = image::imageops::resize(&image, 64, 64, image::imageops::FilterType::Lanczos3);
    let mut icon = macroquad::miniquad::conf::Icon {
        small: [0; 16 * 16 * 4],
        medium: [0; 32 * 32 * 4],
        big: [0; 64 * 64 * 4],
    };
    icon.small.copy_from_slice(small.as_raw());
    icon.medium.copy_from_slice(medium.as_raw());
    icon.big.copy_from_slice(big.as_raw());
    Some(icon)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Quixotic".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        icon: window_icon(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    prevent_quit();
    let mut game = game::Game::new().await;
    let render_target = render_target(ui::SCREEN_WIDTH as u32, ui::SCREEN_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    loop {
        if is_quit_requested() {
            break;
        }
        let delta = get_frame_time() as f64;
        game.update(delta);
        ui::update(&mut game);
        let background = theme::background(&game.settings);
        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, ui::SCREEN_WIDTH, ui::SCREEN_HEIGHT));
        camera.render_target = Some(render_target.clone());
        set_camera(&camera);
        clear_background(background);
        ui::draw(&game);
        set_default_camera();
        clear_background(background);
        let (viewport, _) = ui::screen_viewport();
        draw_texture_ex(
            &render_target.texture,
            viewport.x,
            viewport.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(viewport.w, viewport.h)),
                flip_y: true,
                ..Default::default()
            },
        );
        next_frame().await;
    }
    game.save_on_exit();
}
