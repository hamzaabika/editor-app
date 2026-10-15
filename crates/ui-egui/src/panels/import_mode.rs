//! Import mode / Welcome Screen: New Project workflow, Open Project, and real Recent Projects.

use std::path::Path;
use egui::{Align2, Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::FilmcraftApp;
use crate::theme::Tokens;

#[derive(Clone, Debug)]
struct NewProjectModal {
    pub open: bool,
    pub name: String,
    pub location: String,
    pub preset: usize,
    pub fps_idx: usize,
}

impl Default for NewProjectModal {
    fn default() -> Self {
        let default_loc = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".into());
        Self {
            open: false,
            name: "Untitled Project".into(),
            location: default_loc,
            preset: 0,
            fps_idx: 3, // 29.97 fps
        }
    }
}

const PRESETS: &[(&str, u32, u32)] = &[
    ("1080p Landscape (1920 × 1080 — 16:9)", 1920, 1080),
    ("Vertical Video / Reels (1080 × 1920 — 9:16)", 1080, 1920),
    ("Square Post (1080 × 1080 — 1:1)", 1080, 1080),
    ("Portrait Feed (1080 × 1350 — 4:5)", 1080, 1350),
    ("4K UHD Landscape (3840 × 2160 — 16:9)", 3840, 2160),
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
];

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    ui.painter().rect_filled(rect, t.radius, t.panel_bg);

    let modal_id = egui::Id::new("filmcraft_new_project_modal_state");
    let mut modal = ui.ctx().data_mut(|d| d.get_temp::<NewProjectModal>(modal_id)).unwrap_or_default();

    // Top Header with FilmCraft Studio Identity
    let header_y = rect.min.y + 24.0;
    let badge_rect = Rect::from_min_size(pos2(rect.min.x + 32.0, header_y), vec2(28.0, 28.0));
    ui.painter().rect_filled(badge_rect, t.radius_sm, t.accent);
    crate::icons::paint(
        ui.painter(),
        Rect::from_center_size(badge_rect.center(), vec2(16.0, 16.0)),
        crate::icons::Icon::Film,
        Color32::WHITE,
    );

    ui.painter().text(
        pos2(badge_rect.max.x + 10.0, header_y + 14.0),
        Align2::LEFT_CENTER,
        "FILMCRAFT STUDIO",
        Tokens::semibold(18.0),
        t.text,
    );
    ui.painter().text(
        pos2(rect.min.x + 32.0, header_y + 42.0),
        Align2::LEFT_CENTER,
        tl!("Next-Generation Native Video Engine • Ultra-responsive Editing Workspace"),
        Tokens::ui(13.0),
        t.text_dim,
    );

    let ctx = ui.ctx().clone();

    // Action bar buttons: New Project, Open Project, Import Media
    let btn_y = header_y + 68.0;
    let mut btn_x = rect.min.x + 32.0;

    // 1. New Project Button
    let new_btn_r = Rect::from_min_size(pos2(btn_x, btn_y), vec2(154.0, 38.0));
    let new_resp = ui.interact(new_btn_r, egui::Id::new("home-new-project"), Sense::click());
    let new_bg = if new_resp.hovered() { t.accent_hover } else { t.accent };
    ui.painter().rect_filled(new_btn_r, t.radius, new_bg);
    crate::icons::paint(
        ui.painter(),
        Rect::from_center_size(pos2(new_btn_r.min.x + 22.0, new_btn_r.center().y), vec2(16.0, 16.0)),
        crate::icons::Icon::Plus,
        Color32::WHITE,
    );
    ui.painter().text(pos2(new_btn_r.min.x + 40.0, new_btn_r.center().y), Align2::LEFT_CENTER, tl!("New Project"), Tokens::semibold(13.5), Color32::WHITE);
    if new_resp.clicked() {
        modal.open = true;
    }
    btn_x = new_btn_r.max.x + 12.0;

    // 2. Open Project Button
    let open_btn_r = Rect::from_min_size(pos2(btn_x, btn_y), vec2(154.0, 38.0));
    let open_resp = ui.interact(open_btn_r, egui::Id::new("home-open-project"), Sense::click());
    let open_bg = if open_resp.hovered() { t.hover } else { t.tl_header_bg };
    ui.painter().rect_filled(open_btn_r, t.radius, open_bg);
    ui.painter().rect_stroke(open_btn_r, t.radius, Stroke::new(1.0, if open_resp.hovered() { t.focus } else { t.separator }), egui::StrokeKind::Inside);
    crate::icons::paint(
        ui.painter(),
        Rect::from_center_size(pos2(open_btn_r.min.x + 22.0, open_btn_r.center().y), vec2(16.0, 16.0)),
        crate::icons::Icon::Folder,
        t.text,
    );
    ui.painter().text(pos2(open_btn_r.min.x + 40.0, open_btn_r.center().y), Align2::LEFT_CENTER, tl!("Open Project…"), Tokens::semibold(13.5), t.text);
    if open_resp.clicked() {
        let _ = crate::menus::invoke(app, &ctx, "file.open", json!({}));
    }
    btn_x = open_btn_r.max.x + 12.0;

    // 3. Browse Media Files Button
    let imp_btn_r = Rect::from_min_size(pos2(btn_x, btn_y), vec2(154.0, 38.0));
    let imp_resp = ui.interact(imp_btn_r, egui::Id::new("home-import-files"), Sense::click());
    let imp_bg = if imp_resp.hovered() { t.hover } else { t.tl_header_bg };
    ui.painter().rect_filled(imp_btn_r, t.radius, imp_bg);
    ui.painter().rect_stroke(imp_btn_r, t.radius, Stroke::new(1.0, if imp_resp.hovered() { t.focus } else { t.separator }), egui::StrokeKind::Inside);
    crate::icons::paint(
        ui.painter(),
        Rect::from_center_size(pos2(imp_btn_r.min.x + 22.0, imp_btn_r.center().y), vec2(16.0, 16.0)),
        crate::icons::Icon::Film,
        t.text,
    );
    ui.painter().text(pos2(imp_btn_r.min.x + 40.0, imp_btn_r.center().y), Align2::LEFT_CENTER, tl!("Import Media…"), Tokens::semibold(13.5), t.text);
    if imp_resp.clicked() {
        let _ = crate::menus::invoke(app, &ctx, "file.import", json!({}));
        app.ui.mode = crate::state::Mode::Edit;
    }

    // Quick Start Sequence Cards
    let quick_y = btn_y + 54.0;
    ui.painter().text(
        pos2(rect.min.x + 32.0, quick_y + 8.0),
        Align2::LEFT_CENTER,
        tl!("Quick Start Format Presets"),
        Tokens::semibold(13.5),
        t.text_dim,
    );

    let quick_presets = [
        ("16:9 Cinema", "1920 × 1080", 0usize),
        ("9:16 Vertical", "1080 × 1920", 1usize),
        ("1:1 Social", "1080 × 1080", 2usize),
        ("4K Ultra HD", "3840 × 2160", 4usize),
    ];
    let mut qx = rect.min.x + 32.0;
    let card_w = 142.0;
    let card_h = 56.0;
    for (title, res, preset_idx) in quick_presets {
        let card_r = Rect::from_min_size(pos2(qx, quick_y + 20.0), vec2(card_w, card_h));
        let c_resp = ui.interact(card_r, egui::Id::new(("home-quick-preset", preset_idx)), Sense::click());
        let bg = if c_resp.hovered() { t.hover } else { t.tl_header_bg };
        ui.painter().rect_filled(card_r, t.radius_sm, bg);
        ui.painter().rect_stroke(card_r, t.radius_sm, Stroke::new(1.0, if c_resp.hovered() { t.focus } else { t.separator }), egui::StrokeKind::Inside);

        ui.painter().text(pos2(card_r.min.x + 12.0, card_r.min.y + 16.0), Align2::LEFT_CENTER, title, Tokens::semibold(12.5), t.text);
        ui.painter().text(pos2(card_r.min.x + 12.0, card_r.min.y + 36.0), Align2::LEFT_CENTER, res, Tokens::ui(11.0), t.text_dim);

        if c_resp.clicked() {
            modal.preset = preset_idx;
            modal.open = true;
        }
        qx += card_w + 10.0;
    }

    // Recent Projects Section
    let list_y = quick_y + 92.0;
    let next_y = recent_projects_table(app, ui, rect, list_y);

    // Community footer
    let logo = crate::brand::paint_wordmark(ui, rect.min + vec2(32.0, next_y), 15.0, app.ui.dark);
    let tx = logo.map_or(rect.min.x + 32.0, |r| r.max.x + 12.0);
    ui.painter().text(pos2(tx, rect.min.y + next_y), Align2::LEFT_CENTER, tl!("Community & Resources"), Tokens::semibold(14.0), t.text);
    let mut lx = rect.min.x + 32.0;
    for (i, (id, label, url)) in crate::links::ALL.iter().take(4).enumerate() {
        let primary = i == 0;
        let label_text = if primary { tl!("Join us on Discord") } else { crate::i18n::t(label) };
        let g = ui.painter().layout_no_wrap(label_text.to_string(), Tokens::ui(12.5), t.text);
        let r = Rect::from_min_size(pos2(lx, rect.min.y + next_y + 18.0), vec2(g.size().x + 40.0, 30.0));
        let resp = ui.interact(r, egui::Id::new(("imp-link", *id)), Sense::click()).on_hover_text(*url);
        let (bg, fg) = if primary {
            (if resp.hovered() { t.accent_hover } else { t.accent }, Color32::WHITE)
        } else {
            (if resp.hovered() { t.hover } else { t.tl_header_bg }, t.text)
        };
        ui.painter().rect_filled(r, t.radius_sm, bg);
        let icon = [crate::icons::Icon::Chat, crate::icons::Icon::Globe, crate::icons::Icon::Globe, crate::icons::Icon::Code][i];
        crate::icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 18.0, r.center().y), vec2(14.0, 14.0)), icon, fg);
        ui.painter().galley(pos2(r.min.x + 32.0, r.center().y - g.size().y / 2.0), g, fg);
        if resp.clicked() {
            crate::links::open(app, &ctx, url);
        }
        lx = r.max.x + 10.0;
    }

    // Modal: New Project
    if modal.open {
        let mut create_project = false;
        let mut close_modal = false;

        egui::Window::new(tl!("New Project"))
            .id(egui::Id::new("filmcraft_new_project_dialog"))
            .pivot(Align2::CENTER_CENTER)
            .default_pos(crate::widgets::dialog_center(&ctx))
            .default_width(480.0)
            .resizable(false)
            .collapsible(false)
            .show(&ctx, |ui| {
                ui.add_space(8.0);
                ui.label(egui::RichText::new(tl!("Create a new video project")).strong().size(14.0));
                ui.add_space(12.0);

                // Project Name
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(tl!("Project Name:")).size(13.0));
                    ui.add(egui::TextEdit::singleline(&mut modal.name).desired_width(300.0));
                });

                ui.add_space(8.0);

                // Project Location
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(tl!("Location:")).size(13.0));
                    ui.add(egui::TextEdit::singleline(&mut modal.location).desired_width(260.0));
                    if ui.button(tl!("Browse…")).clicked() {
                        app.pick_ui(crate::FileDialog::Folder { at: None }, |_, _| {});
                    }
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // Sequence Settings Preset
                ui.label(egui::RichText::new(tl!("Initial Sequence Preset:")).strong().size(13.0));
                egui::ComboBox::from_id_salt("new_proj_preset_combo")
                    .width(ui.available_width() - 8.0)
                    .selected_text(PRESETS[modal.preset].0)
                    .show_ui(ui, |ui| {
                        for (idx, (label, _, _)) in PRESETS.iter().enumerate() {
                            ui.selectable_value(&mut modal.preset, idx, *label);
                        }
                    });

                ui.add_space(8.0);

                // Frame Rate
                ui.horizontal(|ui| {
                    ui.label(tl!("Frame Rate:"));
                    let current_tb = TIMEBASES.get(modal.fps_idx).map(|t| t.2).unwrap_or("29.97 fps");
                    egui::ComboBox::from_id_salt("new_proj_fps_combo")
                        .selected_text(current_tb)
                        .show_ui(ui, |ui| {
                            for (idx, (_, _, label)) in TIMEBASES.iter().enumerate() {
                                ui.selectable_value(&mut modal.fps_idx, idx, *label);
                            }
                        });
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let create_btn = ui.add(
                            egui::Button::new(egui::RichText::new(tl!("Create Project")).strong().color(Color32::WHITE)).fill(t.accent),
                        );
                        if create_btn.clicked() {
                            create_project = true;
                            close_modal = true;
                        }
                        if ui.button(tl!("Cancel")).clicked() {
                            close_modal = true;
                        }
                    });
                });
            });

        if create_project {
            let (w, h) = (PRESETS[modal.preset].1, PRESETS[modal.preset].2);
            let tb = TIMEBASES[modal.fps_idx];
            let fps = tb.0 as f64 / tb.1 as f64;
            let name = if modal.name.trim().is_empty() { "Untitled Project" } else { modal.name.trim() };
            let _ = app.session.execute("file.newProject", json!({"name": name}));
            let _ = app.session.execute("sequence.settings", json!({
                "width": w,
                "height": h,
                "fps": fps,
                "scaleMotion": true,
            }));
            app.ui.mode = crate::state::Mode::Edit;
            modal.open = false;
        } else if close_modal {
            modal.open = false;
        }
    }

    ui.ctx().data_mut(|d| d.insert_temp(modal_id, modal));
}

