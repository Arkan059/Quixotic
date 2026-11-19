use macroquad::audio::{
    load_sound_from_bytes, play_sound, set_sound_volume, PlaySoundParams, Sound,
};

use crate::save::Settings;

const SAMPLE_RATE: f64 = 44_100.0;
const BPM: f64 = 108.0;
const BARS: usize = 8;
const BEATS_PER_BAR: usize = 4;
const BEAT_SECONDS: f64 = 60.0 / BPM;
const STEP_SECONDS: f64 = BEAT_SECONDS / 2.0;
const LOOP_SECONDS: f64 = BEAT_SECONDS * (BARS * BEATS_PER_BAR) as f64;

const BASE_LAYER_GAINS: [f32; 4] = [0.42, 0.32, 0.18, 0.16];

const MELODY: [(i32, f64); 54] = [
    (72, 2.0),
    (76, 1.0),
    (79, 1.0),
    (76, 1.0),
    (72, 1.0),
    (74, 1.0),
    (76, 1.0),
    (71, 1.0),
    (74, 1.0),
    (79, 2.0),
    (74, 1.0),
    (71, 1.0),
    (72, 1.0),
    (74, 1.0),
    (69, 1.0),
    (72, 1.0),
    (76, 2.0),
    (81, 1.0),
    (79, 1.0),
    (76, 1.0),
    (72, 1.0),
    (65, 1.0),
    (69, 1.0),
    (72, 2.0),
    (77, 1.0),
    (76, 1.0),
    (72, 1.0),
    (69, 1.0),
    (72, 1.0),
    (76, 1.0),
    (79, 1.0),
    (84, 2.0),
    (83, 1.0),
    (79, 1.0),
    (76, 1.0),
    (71, 1.0),
    (74, 1.0),
    (79, 1.0),
    (83, 2.0),
    (81, 1.0),
    (79, 1.0),
    (74, 1.0),
    (69, 1.0),
    (72, 1.0),
    (77, 1.0),
    (81, 2.0),
    (79, 1.0),
    (76, 1.0),
    (72, 1.0),
    (67, 1.0),
    (72, 1.0),
    (76, 1.0),
    (79, 2.0),
    (84, 3.0),
];

const CHORDS: [[i32; 3]; 8] = [
    [48, 52, 55],
    [43, 47, 50],
    [45, 48, 52],
    [41, 45, 48],
    [48, 52, 55],
    [43, 47, 50],
    [41, 45, 48],
    [43, 47, 50],
];

#[derive(Clone, Copy)]
enum Waveform {
    Square,
    Triangle,
}

struct NoiseSource {
    state: u32,
}

impl NoiseSource {
    fn new(seed: u32) -> Self {
        NoiseSource { state: seed | 1 }
    }

    fn next(&mut self) -> f32 {
        self.state = self
            .state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let value = (self.state >> 9) as f32 / 4_194_304.0;
        value * 2.0 - 1.0
    }
}

fn midi_frequency(note: i32) -> f64 {
    440.0 * 2f64.powf((note as f64 - 69.0) / 12.0)
}

fn envelope(t: f64, duration: f64) -> f64 {
    if duration <= 0.0 {
        return 0.0;
    }
    let attack = 0.018f64.min(duration * 0.3);
    let release = 0.09f64.min(duration * 0.4);
    if t < attack {
        t / attack
    } else if t > duration - release {
        ((duration - t) / release).clamp(0.0, 1.0)
    } else {
        1.0
    }
}

