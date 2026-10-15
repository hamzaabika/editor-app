//! The FilmCraft Studio Header: Brand mark on the left, streamlined workspace pills,
//! document status indicator, and quick studio actions on the right.

use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};

use crate::FilmcraftApp;
use crate::icons::{self, Icon};
use crate::state::Mode;
use crate::theme::Tokens;

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let p = ui.painter().clone();
    p.rect_filled(rect, 0.0, t.header_bg);
    p.line_segment([rect.left_bottom(), rect.right_bottom()], Stroke::new(1.0, t.separator));

    let drag = ui.interact(rect, egui::Id::new("header-drag"), Sense::click_and_drag());
    if drag.drag_started() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if drag.double_clicked() {
        let max = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
    }

    let mut x = rect.min.x + if app.integrated_titlebar { 104.0 } else { 12.0 };

    // --- 1. Brand Mark & Home Hub ---
    let brand_w = 110.0;
    let brand_r = Rect::from_min_size(pos2(x, rect.center().y - 14.0), vec2(brand_w, 28.0));
    let brand_resp = ui.interact(brand_r, egui::Id::new("hdr-home"), Sense::click()).on_hover_text(tl!("Home / Project Hub"));
    app.auto.add("header.home", brand_r, "Home");
    if brand_resp.hovered() {
        p.rect_filled(brand_r, 6.0, t.hover);
    }
    // FilmCraft Icon Badge
    let icon_badge = Rect::from_center_size(pos2(brand_r.min.x + 14.0, brand_r.center().y), vec2(20.0, 20.0));
    p.rect_filled(icon_badge, 4.0, t.pill_active_bg);
    icons::paint(&p, icon_badge.shrink(4.0), Icon::Film, t.hot_text);
    // FilmCraft Wordmark
    let title_galley = p.layout_no_wrap("FILMCRAFT".into(), Tokens::semibold(12.5), t.text);
    p.galley(pos2(brand_r.min.x + 30.0, brand_r.center().y - title_galley.size().y / 2.0), title_galley, t.text);
    if brand_resp.clicked() {
        app.ui.mode = Mode::Import;
    }
    x = brand_r.max.x + 14.0;

    // --- 2. Workspace & Mode Navigation Pills ---
    // Seamless workflow: Studio, Color, Audio, Effects, Delivery
    let nav_items = [
        (Mode::Edit, "Editing", "Studio", tl!("Studio"), Icon::Sequence),
        (Mode::Edit, "Color", "Color", tl!("Color"), Icon::Sun),
        (Mode::Edit, "Audio", "Audio", tl!("Audio"), Icon::Audio),
        (Mode::Edit, "Effects", "Effects", tl!("Effects"), Icon::Sparkle),
        (Mode::Export, "Export", "Export", tl!("Export"), Icon::Export),
    ];

    let pill_group_id = egui::Id::new("hdr-workspace-pills");
    let pill_h = 24.0;
    for (m, ws_name, key, label, icon) in nav_items {
        let is_current = if m == Mode::Export {
            app.ui.mode == Mode::Export
        } else {
            app.ui.mode == Mode::Edit && app.ui.workspace == ws_name
        };
        let text_g = p.layout_no_wrap(label.to_string(), Tokens::ui(12.0), if is_current { t.text } else { t.text_dim });
        let item_w = text_g.size().x + 28.0;
        let item_r = Rect::from_min_size(pos2(x, rect.center().y - pill_h / 2.0), vec2(item_w, pill_h));
        let resp = ui.interact(item_r, pill_group_id.with(key), Sense::click());
        let legacy_key = match key {
            "Studio" => "edit",
            "Export" => "export",
            other => other,
        };
        app.auto.add(&format!("header.mode.{}", legacy_key.to_ascii_lowercase()), item_r, label);

        if is_current {
            p.rect_filled(item_r, 6.0, t.pill_active_bg);
            p.rect_stroke(item_r, 6.0, Stroke::new(1.0, t.focus), egui::StrokeKind::Inside);
        } else if resp.hovered() {
            p.rect_filled(item_r, 6.0, t.hover);
        }

        let icon_r = Rect::from_center_size(pos2(item_r.min.x + 12.0, item_r.center().y), vec2(12.0, 12.0));
        icons::paint(&p, icon_r, icon, if is_current { t.hot_text } else { t.text_dim });

        let tx = item_r.min.x + 22.0;
        let tc = if is_current { t.text } else if resp.hovered() { t.tab_text_active } else { t.text_dim };
        p.galley(pos2(tx, item_r.center().y - text_g.size().y / 2.0), text_g, tc);

        if resp.clicked() {
            if m == Mode::Export {
                app.ui.mode = Mode::Export;
            } else {
                app.ui.mode = Mode::Edit;
                app.set_workspace(ws_name);
            }
        }
        x += item_w + 6.0;
    }

    let mut left_end = x + 10.0;

    // --- 3. In-window Menus ---
    if app.ui.show_menu_bar {
        let menu_rect = Rect::from_min_max(pos2(x + 6.0, rect.min.y + 6.0), pos2(x + 480.0, rect.max.y - 6.0));
        let mut mu = ui.new_child(egui::UiBuilder::new().max_rect(menu_rect).layout(egui::Layout::left_to_right(egui::Align::Center)));
        mu.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        mu.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        mu.style_mut().visuals.override_text_color = Some(t.text_dim);
        crate::menus::menu_bar(app, &mut mu);
        left_end = mu.min_rect().max.x;
    }

    // --- 4. Right Utility Cluster ---
    let mut rx = rect.max.x - 12.0;

    // Helper for sleek modern buttons
    let mut btn = |ui: &mut egui::Ui, icon: Icon, id: &str, tip: &str, app: &mut FilmcraftApp| -> egui::Response {
        let r = Rect::from_center_size(pos2(rx - 14.0, rect.center().y), vec2(28.0, 28.0));
        rx -= 34.0;
        let resp = ui.interact(r, egui::Id::new(("hdr", id)), Sense::click()).on_hover_text(tip);
        app.auto.add(&format!("header.{id}"), r, tip);
        if resp.hovered() {
            ui.painter().rect_filled(r, 6.0, t.hover);
        }
        icons::paint(ui.painter(), r.shrink(6.0), icon, if resp.hovered() { t.text } else { t.text_dim });
        resp
    };

    if btn(ui, Icon::Fullscreen, "fullscreen", tl!("Full screen"), app).clicked() {
        let fs = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fs));
    }
    if btn(ui, Icon::Speaker, "volume", tl!("Volume"), app).clicked() {
        app.ui.status = tl!("Master volume: use the Audio Track Mixer").into();
    }
    if btn(ui, Icon::Search, "search", tl!("Search Effects & Assets"), app).clicked() {
        app.show_panel(crate::dock::PanelKind::Effects);
    }
    if btn(ui, Icon::Bell, "notifications", tl!("Render Queue / Progress"), app).clicked() {
        app.ui.mode = Mode::Export;
    }

    // Quick Export Pill Button (Prominent Action)
    let qx_w = 90.0;
    let qx_r = Rect::from_min_size(pos2(rx - qx_w + 6.0, rect.center().y - 14.0), vec2(qx_w, 28.0));
    rx -= qx_w + 10.0;
    let qx_resp = ui.interact(qx_r, egui::Id::new("hdr-quick-export"), Sense::click()).on_hover_text(tl!("Quick Export (H.264 / ProRes)"));
    app.auto.add("header.quickExport", qx_r, "Quick Export");
    let qx_bg = if qx_resp.hovered() { t.accent_hover } else { t.accent };
    p.rect_filled(qx_r, 6.0, qx_bg);
    icons::paint(&p, Rect::from_center_size(pos2(qx_r.min.x + 14.0, qx_r.center().y), vec2(14.0, 14.0)), Icon::Export, Color32::WHITE);
    let qx_text = p.layout_no_wrap(tl!("Export").into(), Tokens::semibold(12.0), Color32::WHITE);
    p.galley(pos2(qx_r.min.x + 28.0, qx_r.center().y - qx_text.size().y / 2.0), qx_text, Color32::WHITE);
    if qx_resp.clicked() {
        app.ui.export.quick_open = !app.ui.export.quick_open;
        ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("quick-export-toggled"), true));
    }
    crate::panels::export_mode::quick_export(app, ui.ctx(), pos2(qx_r.min.x - 220.0, rect.max.y + 4.0));

    // Appearance Mode toggle
    let (icon, mode) = match app.session.prefs.appearance.appearance_mode.as_str() {
        "auto" => (Icon::Monitor, tl!("Sync with system")),
        "light" => (Icon::Sun, tl!("Light")),
        _ => (Icon::Moon, tl!("Dark")),
    };
    let tip = format!("{}: {mode}", tl!("Appearance Mode"));
    let appearance = btn(ui, icon, "appearance", &tip, app);
    appearance.context_menu(|ui| crate::panels::settings::appearance_context_menu(app, ui));
    if appearance.clicked() {
        crate::panels::settings::cycle_appearance(app, ui.ctx());
    }

    // Workspaces Manager Button
    let ws_resp = btn(ui, Icon::Workspaces, "workspaces", tl!("Workspaces"), app);

    // --- 5. Project Title & Save Status (Centered) ---
    let is_dirty = app.session.is_dirty();
    let name_str = app.session.project.name.clone();
    let title = if is_dirty { format!("{} •", name_str) } else { name_str };

    let right_limit = rx - 12.0;
    if right_limit > left_end + 60.0 {
        let max_w = right_limit - left_end;
        let mut job = egui::text::LayoutJob::simple_singleline(title, Tokens::semibold(13.0), if is_dirty { t.hot_text } else { t.text });
        job.wrap.max_width = max_w;
        job.wrap.max_rows = 1;
        let galley = p.layout_job(job);
        let gw = galley.size().x;
        let cx = (left_end + right_limit) / 2.0;
        let chip_r = Rect::from_center_size(pos2(cx, rect.center().y), vec2(gw + 24.0, 24.0));
        p.rect_filled(chip_r, 6.0, t.field_bg);
        p.rect_stroke(chip_r, 6.0, Stroke::new(1.0, t.field_border), egui::StrokeKind::Inside);
        // Status indicator dot
        let dot_color = if is_dirty { t.render_yellow } else { t.render_green };
        p.circle_filled(pos2(chip_r.min.x + 10.0, chip_r.center().y), 3.0, dot_color);
        p.galley(pos2(chip_r.min.x + 18.0, chip_r.center().y - galley.size().y / 2.0), galley, t.text);
    }

    // --- 6. Workspaces Dropdown Area ---
    let popup_id = egui::Id::new("workspaces-popup");
    if ws_resp.clicked() {
        ui.ctx().data_mut(|d| d.insert_temp(popup_id, true));
    }
    let open = ui.ctx().data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));
    if open {
        let anchor = pos2(ws_resp.rect.min.x - 120.0, rect.max.y + 4.0);
        let area = egui::Area::new(popup_id.with("area")).order(egui::Order::Foreground).fixed_pos(anchor).show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).corner_radius(8.0).show(ui, |ui| {
                ui.set_min_width(220.0);
                ui.label(egui::RichText::new(tl!("Workspaces")).size(12.0).strong());
                ui.separator();
                for w in crate::dock::names(&app.workspaces) {
                    let sel = app.ui.workspace == w;
                    if ui.selectable_label(sel, crate::i18n::t(&w)).clicked() {
                        app.set_workspace(&w);
                        ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                    }
                }
                ui.separator();
                if ui.button(tl!("Reset to Saved Layout")).clicked() {
                    let n = app.ui.workspace.clone();
                    app.set_workspace(&n);
                    ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                }
            });
        });
        if area.response.clicked_elsewhere() && !ws_resp.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
        }
    }
}