const RECENT_ROW_H: f32 = 34.0;

fn recent_projects_table(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect, y: f32) -> f32 {
    let t = app.tokens;
    let paths: Vec<String> = app.session.prefs.general.recent_projects.clone();

    ui.painter().text(rect.min + vec2(32.0, y), Align2::LEFT_CENTER, tl!("Recent Projects"), Tokens::semibold(16.0), t.text);

    if paths.is_empty() {
        let empty_r = Rect::from_min_size(rect.min + vec2(32.0, y + 20.0), vec2(rect.width() - 64.0, 60.0));
        ui.painter().rect_filled(empty_r, t.radius, t.tl_header_bg);
        ui.painter().rect_stroke(empty_r, t.radius, Stroke::new(1.0, t.separator), egui::StrokeKind::Inside);
        ui.painter().text(
            empty_r.center(),
            Align2::CENTER_CENTER,
            tl!("No recent projects. Click 'New Project' or 'Open Project' to get started."),
            Tokens::ui(13.0),
            t.text_dim,
        );
        return y + 100.0;
    }

    // Table Header
    let table_w = (rect.width() - 64.0).max(400.0);
    let table_rect = Rect::from_min_size(rect.min + vec2(32.0, y + 20.0), vec2(table_w, (paths.len().min(8) as f32 + 1.0) * RECENT_ROW_H));
    ui.painter().rect_filled(table_rect, t.radius, t.tl_header_bg);
    ui.painter().rect_stroke(table_rect, t.radius, Stroke::new(1.0, t.separator), egui::StrokeKind::Inside);

    let mut open = None;

    for (i, path) in paths.iter().take(8).enumerate() {
        let row_top = table_rect.min.y + i as f32 * RECENT_ROW_H;
        let row = Rect::from_min_size(pos2(table_rect.min.x, row_top), vec2(table_w, RECENT_ROW_H));
        let resp = ui.interact(row, egui::Id::new(("home-recent", i)), Sense::click()).on_hover_text(path.as_str());

        if resp.hovered() {
            ui.painter().rect_filled(row.shrink2(vec2(2.0, 1.0)), t.radius_sm, t.hover);
        }

        let p = ui.painter().with_clip_rect(row);
        let name = crate::menus::recent_name(path);

        // Icon
        crate::icons::paint(
            &p,
            Rect::from_center_size(pos2(row.min.x + 20.0, row.center().y), vec2(15.0, 15.0)),
            crate::icons::Icon::Sequence,
            if resp.hovered() { t.accent } else { t.text_dim },
        );

        // Name
        let name_pos = pos2(row.min.x + 40.0, row.center().y);
        let name_rect = p.text(name_pos, Align2::LEFT_CENTER, &name, Tokens::semibold(13.0), if resp.hovered() { t.accent_hover } else { t.text });

        // Folder Path
        if let Some(folder) = Path::new(path).parent() {
            let path_pos = pos2(name_rect.max.x + 16.0, row.center().y);
            p.text(path_pos, Align2::LEFT_CENTER, folder.to_string_lossy(), Tokens::ui(11.5), t.text_dim);
        }

        // File metadata: file size & last modified
        if let Ok(meta) = std::fs::metadata(path) {
            let size_kb = meta.len() / 1024;
            let size_str = if size_kb > 1024 {
                format!("{:.1} MB", size_kb as f64 / 1024.0)
            } else {
                format!("{size_kb} KB")
            };
            p.text(pos2(row.max.x - 24.0, row.center().y), Align2::RIGHT_CENTER, size_str, Tokens::ui(11.5), t.text_dim);
        }

        if resp.clicked() {
            open = Some(path.clone());
        }
    }

    if let Some(path) = open {
        let ctx = ui.ctx().clone();
        if crate::menus::invoke(app, &ctx, "file.open", json!({"path": path})).is_ok() {
            app.ui.mode = crate::state::Mode::Edit;
        }
    }

    table_rect.max.y + 36.0
}
