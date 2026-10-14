//! Sequence Mismatch Warning Dialog.
//!
//! When a clip added to the timeline does not match the active sequence's resolution
//! or frame rate, this dialog offers:
//! 1. "Keep Existing Sequence Settings"
//! 2. "Change Sequence to Match Clip"
//! 3. "Create Custom Sequence" (presets: 16:9, 9:16 vertical, 1:1, 4:5, 4K, custom)

use egui::{Align2, Color32, Rect, vec2};
use serde_json::json;

use crate::FilmcraftApp;

const PRESETS: &[(&str, u32, u32)] = &[
    ("1920 × 1080 — 16:9 Landscape (Full HD)", 1920, 1080),
    ("1080 × 1920 — 9:16 Vertical (Reels / TikTok)", 1080, 1920),
    ("1080 × 1080 — 1:1 Square (Social Media)", 1080, 1080),
    ("1080 × 1350 — 4:5 Portrait (Feed)", 1080, 1350),
    ("3840 × 2160 — 4K Landscape (Ultra HD)", 3840, 2160),
    ("Custom Dimensions", 0, 0),
];

const TIMEBASES: &[(i64, i64, &str)] = &[
    (24_000, 1001, "23.976 fps"),
    (24, 1, "24 fps"),
    (25, 1, "25 fps"),
    (30_000, 1001, "29.97 fps"),
    (30, 1, "30 fps"),
    (50, 1, "50 fps"),
    (60_000, 1001, "59.94 fps"),
    (60, 1, "60 fps"),
    (120, 1, "120 fps"),
];

