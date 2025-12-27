use std::path::PathBuf;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Sense, Stroke, Vec2};

use crate::ast::{Score, Header};
use crate::ir::{IrScore, IrEventKind};

// Piano roll constants
const KEY_WIDTH: f32 = 60.0;
const ROW_HEIGHT: f32 = 12.0;
const HEADER_HEIGHT: f32 = 30.0;
const PIXELS_PER_BEAT: f32 = 80.0;

// Track colors (for different parts)
const TRACK_COLORS: &[Color32] = &[
    Color32::from_rgb(100, 180, 255),  // Blue
    Color32::from_rgb(255, 150, 100),  // Orange
    Color32::from_rgb(150, 255, 150),  // Green
    Color32::from_rgb(255, 150, 255),  // Pink
    Color32::from_rgb(255, 255, 150),  // Yellow
    Color32::from_rgb(150, 255, 255),  // Cyan
    Color32::from_rgb(200, 150, 255),  // Purple
    Color32::from_rgb(255, 200, 150),  // Peach
];

pub struct PianoRollApp {
    score_path: PathBuf,
    ast: Score,
    ir: IrScore,
    // View state
    scroll_x: f32,
    scroll_y: f32,
    zoom_x: f32,
    // Derived data
    tempo: u32,
    time_sig: (u32, u32),
    total_beats: u32,
    pitch_range: (u8, u8), // (min, max) MIDI pitch
    track_visibility: Vec<bool>,
}

impl PianoRollApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, score_path: PathBuf, ast: Score, ir: IrScore) -> Self {
        let tempo = ast.headers.iter()
            .find_map(|h| if let Header::Tempo(t) = h { Some(*t) } else { None })
            .unwrap_or(120);

        let time_sig = ast.headers.iter()
            .find_map(|h| if let Header::TimeSignature(n, d) = h { Some((*n, *d)) } else { None })
            .unwrap_or((4, 4));

        let (pitch_range, total_beats) = Self::analyze_score(&ir);
        let track_visibility = vec![true; ir.tracks.len()];

        Self {
            score_path,
            ast,
            ir,
            scroll_x: 0.0,
            scroll_y: 0.0,
            zoom_x: 1.0,
            tempo,
            time_sig,
            total_beats,
            pitch_range,
            track_visibility,
        }
    }

    fn analyze_score(ir: &IrScore) -> ((u8, u8), u32) {
        let mut min_pitch: u8 = 127;
        let mut max_pitch: u8 = 0;
        let mut max_tick: u32 = 0;

        for track in &ir.tracks {
            for event in &track.events {
                if let IrEventKind::Note { pitch, duration, .. } = &event.kind {
                    min_pitch = min_pitch.min(*pitch);
                    max_pitch = max_pitch.max(*pitch);
                    max_tick = max_tick.max(event.time + duration);
                }
            }
        }

        // Add some padding to pitch range
        min_pitch = min_pitch.saturating_sub(2);
        max_pitch = (max_pitch + 2).min(127);

        // Ensure at least an octave range
        if max_pitch - min_pitch < 12 {
            let mid = (min_pitch + max_pitch) / 2;
            min_pitch = mid.saturating_sub(6);
            max_pitch = (mid + 6).min(127);
        }

        let total_beats = (max_tick / ir.ppq) + 1;
        ((min_pitch, max_pitch), total_beats)
    }

    fn pitch_to_name(pitch: u8) -> String {
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (pitch / 12) as i8 - 1;
        let note = (pitch % 12) as usize;
        format!("{}{}", note_names[note], octave)
    }

    fn is_black_key(pitch: u8) -> bool {
        matches!(pitch % 12, 1 | 3 | 6 | 8 | 10)
    }

    fn pixels_per_beat(&self) -> f32 {
        PIXELS_PER_BEAT * self.zoom_x
    }
}

