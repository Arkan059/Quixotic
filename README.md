![Don Quixote being patted](pet.gif)

# Quixotic

Quixotic is a small, cheerful fan-made incremental game about showing Don Quixote the affection she so richly deserves. Pat the plush knight, gather Affection, recruit a delightfully impractical kingdom, and grow an increasingly absurd legend one gentle pat at a time.

Everything in the game is original and created for this project. The interface and text are drawn with a built-in bitmap pixel font, the visuals are composed from simple code-drawn shapes plus the supplied character art, and every piece of music and every sound effect is synthesized procedurally at runtime.

## Disclaimer

This is an unofficial fan game made purely for fun. It is not affiliated with, sponsored by, or endorsed by Project Moon, Limbus Company, the creators of Undertale and Deltarune, or any related rights holders. It is a non-commercial project. All rights to the original characters, settings, and associated intellectual property belong to their respective owners.

No copyrighted music, sprites, dialogue, or other game assets from Limbus Company, Undertale, or Deltarune are reproduced by the code in this project. The visual style takes broad inspiration from classic pixel RPG menus and dialogue boxes, but all melodies, jokes, names, text, and code in Quixotic are original.

## Build and Run

Requirements:

- A current stable Rust toolchain with Cargo.
- Windows: the Microsoft C++ build tools and Windows SDK, which provide the resource compiler used to embed icon.ico in the executable.
- macOS: the Xcode command line tools.
- Linux: the usual native development packages for Macroquad. On Debian or Ubuntu, install libasound2-dev libx11-dev libxi-dev and libgl1-mesa-dev. Other distributions need the equivalent ALSA, X11, and OpenGL development packages.

Place these three files in the project root, next to Cargo.toml:

- don_quixote.png
- pet.gif
- icon.ico

Then build and run:

    cargo run --release

The first build downloads the Rust dependencies and compiles Macroquad, image decoding, and the rest of the game. The game assets are embedded into the built executable, so after the build the game is self-contained.

To build only:

    cargo build --release

The Windows executable is target/release/quixotic.exe and receives icon.ico through build.rs. The same icon is also applied as the live window icon at runtime.

## Controls

- Click Don Quixote to pat her.
- Space also pats Don Quixote.
- Arrow keys move the heart cursor between the tabs, purchase buttons, list rows, and settings controls.
- Enter or Z confirms the focused option.
- Left and Right adjust a focused volume slider.
- The mouse wheel scrolls the building, upgrade, and award lists.
- Click the amount buttons to choose buy 1, buy 10, buy 100, or buy Max.
- Click a building to purchase the selected amount.
- Click an upgrade to purchase it.
- Click the dialogue box to reveal the current line immediately.
- In Settings, use DARK THEME to switch between the light and dark themes.
- Escape closes the reset confirmation without resetting anything.

## Features

- A complete pat-based incremental game built around one very patient plush knight.
- 26 affection-producing buildings, from the humble Windmill to the Cosmic Windmill.
- 70 upgrades split across click power, click multipliers, building boosts, global multipliers, and Quixote-themed specials.
- 75 achievements covering pats, earnings, buildings, upgrades, pat speed, playtime, offline returns, and a few hidden deeds.
- Every unlocked achievement grants a permanent 0.5 percent global bonus.
- Formula-driven costs, cost growth, bulk purchasing, and large-number formatting with compact suffixes and scientific notation fallback.
- A progress reveal system that keeps unavailable buttons hidden until they are affordable or their predecessor has been purchased. Revealed buttons are remembered in the save file.
- Tooltips with names, descriptions, costs, effects, requirements, and production details.
- An animated patting GIF with per-frame timing, pat-speed-based playback scaling, smooth acceleration, squash-and-stretch feedback, floating numbers, and code-drawn heart particles.
- A high-contrast pixel interface with thick panel borders, a soul-heart cursor, a typewriter dialogue box, achievement toasts, and separate screens for buildings, upgrades, achievements, statistics, and settings.
- A saved light-or-dark theme toggle, so the whole game can be played on a white or near-black background.
- Original procedural chiptune music with a soft triangle-wave lead, a warm triangle bass, gentle noise percussion, and a mellow harmony layer. The mix is low-passed for a relaxing, less piercing sound, and layers fade in as the quest grows.
- Fully synthesized sound effects for patting, purchases, errors, achievements, menu movement, and dialogue speech blips.
- Persistent settings for music volume, effects volume, mute, and light or dark theme.
- Persistent saves in the operating system data directory through the directories crate, with serde defaults for forward compatibility. If the primary location is unavailable, the game automatically falls back to a writable location and shows the active save path in Settings.
- Atomic save writes, autosavings every 30 seconds, a manual save button, reset with confirmation, and a save on exit.
- Offline Affection based on production while away, capped at eight hours, reported by Don Quixote on the next launch.
- Resizable window with a fixed internal 640 by 480 pixel canvas, nearest-neighbor scaling, and aspect-ratio letterboxing.
- Keyboard and mouse support throughout.

## Project Layout

    Cargo.toml
    build.rs
    README.md
    don_quixote.png
    pet.gif
    icon.ico
    src/
      main.rs
      game.rs
      save.rs
      theme.rs
      upgrades.rs
      buildings.rs
      achievements.rs
      audio.rs
      ui.rs
      font.rs
      pat_animation.rs
      format.rs
      dialogue.rs

## Notes

- The save file location is shown inside the Settings screen. If the preferred operating system save folder cannot be used, the game selects a writable fallback and continues saving automatically.
- The patting animation and idle portrait are loaded from embedded copies of pet.gif and don_quixote.png.
- No font files, image atlases, audio files, or other generated assets are used. The font, hearts, borders, particles, cursor, and every other visual element are drawn in code.
- The game targets 4:3. If the window has another shape, the game field is letterboxed rather than stretched.

created by arkanquixote