pub fn show(app: &mut FilmcraftApp, ctx: &egui::Context) {
    let Some(mut draft) = app.ui.mismatch_dialog.clone() else { return };
    let mut close = false;
    let mut execute_placement = false;
    let mut new_seq_settings: Option<(u32, u32, f64)> = None;

    let t = app.tokens;

    egui::Window::new(tl!("Clip Mismatch Warning"))
        .id(egui::Id::new("clip_mismatch_warning_window"))
        .pivot(Align2::CENTER_CENTER)
        .default_pos(crate::widgets::dialog_center(ctx))
        .default_width(520.0)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            
            // Header icon + title
            ui.horizontal(|ui| {
                crate::icons::paint(
                    ui.painter(),
                    Rect::from_min_size(ui.cursor().min, vec2(28.0, 28.0)),
                    crate::icons::Icon::Sequence,
                    t.accent,
                );
                ui.add_space(34.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(tl!("The clip does not match the sequence settings.")).strong().size(15.0));
                    ui.label(egui::RichText::new(tl!("What would you like to do?")).color(t.text_dim).size(13.0));
                });
            });

            ui.add_space(14.0);

            // Comparison Card
            let clip_fps_f = draft.clip_fps_num as f64 / draft.clip_fps_den.max(1) as f64;
            let seq_fps_f = draft.seq_fps_num as f64 / draft.seq_fps_den.max(1) as f64;
            let clip_aspect = aspect_str(draft.clip_width, draft.clip_height);
            let seq_aspect = aspect_str(draft.seq_width, draft.seq_height);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let half = (ui.available_width() - 16.0) / 2.0;
                    ui.allocate_ui_with_layout(vec2(half, 56.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label(egui::RichText::new(tl!("Clip Settings")).strong().color(t.accent_hover).size(12.0));
                        ui.label(format!("{} × {} ({})", draft.clip_width, draft.clip_height, clip_aspect));
                        ui.label(format!("{:.2} fps — {}", clip_fps_f, draft.clip_name));
                    });
                    ui.add_space(16.0);
                    ui.allocate_ui_with_layout(vec2(half, 56.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label(egui::RichText::new(tl!("Active Sequence Settings")).strong().color(t.text).size(12.0));
                        ui.label(format!("{} × {} ({})", draft.seq_width, draft.seq_height, seq_aspect));
                        ui.label(format!("{:.2} fps", seq_fps_f));
                    });
                });
            });

            ui.add_space(12.0);

            // Primary choices: Keep vs Match
            ui.vertical(|ui| {
                let keep_btn = ui.add_sized(
                    [ui.available_width(), 32.0],
                    egui::Button::new(egui::RichText::new(tl!("Keep Existing Sequence Settings")).strong()),
                );
                if keep_btn.clicked() {
                    execute_placement = true;
                    close = true;
                }

                ui.add_space(4.0);

                let match_btn = ui.add_sized(
                    [ui.available_width(), 32.0],
                    egui::Button::new(
                        egui::RichText::new(format!(
                            "{} ({} × {}, {:.2} fps)",
                            tl!("Change Sequence to Match Clip"),
                            draft.clip_width,
                            draft.clip_height,
                            clip_fps_f
                        ))
                        .strong()
                        .color(Color32::WHITE),
                    )
                    .fill(t.accent),
                );
                if match_btn.clicked() {
                    new_seq_settings = Some((draft.clip_width, draft.clip_height, clip_fps_f));
                    execute_placement = true;
                    close = true;
                }
            });

            ui.add_space(10.0);

            // Collapsible / toggle custom sequence presets
            let custom_header = if draft.show_custom {
                tl!("▼ Custom Sequence Settings")
            } else {
                tl!("▶ Create Custom Sequence…")
            };
            if ui.button(custom_header).clicked() {
                draft.show_custom = !draft.show_custom;
            }

            if draft.show_custom {
                ui.add_space(6.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new(tl!("Preset:")).size(12.0));
                    egui::ComboBox::from_id_salt("custom_preset_combo")
                        .width(ui.available_width() - 10.0)
                        .selected_text(PRESETS[draft.custom_preset].0)
                        .show_ui(ui, |ui| {
                            for (idx, (label, w, h)) in PRESETS.iter().enumerate() {
                                if ui.selectable_value(&mut draft.custom_preset, idx, *label).clicked() {
                                    if *w > 0 && *h > 0 {
                                        draft.custom_width = *w;
                                        draft.custom_height = *h;
                                    }
                                }
                            }
                        });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(tl!("Width:"));
                        let mut w_str = draft.custom_width.to_string();
                        if ui.add(egui::TextEdit::singleline(&mut w_str).desired_width(70.0)).changed() {
                            if let Ok(v) = w_str.parse() {
                                draft.custom_width = v;
                            }
                        }
                        ui.label(tl!("Height:"));
                        let mut h_str = draft.custom_height.to_string();
                        if ui.add(egui::TextEdit::singleline(&mut h_str).desired_width(70.0)).changed() {
                            if let Ok(v) = h_str.parse() {
                                draft.custom_height = v;
                            }
                        }
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(tl!("Timebase:"));
                        let current_tb = TIMEBASES.get(draft.custom_fps_idx).map(|t| t.2).unwrap_or("24 fps");
                        egui::ComboBox::from_id_salt("custom_tb_combo")
                            .selected_text(current_tb)
                            .show_ui(ui, |ui| {
                                for (idx, (_, _, label)) in TIMEBASES.iter().enumerate() {
                                    ui.selectable_value(&mut draft.custom_fps_idx, idx, *label);
                                }
                            });
                    });

                    ui.add_space(6.0);
                    if ui.button(tl!("Apply Custom Sequence Settings and Place Clip")).clicked() {
                        let fps = TIMEBASES[draft.custom_fps_idx];
                        let fps_f = fps.0 as f64 / fps.1 as f64;
                        new_seq_settings = Some((draft.custom_width.max(16), draft.custom_height.max(16), fps_f));
                        execute_placement = true;
                        close = true;
                    }
                });
            }

            ui.add_space(14.0);
            ui.separator();

            // Bottom bar: Don't ask again checkbox + Cancel button
            ui.horizontal(|ui| {
                ui.checkbox(&mut draft.dont_ask_again, tl!("Don't ask again for this project"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tl!("Cancel")).clicked() {
                        close = true;
                    }
                });
            });
        });

    if close {
        if draft.dont_ask_again {
            app.session.prefs.timeline.clip_mismatch_warning = false;
        }
        if let Some((w, h, fps)) = new_seq_settings {
            let _ = app.session.execute("sequence.settings", json!({
                "width": w,
                "height": h,
                "fps": fps,
                "scaleMotion": true,
            }));
        }
        if execute_placement && !draft.params.is_null() {
            if let Err(e) = app.session.execute("timeline.place", draft.params) {
                app.ui.status = e.to_string();
            }
        }
        app.ui.mismatch_dialog = None;
    } else {
        app.ui.mismatch_dialog = Some(draft);
    }
}

fn aspect_str(w: u32, h: u32) -> String {
    if h == 0 {
        return "16:9".into();
    }
    let gcd = gcd(w, h);
    let (aw, ah) = (w / gcd, h / gcd);
    if (aw, ah) == (16, 9) || (aw, ah) == (9, 16) || (aw, ah) == (1, 1) || (aw, ah) == (4, 5) || (aw, ah) == (4, 3) {
        format!("{aw}:{ah}")
    } else {
        format!("{:.2}:1", w as f64 / h as f64)
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}