impl eframe::App for PianoRollApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with controls
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Melos Piano Roll");
                ui.separator();
                ui.label(format!("File: {}", self.score_path.file_name().unwrap_or_default().to_string_lossy()));
                ui.separator();
                ui.label(format!("Tempo: {} BPM", self.tempo));
                ui.label(format!("Time: {}/{}", self.time_sig.0, self.time_sig.1));
                ui.separator();

                ui.label("Zoom:");
                if ui.button("-").clicked() {
                    self.zoom_x = (self.zoom_x * 0.8).max(0.25);
                }
                ui.label(format!("{:.0}%", self.zoom_x * 100.0));
                if ui.button("+").clicked() {
                    self.zoom_x = (self.zoom_x * 1.25).min(4.0);
                }
            });
        });

        // Bottom panel with track legend
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tracks:");
                for (i, track) in self.ir.tracks.iter().enumerate() {
                    let color = TRACK_COLORS[i % TRACK_COLORS.len()];
                    let visible = self.track_visibility.get(i).copied().unwrap_or(true);

                    let button_text = egui::RichText::new(&track.name)
                        .color(if visible { color } else { Color32::GRAY });

                    if ui.button(button_text).clicked() {
                        if let Some(v) = self.track_visibility.get_mut(i) {
                            *v = !*v;
                        }
                    }
                }
            });
        });

        // Main piano roll area
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            // Handle scroll input
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
            self.scroll_x -= scroll_delta.x;
            self.scroll_y -= scroll_delta.y;

            // Clamp scroll
            let content_width = self.total_beats as f32 * self.pixels_per_beat();
            let content_height = (self.pitch_range.1 - self.pitch_range.0) as f32 * ROW_HEIGHT;
            self.scroll_x = self.scroll_x.clamp(0.0, (content_width - available_size.x + KEY_WIDTH).max(0.0));
            self.scroll_y = self.scroll_y.clamp(0.0, (content_height - available_size.y + HEADER_HEIGHT).max(0.0));

            // Create the piano roll area
            let (response, mut painter) = ui.allocate_painter(available_size, Sense::drag());
            let rect = response.rect;

            // Handle drag for panning
            if response.dragged() {
                self.scroll_x -= response.drag_delta().x;
                self.scroll_y -= response.drag_delta().y;
                self.scroll_x = self.scroll_x.clamp(0.0, (content_width - available_size.x + KEY_WIDTH).max(0.0));
                self.scroll_y = self.scroll_y.clamp(0.0, (content_height - available_size.y + HEADER_HEIGHT).max(0.0));
            }

            // Background
            painter.rect_filled(rect, Rounding::ZERO, Color32::from_rgb(30, 30, 35));

            // Piano keys area (left side)
            let keys_rect = Rect::from_min_size(
                rect.min,
                Vec2::new(KEY_WIDTH, rect.height()),
            );
            painter.rect_filled(keys_rect, Rounding::ZERO, Color32::from_rgb(40, 40, 45));

            // Draw piano keys
            let (min_pitch, max_pitch) = self.pitch_range;
            for pitch in min_pitch..=max_pitch {
                let row = (max_pitch - pitch) as f32;
                let y = rect.min.y + HEADER_HEIGHT + row * ROW_HEIGHT - self.scroll_y;

                if y < rect.min.y + HEADER_HEIGHT - ROW_HEIGHT || y > rect.max.y {
                    continue;
                }

                let key_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x, y),
                    Vec2::new(KEY_WIDTH - 2.0, ROW_HEIGHT - 1.0),
                );

                let is_black = Self::is_black_key(pitch);
                let key_color = if is_black {
                    Color32::from_rgb(40, 40, 45)
                } else {
                    Color32::from_rgb(220, 220, 220)
                };
                let text_color = if is_black { Color32::WHITE } else { Color32::BLACK };

                painter.rect_filled(key_rect, Rounding::same(2.0), key_color);

                // Show note name for C notes or at edges
                if pitch % 12 == 0 || pitch == min_pitch || pitch == max_pitch {
                    painter.text(
                        Pos2::new(rect.min.x + 5.0, y + ROW_HEIGHT / 2.0),
                        egui::Align2::LEFT_CENTER,
                        Self::pitch_to_name(pitch),
                        egui::FontId::proportional(10.0),
                        text_color,
                    );
                }
            }

            // Grid area
            let grid_rect = Rect::from_min_max(
                Pos2::new(rect.min.x + KEY_WIDTH, rect.min.y + HEADER_HEIGHT),
                rect.max,
            );
            painter.set_clip_rect(grid_rect);

            // Draw horizontal grid lines (pitch rows)
            for pitch in min_pitch..=max_pitch {
                let row = (max_pitch - pitch) as f32;
                let y = rect.min.y + HEADER_HEIGHT + row * ROW_HEIGHT - self.scroll_y;

                if y < grid_rect.min.y - ROW_HEIGHT || y > grid_rect.max.y {
                    continue;
                }

                let is_black = Self::is_black_key(pitch);
                let row_color = if is_black {
                    Color32::from_rgb(35, 35, 40)
                } else {
                    Color32::from_rgb(45, 45, 50)
                };

                let row_rect = Rect::from_min_size(
                    Pos2::new(grid_rect.min.x, y),
                    Vec2::new(grid_rect.width(), ROW_HEIGHT),
                );
                painter.rect_filled(row_rect, Rounding::ZERO, row_color);

                // C notes get a subtle highlight
                if pitch % 12 == 0 {
                    painter.hline(
                        grid_rect.min.x..=grid_rect.max.x,
                        y,
                        Stroke::new(1.0, Color32::from_rgb(80, 80, 90)),
                    );
                }
            }

            // Draw vertical grid lines (beats and measures)
            let ppb = self.pixels_per_beat();
            let start_beat = (self.scroll_x / ppb) as u32;
            let end_beat = ((self.scroll_x + grid_rect.width()) / ppb) as u32 + 1;

            for beat in start_beat..=end_beat.min(self.total_beats) {
                let x = grid_rect.min.x + beat as f32 * ppb - self.scroll_x;

                let is_measure_start = beat % self.time_sig.0 == 0;
                let stroke = if is_measure_start {
                    Stroke::new(1.5, Color32::from_rgb(100, 100, 110))
                } else {
                    Stroke::new(0.5, Color32::from_rgb(60, 60, 70))
                };

                painter.vline(x, grid_rect.min.y..=grid_rect.max.y, stroke);
            }

            // Draw notes
            let ppq = self.ir.ppq as f32;
            for (track_idx, track) in self.ir.tracks.iter().enumerate() {
                if !self.track_visibility.get(track_idx).copied().unwrap_or(true) {
                    continue;
                }

                let track_color = TRACK_COLORS[track_idx % TRACK_COLORS.len()];

                for event in &track.events {
                    if let IrEventKind::Note { pitch, duration, velocity } = &event.kind {
                        // Calculate note position and size
                        let note_start_beat = event.time as f32 / ppq;
                        let note_duration_beats = *duration as f32 / ppq;

                        let x = grid_rect.min.x + note_start_beat * ppb - self.scroll_x;
                        let width = note_duration_beats * ppb - 2.0;

                        let row = (max_pitch - pitch) as f32;
                        let y = rect.min.y + HEADER_HEIGHT + row * ROW_HEIGHT - self.scroll_y + 1.0;

                        // Skip if not visible
                        if x + width < grid_rect.min.x || x > grid_rect.max.x {
                            continue;
                        }
                        if y + ROW_HEIGHT < grid_rect.min.y || y > grid_rect.max.y {
                            continue;
                        }

                        let note_rect = Rect::from_min_size(
                            Pos2::new(x.max(grid_rect.min.x), y),
                            Vec2::new(width.min(grid_rect.max.x - x), ROW_HEIGHT - 2.0),
                        );

                        // Adjust alpha based on velocity
                        let alpha = 150 + ((*velocity as f32 / 127.0) * 105.0) as u8;
                        let note_color = Color32::from_rgba_unmultiplied(
                            track_color.r(),
                            track_color.g(),
                            track_color.b(),
                            alpha,
                        );

                        painter.rect_filled(note_rect, Rounding::same(2.0), note_color);
                        painter.rect_stroke(note_rect, Rounding::same(2.0), Stroke::new(1.0, track_color));
                    }
                }
            }

            // Reset clip rect and draw header
            painter.set_clip_rect(rect);

            // Header background
            let header_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + KEY_WIDTH, rect.min.y),
                Vec2::new(rect.width() - KEY_WIDTH, HEADER_HEIGHT),
            );
            painter.rect_filled(header_rect, Rounding::ZERO, Color32::from_rgb(50, 50, 55));

            // Draw measure numbers in header
            painter.set_clip_rect(header_rect);
            for beat in start_beat..=end_beat.min(self.total_beats) {
                if beat % self.time_sig.0 == 0 {
                    let measure = beat / self.time_sig.0 + 1;
                    let x = grid_rect.min.x + beat as f32 * ppb - self.scroll_x;

                    painter.text(
                        Pos2::new(x + 4.0, rect.min.y + HEADER_HEIGHT / 2.0),
                        egui::Align2::LEFT_CENTER,
                        format!("{}", measure),
                        egui::FontId::proportional(12.0),
                        Color32::WHITE,
                    );
                }
            }
        });
    }
}