fn add_tone(
    buffer: &mut [f32],
    start_seconds: f64,
    duration: f64,
    start_freq: f64,
    end_freq: f64,
    amplitude: f64,
    waveform: Waveform,
    duty: f64,
) {
    let start_sample = (start_seconds * SAMPLE_RATE).round() as isize;
    let sample_count = (duration * SAMPLE_RATE).round() as isize;
    let mut phase = 0.0f64;
    for offset in 0..sample_count {
        let index = start_sample + offset;
        if index < 0 || index as usize >= buffer.len() {
            continue;
        }
        let t = offset as f64 / SAMPLE_RATE;
        let progress = if duration > 0.0 { t / duration } else { 1.0 };
        let frequency = start_freq + (end_freq - start_freq) * progress;
        phase = (phase + frequency / SAMPLE_RATE).fract();
        let wave = match waveform {
            Waveform::Square => {
                if phase < duty {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Triangle => 4.0 * (phase - 0.5).abs() - 1.0,
        };
        buffer[index as usize] += (wave * amplitude * envelope(t, duration)) as f32;
    }
}

fn add_noise(
    buffer: &mut [f32],
    start_seconds: f64,
    duration: f64,
    amplitude: f64,
    decay: f64,
    seed: u32,
) {
    let start_sample = (start_seconds * SAMPLE_RATE).round() as isize;
    let sample_count = (duration * SAMPLE_RATE).round() as isize;
    let mut noise = NoiseSource::new(seed);
    for offset in 0..sample_count {
        let index = start_sample + offset;
        if index < 0 || index as usize >= buffer.len() {
            continue;
        }
        let t = offset as f64 / SAMPLE_RATE;
        let attack = (t / 0.002).min(1.0);
        let env = attack * (-decay * t).exp();
        buffer[index as usize] += noise.next() * amplitude as f32 * env as f32;
    }
}

fn normalize(buffer: &mut [f32], target_peak: f32) {
    let mut peak = 0.0f32;
    for sample in buffer.iter() {
        peak = peak.max(sample.abs());
    }
    if peak > 0.0001 {
        let scale = target_peak / peak;
        for sample in buffer.iter_mut() {
            *sample *= scale;
        }
    }
}

fn soften_highs(buffer: &mut [f32], cutoff: f64) {
    if cutoff <= 0.0 || buffer.is_empty() {
        return;
    }
    let dt = 1.0 / SAMPLE_RATE;
    let rc = 1.0 / (2.0 * std::f64::consts::PI * cutoff);
    let alpha = dt / (rc + dt);
    let mut previous = 0.0f64;
    for sample in buffer.iter_mut() {
        previous += alpha * (*sample as f64 - previous);
        *sample = previous as f32;
    }
}

fn add_soft_echo(buffer: &mut [f32], delay_seconds: f64, wet: f32) {
    let delay = (delay_seconds * SAMPLE_RATE).round() as usize;
    if delay == 0 || delay >= buffer.len() {
        return;
    }
    let dry = buffer.to_vec();
    for (index, sample) in buffer.iter_mut().enumerate() {
        let echo = dry[(index + dry.len() - delay) % dry.len()];
        *sample += echo * wet;
    }
}

fn soften_loop_edges(buffer: &mut [f32], fade_seconds: f64) {
    let fade = ((fade_seconds * SAMPLE_RATE).round() as usize).min(buffer.len() / 2);
    if fade == 0 {
        return;
    }
    for index in 0..fade {
        let gain = index as f32 / fade as f32;
        buffer[index] *= gain;
        let end = buffer.len() - 1 - index;
        buffer[end] *= gain;
    }
}

fn melody_layer() -> Vec<f32> {
    let total = (LOOP_SECONDS * SAMPLE_RATE) as usize;
    let mut track = vec![0.0f32; total];
    let mut step = 0usize;
    for (note, length) in MELODY.iter() {
        let start = step as f64 * STEP_SECONDS;
        let duration = length * STEP_SECONDS * 0.90;
        let frequency = midi_frequency(*note);
        add_tone(
            &mut track,
            start,
            duration,
            frequency,
            frequency,
            0.32,
            Waveform::Triangle,
            0.5,
        );
        add_tone(
            &mut track,
            start,
            duration,
            frequency,
            frequency,
            0.025,
            Waveform::Square,
            0.5,
        );
        step += *length as usize;
    }
    add_soft_echo(&mut track, 0.19, 0.10);
    soften_highs(&mut track, 2_200.0);
    normalize(&mut track, 0.55);
    soften_loop_edges(&mut track, 0.012);
    track
}

fn bass_layer() -> Vec<f32> {
    let total = (LOOP_SECONDS * SAMPLE_RATE) as usize;
    let mut track = vec![0.0f32; total];
    let pattern = [0, 7, 12, 7];
    for bar in 0..BARS {
        let root = CHORDS[bar][0];
        for beat in 0..BEATS_PER_BAR {
            let start = (bar * BEATS_PER_BAR + beat) as f64 * BEAT_SECONDS;
            let note = root + pattern[beat];
            let frequency = midi_frequency(note);
            add_tone(
                &mut track,
                start,
                BEAT_SECONDS * 0.90,
                frequency,
                frequency,
                0.42,
                Waveform::Triangle,
                0.5,
            );
        }
    }
    soften_highs(&mut track, 620.0);
    normalize(&mut track, 0.52);
    soften_loop_edges(&mut track, 0.012);
    track
}

fn harmony_layer() -> Vec<f32> {
    let total = (LOOP_SECONDS * SAMPLE_RATE) as usize;
    let mut track = vec![0.0f32; total];
    let arpeggio = [0, 1, 2, 1];
    for bar in 0..BARS {
        for step in 0..BEATS_PER_BAR * 2 {
            let start = (bar * BEATS_PER_BAR * 2 + step) as f64 * STEP_SECONDS;
            let chord_index = arpeggio[step % 4];
            let note = CHORDS[bar][chord_index] + 12;
            let frequency = midi_frequency(note);
            add_tone(
                &mut track,
                start,
                STEP_SECONDS * 0.70,
                frequency,
                frequency,
                0.15,
                Waveform::Triangle,
                0.5,
            );
        }
    }
    add_soft_echo(&mut track, 0.28, 0.08);
    soften_highs(&mut track, 1_500.0);
    normalize(&mut track, 0.44);
    soften_loop_edges(&mut track, 0.012);
    track
}

fn percussion_layer() -> Vec<f32> {
    let total = (LOOP_SECONDS * SAMPLE_RATE) as usize;
    let mut track = vec![0.0f32; total];
    for bar in 0..BARS {
        for beat in 0..BEATS_PER_BAR {
            let start = (bar * BEATS_PER_BAR + beat) as f64 * BEAT_SECONDS;
            if beat == 0 || beat == 2 {
                add_tone(
                    &mut track,
                    start,
                    0.16,
                    118.0,
                    50.0,
                    0.34,
                    Waveform::Triangle,
                    0.5,
                );
                add_noise(
                    &mut track,
                    start,
                    0.035,
                    0.05,
                    55.0,
                    17 + (bar * 4 + beat) as u32,
                );
            }
            if beat == 1 || beat == 3 {
                add_noise(
                    &mut track,
                    start,
                    0.12,
                    0.16,
                    30.0,
                    31 + (bar * 4 + beat) as u32,
                );
                add_tone(
                    &mut track,
                    start,
                    0.08,
                    180.0,
                    155.0,
                    0.05,
                    Waveform::Triangle,
                    0.5,
                );
            }
            let offbeat = start + STEP_SECONDS;
            add_noise(
                &mut track,
                offbeat,
                0.030,
                0.06,
                90.0,
                53 + (bar * 4 + beat) as u32,
            );
        }
    }
    soften_highs(&mut track, 2_800.0);
    normalize(&mut track, 0.40);
    soften_loop_edges(&mut track, 0.012);
    track
}

fn wav_bytes(samples: &[f32]) -> Vec<u8> {
    let data_length = samples.len() * 2;
    let mut output = Vec::with_capacity(44 + data_length);
    output.extend_from_slice(b"RIFF");
    output.extend_from_slice(&((36 + data_length) as u32).to_le_bytes());
    output.extend_from_slice(b"WAVEfmt ");
    output.extend_from_slice(&16u32.to_le_bytes());
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&(SAMPLE_RATE as u32).to_le_bytes());
    output.extend_from_slice(&((SAMPLE_RATE as u32) * 2).to_le_bytes());
    output.extend_from_slice(&2u16.to_le_bytes());
    output.extend_from_slice(&16u16.to_le_bytes());
    output.extend_from_slice(b"data");
    output.extend_from_slice(&(data_length as u32).to_le_bytes());
    for sample in samples {
        let scaled = (sample.clamp(-1.0, 1.0) * 32_000.0) as i16;
        output.extend_from_slice(&scaled.to_le_bytes());
    }
    output
}

fn music_wavs() -> [Vec<u8>; 4] {
    [
        melody_layer(),
        bass_layer(),
        percussion_layer(),
        harmony_layer(),
    ]
    .map(|layer| wav_bytes(&layer))
}

fn effect_tone(
    start: f64,
    end: f64,
    duration: f64,
    amplitude: f64,
    waveform: Waveform,
    duty: f64,
) -> Vec<f32> {
    let count = (duration * SAMPLE_RATE).round() as usize;
    let mut buffer = vec![0.0f32; count];
    add_tone(
        &mut buffer,
        0.0,
        duration,
        start,
        end,
        amplitude,
        waveform,
        duty,
    );
    normalize(&mut buffer, 0.82);
    buffer
}

fn pat_sound_data() -> Vec<Vec<u8>> {
    let mut sounds = Vec::new();
    for variant in 0..4 {
        let start = 610.0 + variant as f64 * 90.0;
        let end = 990.0 + variant as f64 * 55.0;
        let mut buffer = effect_tone(start, end, 0.09, 0.35, Waveform::Square, 0.3);
        let extra = effect_tone(end * 1.5, end * 1.8, 0.045, 0.14, Waveform::Triangle, 0.5);
        for (index, sample) in extra.iter().enumerate() {
            if index < buffer.len() {
                buffer[index] += sample * 0.5;
            }
        }
        sounds.push(wav_bytes(&buffer));
    }
    sounds
}

fn note_sequence(
    notes: &[(f64, f64, f64)],
    waveform: Waveform,
    duty: f64,
    amplitude: f64,
) -> Vec<f32> {
    let total_time: f64 = notes
        .iter()
        .map(|(_, start, length)| start + length)
        .fold(0.0, f64::max);
    let mut buffer = vec![0.0f32; (total_time * SAMPLE_RATE).ceil() as usize];
    for (frequency, start, length) in notes {
        add_tone(
            &mut buffer,
            *start,
            *length,
            *frequency,
            *frequency,
            amplitude,
            waveform,
            duty,
        );
    }
    normalize(&mut buffer, 0.82);
    buffer
}

fn sfx_data() -> Vec<Vec<u8>> {
    let mut sounds = Vec::new();
    let purchase = note_sequence(
        &[
            (523.25, 0.0, 0.075),
            (659.25, 0.065, 0.075),
            (783.99, 0.13, 0.13),
        ],
        Waveform::Square,
        0.25,
        0.35,
    );
    sounds.push(wav_bytes(&purchase));
    let error = effect_tone(180.0, 92.0, 0.18, 0.42, Waveform::Square, 0.5);
    sounds.push(wav_bytes(&error));
    let achievement = note_sequence(
        &[
            (523.25, 0.0, 0.10),
            (659.25, 0.10, 0.10),
            (783.99, 0.20, 0.10),
            (1046.50, 0.30, 0.27),
            (659.25, 0.30, 0.27),
            (523.25, 0.56, 0.20),
        ],
        Waveform::Square,
        0.25,
        0.28,
    );
    sounds.push(wav_bytes(&achievement));
    let cursor = effect_tone(1320.0, 1560.0, 0.028, 0.20, Waveform::Square, 0.5);
    sounds.push(wav_bytes(&cursor));
    sounds
}

fn blip_data() -> Vec<Vec<u8>> {
    let mut sounds = Vec::new();
    for variant in 0..8 {
        let frequency = 520.0 + variant as f64 * 65.0;
        let buffer = effect_tone(
            frequency,
            frequency * 1.04,
            0.032,
            0.22,
            Waveform::Square,
            0.5,
        );
        sounds.push(wav_bytes(&buffer));
    }
    sounds
}

pub struct AudioSystem {
    music_layers: Vec<Option<Sound>>,
    music_gains: [f32; 4],
    pat_sounds: Vec<Sound>,
    purchase_sound: Option<Sound>,
    error_sound: Option<Sound>,
    achievement_sound: Option<Sound>,
    cursor_sound: Option<Sound>,
    blip_sounds: Vec<Sound>,
    music_volume: f32,
    effects_volume: f32,
    muted: bool,
    random_state: u64,
}

impl AudioSystem {
    pub async fn new(settings: &Settings) -> Self {
        let mut music_layers = Vec::with_capacity(4);
        for wav in music_wavs() {
            music_layers.push(load_sound_from_bytes(&wav).await.ok());
        }
        let mut pat_sounds = Vec::new();
        for wav in pat_sound_data() {
            if let Ok(sound) = load_sound_from_bytes(&wav).await {
                pat_sounds.push(sound);
            }
        }
        let mut effect_sounds = Vec::new();
        for wav in sfx_data() {
            effect_sounds.push(load_sound_from_bytes(&wav).await.ok());
        }
        let cursor_sound = effect_sounds.pop().flatten();
        let achievement_sound = effect_sounds.pop().flatten();
        let error_sound = effect_sounds.pop().flatten();
        let purchase_sound = effect_sounds.pop().flatten();
        let mut blip_sounds = Vec::new();
        for wav in blip_data() {
            if let Ok(sound) = load_sound_from_bytes(&wav).await {
                blip_sounds.push(sound);
            }
        }
        let mut audio = AudioSystem {
            music_layers,
            music_gains: [0.0; 4],
            pat_sounds,
            purchase_sound,
            error_sound,
            achievement_sound,
            cursor_sound,
            blip_sounds,
            music_volume: settings.music_volume,
            effects_volume: settings.effects_volume,
            muted: settings.muted,
            random_state: seed(),
        };
        audio.start_music();
        audio
    }

    fn start_music(&mut self) {
        for layer in self.music_layers.iter().flatten() {
            play_sound(
                layer,
                PlaySoundParams {
                    looped: true,
                    volume: 0.0,
                },
            );
        }
    }

    fn random(&mut self) -> u32 {
        let mut value = self.random_state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.random_state = value;
        value as u32
    }

    fn play_effect(&self, sound: &Sound, volume: f32) {
        if self.muted {
            return;
        }
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: (self.effects_volume * volume).clamp(0.0, 1.0),
            },
        );
    }

    pub fn play_pat(&mut self) {
        if self.pat_sounds.is_empty() {
            return;
        }
        let index = self.random() as usize % self.pat_sounds.len();
        let sound = self.pat_sounds[index].clone();
        self.play_effect(&sound, 0.52);
    }

    pub fn play_purchase(&self) {
        if let Some(sound) = &self.purchase_sound {
            self.play_effect(sound, 0.65);
        }
    }

    pub fn play_error(&self) {
        if let Some(sound) = &self.error_sound {
            self.play_effect(sound, 0.55);
        }
    }

    pub fn play_achievement(&self) {
        if let Some(sound) = &self.achievement_sound {
            self.play_effect(sound, 0.8);
        }
    }

    pub fn play_cursor(&self) {
        if let Some(sound) = &self.cursor_sound {
            self.play_effect(sound, 0.22);
        }
    }

    pub fn play_blip(&mut self) {
        if self.blip_sounds.is_empty() {
            return;
        }
        let index = self.random() as usize % self.blip_sounds.len();
        let sound = self.blip_sounds[index].clone();
        self.play_effect(&sound, 0.25);
    }

    pub fn update(&mut self, delta: f64, settings: &Settings, stage: usize) {
        self.music_volume = settings.music_volume;
        self.effects_volume = settings.effects_volume;
        self.muted = settings.muted;
        let target_scale = (1.0 - (-(delta as f32) * 4.0).exp()).clamp(0.0, 1.0);
        let active_layers = (stage + 1).clamp(1, 4);
        for index in 0..4 {
            let target = if index < active_layers {
                BASE_LAYER_GAINS[index]
            } else {
                0.0
            };
            self.music_gains[index] += (target - self.music_gains[index]) * target_scale;
            let mute_scale = if self.muted { 0.0 } else { 1.0 };
            let volume = (self.music_gains[index] * self.music_volume * mute_scale).clamp(0.0, 1.0);
            if let Some(Some(layer)) = self.music_layers.get(index) {
                set_sound_volume(layer, volume);
            }
        }
    }
}

fn seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0x9E3779B97F4A7C15)
        ^ 0x9E3779B97F4A7C15
}

pub fn stage_for_progress(total_earned: f64, total_buildings: u64) -> usize {
    if total_earned >= 250_000.0 || total_buildings >= 60 {
        3
    } else if total_earned >= 5_000.0 || total_buildings >= 20 {
        2
    } else if total_earned >= 250.0 || total_buildings >= 3 {
        1
    } else {
        0
    }
}
