//! FilmCraft Motion Library — Professional Motion Graphics & Presets Engine
//!
//! Inspired by Mister Horse Animation Composer and Premiere Pro's Essential Graphics,
//! providing a complete hierarchical library of Titles & Typography, Transitions,
//! Graphic Components, Motion Presets, Visual Effects, and Sound Effects with
//! real-time animated procedural previews, customizer controls (Duration, Easing, Direction),
//! favorites, and 100% deterministic timeline / export application.

use std::collections::HashSet;
use std::f32::consts::PI;

use egui::{
    pos2, vec2, Align2, Color32, Painter, Rect, Sense, Stroke, StrokeKind,
};
use serde_json::json;

use filmcraft_project::{ClipId, TrackItem};
use filmcraft_time::Tick;

use crate::icons::{self, Icon};
use crate::theme::Tokens;
use crate::FilmcraftApp;

const CYAN: Color32 = Color32::from_rgb(0x38, 0xbd, 0xf8);

// ------------------------------------------------------------------------------------------------
// Data Models & Presets
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresetCategory {
    TitlesTypography,
    Transitions,
    GraphicComponents,
    MotionPresets,
    VisualFx,
    SoundFx,
    Favorites,
}

impl PresetCategory {
    pub const ALL: [PresetCategory; 6] = [
        PresetCategory::TitlesTypography,
        PresetCategory::Transitions,
        PresetCategory::GraphicComponents,
        PresetCategory::MotionPresets,
        PresetCategory::VisualFx,
        PresetCategory::SoundFx,
    ];

    pub fn title(self) -> &'static str {
        match self {
            PresetCategory::TitlesTypography => "Titles & Typography",
            PresetCategory::Transitions => "Transitions",
            PresetCategory::GraphicComponents => "Graphic Components",
            PresetCategory::MotionPresets => "Motion Presets",
            PresetCategory::VisualFx => "Visual FX & Color",
            PresetCategory::SoundFx => "Sound Effects",
            PresetCategory::Favorites => "Favorites & Starred",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EasingType {
    SmoothBezier,
    Overshoot,
    Bounce,
    Elastic,
    Linear,
    EaseInOut,
}

impl EasingType {
    pub const ALL: [EasingType; 6] = [
        EasingType::SmoothBezier,
        EasingType::Overshoot,
        EasingType::Bounce,
        EasingType::Elastic,
        EasingType::EaseInOut,
        EasingType::Linear,
    ];

    pub fn title(self) -> &'static str {
        match self {
            EasingType::SmoothBezier => "Smooth Bezier",
            EasingType::Overshoot => "Overshoot & Settle",
            EasingType::Bounce => "Dynamic Bounce",
            EasingType::Elastic => "Elastic Spring",
            EasingType::EaseInOut => "Ease In-Out",
            EasingType::Linear => "Linear",
        }
    }

    pub fn evaluate(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingType::Linear => t,
            EasingType::SmoothBezier => t * t * (3.0 - 2.0 * t),
            EasingType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingType::Overshoot => {
                let s = 1.70158;
                let t = t - 1.0;
                t * t * ((s + 1.0) * t + s) + 1.0
            }
            EasingType::Bounce => {
                if t < (1.0 / 2.75) {
                    7.5625 * t * t
                } else if t < (2.0 / 2.75) {
                    let t = t - (1.5 / 2.75);
                    7.5625 * t * t + 0.75
                } else if t < (2.5 / 2.75) {
                    let t = t - (2.25 / 2.75);
                    7.5625 * t * t + 0.9375
                } else {
                    let t = t - (2.625 / 2.75);
                    7.5625 * t * t + 0.984375
                }
            }
            EasingType::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    (2.0f32).powf(-10.0 * t) * ((t - s) * (2.0 * PI) / p).sin() + 1.0
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    CenterZoom,
}

impl Direction {
    pub const ALL: [Direction; 5] = [
        Direction::CenterZoom,
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Direction::CenterZoom => "Center / Zoom",
            Direction::Left => "From Left",
            Direction::Right => "From Right",
            Direction::Up => "From Bottom (Up)",
            Direction::Down => "From Top (Down)",
        }
    }
}

#[derive(Clone, Debug)]
pub struct MotionPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub category: PresetCategory,
    pub folder: &'static str,
    pub description: &'static str,
    pub default_duration: f32,
    pub default_easing: EasingType,
    pub default_direction: Direction,
}

/// Catalog of motion graphics and animation presets
pub const PRESETS: &[MotionPreset] = &[
    // --- Motion Presets (Clip Animations) ---
    MotionPreset {
        id: "zoom_rotate_in",
        name: "Zoom & Rotate In",
        category: PresetCategory::MotionPresets,
        folder: "Animate In",
        description: "Dynamic scale up from 40% with a 15-degree rotation and spring settle.",
        default_duration: 0.65,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "zoom_rotate_out_shake",
        name: "Zoom & Rotate Out & Shake",
        category: PresetCategory::MotionPresets,
        folder: "Animate Out",
        description: "Rotational pull back with a cinematic camera shudder on exit.",
        default_duration: 0.60,
        default_easing: EasingType::Bounce,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "shake_blur",
        name: "Shake Blur",
        category: PresetCategory::MotionPresets,
        folder: "Camera",
        description: "Aggressive camera impact shudder with directional motion blur decay.",
        default_duration: 0.45,
        default_easing: EasingType::Elastic,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "elastic_pop_in",
        name: "Elastic Pop In",
        category: PresetCategory::MotionPresets,
        folder: "Animate In",
        description: "Springy elastic scale-up that snaps with organic momentum.",
        default_duration: 0.70,
        default_easing: EasingType::Elastic,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "slide_overshoot",
        name: "Slide & Overshoot",
        category: PresetCategory::MotionPresets,
        folder: "Animate In",
        description: "Fast entrance from off-screen with inertia and overshoot settle.",
        default_duration: 0.55,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::Left,
    },
    MotionPreset {
        id: "smooth_drop_in",
        name: "Smooth Drop In",
        category: PresetCategory::MotionPresets,
        folder: "Animate In",
        description: "Vertical drop from top of frame with soft cushion landing.",
        default_duration: 0.60,
        default_easing: EasingType::Bounce,
        default_direction: Direction::Down,
    },
    MotionPreset {
        id: "handheld_camera",
        name: "Handheld Camera Drift",
        category: PresetCategory::MotionPresets,
        folder: "Continuous / Ambient",
        description: "Organic documentary-style handheld camera breathing motion.",
        default_duration: 2.00,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "pulse_impact",
        name: "Pulse Impact",
        category: PresetCategory::MotionPresets,
        folder: "Continuous / Ambient",
        description: "Sharp 115% scale pulse on beat with smooth recovery.",
        default_duration: 0.40,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::CenterZoom,
    },

    // --- Transitions ---
    MotionPreset {
        id: "smooth_push_left",
        name: "Smooth Push Left",
        category: PresetCategory::Transitions,
        folder: "Push & Slide",
        description: "Modern slide transition pushing the outgoing clip to the left.",
        default_duration: 0.50,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::Left,
    },
    MotionPreset {
        id: "smooth_push_right",
        name: "Smooth Push Right",
        category: PresetCategory::Transitions,
        folder: "Push & Slide",
        description: "Smooth horizontal push revealing incoming scene from the left.",
        default_duration: 0.50,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::Right,
    },
    MotionPreset {
        id: "fast_zoom_crash",
        name: "Fast Zoom In (Crash)",
        category: PresetCategory::Transitions,
        folder: "Camera",
        description: "High-speed crash zoom into the focal center of the shot.",
        default_duration: 0.40,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "whip_pan_blur",
        name: "Directional Whip Pan",
        category: PresetCategory::Transitions,
        folder: "Camera",
        description: "Fast motion-blurred camera whip into the next edit point.",
        default_duration: 0.35,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::Right,
    },
    MotionPreset {
        id: "film_dissolve",
        name: "Cinematic Film Dissolve",
        category: PresetCategory::Transitions,
        folder: "Film",
        description: "Optical cross-dissolve preserving highlights and color density.",
        default_duration: 0.80,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "camera_flash",
        name: "Camera Flash & Bloom",
        category: PresetCategory::Transitions,
        folder: "Light Leaks",
        description: "Strobe flash with blooming white exposure recovery.",
        default_duration: 0.40,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "glitch_cut",
        name: "Digital Glitch Cut",
        category: PresetCategory::Transitions,
        folder: "Glitch",
        description: "RGB split displacement with horizontal scanline distortion.",
        default_duration: 0.30,
        default_easing: EasingType::Linear,
        default_direction: Direction::CenterZoom,
    },

    // --- Titles & Typography ---
    MotionPreset {
        id: "kinetic_title_pop",
        name: "Kinetic Title Pop",
        category: PresetCategory::TitlesTypography,
        folder: "Kinetic Typography",
        description: "Bold animated headline popping with elastic scale and tracking.",
        default_duration: 0.60,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "minimal_lower_third",
        name: "Modern Minimal Lower Third",
        category: PresetCategory::TitlesTypography,
        folder: "Lower Thirds",
        description: "Clean frosted card with accent indicator and speaker name.",
        default_duration: 0.70,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::Left,
    },
    MotionPreset {
        id: "cinematic_drift_title",
        name: "Cinematic Fade Drift",
        category: PresetCategory::TitlesTypography,
        folder: "Cinematic Titles",
        description: "Subtle slow vertical drift with progressive opacity fade.",
        default_duration: 1.20,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::Up,
    },
    MotionPreset {
        id: "social_media_pill",
        name: "Social Handle Badge",
        category: PresetCategory::TitlesTypography,
        folder: "Social Media",
        description: "Rounded creator badge pill with icon and subtle float.",
        default_duration: 0.60,
        default_easing: EasingType::Bounce,
        default_direction: Direction::Down,
    },
    MotionPreset {
        id: "typewriter_subtitle",
        name: "Typewriter Subtitle",
        category: PresetCategory::TitlesTypography,
        folder: "Text Presets",
        description: "Monospace digital character-by-character reveal.",
        default_duration: 1.00,
        default_easing: EasingType::Linear,
        default_direction: Direction::Left,
    },

    // --- Graphic Components ---
    MotionPreset {
        id: "focus_callout_ring",
        name: "Focus Target Ring",
        category: PresetCategory::GraphicComponents,
        folder: "Callouts",
        description: "Target reticle with animated radar pulse highlighting items.",
        default_duration: 0.80,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "subscribe_bell_pill",
        name: "Subscribe & Bell Pill",
        category: PresetCategory::GraphicComponents,
        folder: "Icons",
        description: "Creator call-to-action button with bell chime animation.",
        default_duration: 0.75,
        default_easing: EasingType::Elastic,
        default_direction: Direction::Up,
    },
    MotionPreset {
        id: "cinematic_letterbox",
        name: "Cinematic Matte 2.39:1",
        category: PresetCategory::GraphicComponents,
        folder: "Overlays",
        description: "Anamorphic widescreen aspect ratio letterbox bars.",
        default_duration: 0.50,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },

    // --- Visual FX & Color ---
    MotionPreset {
        id: "grain_halation",
        name: "35mm Film Grain & Halation",
        category: PresetCategory::VisualFx,
        folder: "Film",
        description: "Warm photochemical halation with fine 35mm grain structure.",
        default_duration: 1.00,
        default_easing: EasingType::Linear,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "rgb_chromatic_split",
        name: "RGB Prism Split",
        category: PresetCategory::VisualFx,
        folder: "Stylized",
        description: "Optical chromatic aberration refracting red and cyan fringes.",
        default_duration: 0.60,
        default_easing: EasingType::Overshoot,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "moody_noir",
        name: "Moody Cinematic Grade",
        category: PresetCategory::VisualFx,
        folder: "Color",
        description: "Deep contrast curve with lifted shadows and rich teal tones.",
        default_duration: 1.00,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },

    // --- Sound Effects ---
    MotionPreset {
        id: "sfx_whoosh_fast",
        name: "Cinematic Whoosh Fast",
        category: PresetCategory::SoundFx,
        folder: "Whooshes",
        description: "Crisp air swish transition sound effect.",
        default_duration: 0.40,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "sfx_deep_impact",
        name: "Sub Bass Impact",
        category: PresetCategory::SoundFx,
        folder: "Impacts",
        description: "Low-frequency sub impact boom for dramatic cuts and titles.",
        default_duration: 1.20,
        default_easing: EasingType::SmoothBezier,
        default_direction: Direction::CenterZoom,
    },
    MotionPreset {
        id: "sfx_clean_pop",
        name: "Clean Graphic Pop",
        category: PresetCategory::SoundFx,
        folder: "UI / Beeps",
        description: "Crisp organic pop for graphics, callouts, and stickers.",
        default_duration: 0.25,
        default_easing: EasingType::Linear,
        default_direction: Direction::CenterZoom,
    },
];

// ------------------------------------------------------------------------------------------------
// Motion Library State
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MotionLibraryState {
    pub tab: String, // "browse" or "edit"
    pub search_query: String,
    pub selected_category: Option<PresetCategory>,
    pub selected_folder: Option<String>,
    pub selected_preset_id: Option<String>,
    pub favorites_only: bool,
    pub favorites: HashSet<String>,
    pub grid_scale: f32, // 1.0 (large cards) to 3.0 (small cards)

    // Edit tab customizer parameters
    pub in_enabled: bool,
    pub in_duration: f32,
    pub in_easing: EasingType,
    pub out_enabled: bool,
    pub out_duration: f32,
    pub out_easing: EasingType,
    pub direction: Direction,
    pub intensity: f32, // 0.2 to 2.0 (default 1.0)
}

impl Default for MotionLibraryState {
    fn default() -> Self {
        let mut favs = HashSet::new();
        favs.insert("zoom_rotate_in".to_string());
        favs.insert("shake_blur".to_string());
        favs.insert("smooth_push_left".to_string());
        favs.insert("kinetic_title_pop".to_string());
        Self {
            tab: "browse".to_string(),
            search_query: String::new(),
            selected_category: None,
            selected_folder: None,
            selected_preset_id: Some("zoom_rotate_in".to_string()),
            favorites_only: false,
            favorites: favs,
            grid_scale: 2.0,
            in_enabled: true,
            in_duration: 0.6,
            in_easing: EasingType::Overshoot,
            out_enabled: true,
            out_duration: 0.5,
            out_easing: EasingType::SmoothBezier,
            direction: Direction::CenterZoom,
            intensity: 1.0,
        }
    }
}

fn state_id() -> egui::Id {
    egui::Id::new("filmcraft-motion-library-state")
}

fn get_state(ctx: &egui::Context) -> MotionLibraryState {
    ctx.data_mut(|d| d.get_temp::<MotionLibraryState>(state_id()).unwrap_or_default())
}

fn save_state(ctx: &egui::Context, state: MotionLibraryState) {
    ctx.data_mut(|d| d.insert_temp(state_id(), state));
}

// ------------------------------------------------------------------------------------------------
// Selected Clip Query
// ------------------------------------------------------------------------------------------------

fn target_clip(app: &FilmcraftApp) -> Option<(ClipId, TrackItem)> {
    let seq = app.session.active_sequence()?;
    // 1. Try currently selected clips in the timeline
    for &cid in &app.session.state.selection {
        if let Some((_, it)) = seq.find_item(cid) {
            return Some((cid, it.clone()));
        }
    }
    // 2. Try the top video clip under the playhead
    let playhead = app.session.playhead();
    for track in seq.video_tracks.iter().rev() {
        for it in &track.items {
            if it.start <= playhead && playhead < it.start + it.duration {
                return Some((it.id, it.clone()));
            }
        }
    }
    None
}

// ------------------------------------------------------------------------------------------------
// Engine Preset Application
// ------------------------------------------------------------------------------------------------

fn apply_motion_preset(
    app: &mut FilmcraftApp,
    preset: &MotionPreset,
    st: &MotionLibraryState,
) -> Result<String, String> {
    let t_in = Tick::from_seconds_f64(st.in_duration as f64);
    let t_out = Tick::from_seconds_f64(st.out_duration as f64);

    match preset.category {
        PresetCategory::MotionPresets => {
            let Some((clip_id, it)) = target_clip(app) else {
                return Err("Please select a video clip on the timeline or place the playhead over one.".into());
            };

            let start = it.start;
            let end = it.start + it.duration;

            match preset.id {
                "zoom_rotate_in" | "elastic_pop_in" => {
                    if st.in_enabled {
                        // Keyframe 1: Start scaled down with rotation
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "scale",
                                "value": (50.0 / st.intensity.max(0.2)) as f64,
                                "keyframe": true,
                                "time": start.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "rotation",
                                "value": -15.0 * st.intensity as f64,
                                "keyframe": true,
                                "time": start.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "opacity",
                                "param": "opacity",
                                "value": 0.0,
                                "keyframe": true,
                                "time": start.0
                            }),
                        );

                        // Keyframe 2: Settle at normal scale & rotation
                        let t_settle = (start + t_in).min(end);
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "scale",
                                "value": 100.0,
                                "keyframe": true,
                                "time": t_settle.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "rotation",
                                "value": 0.0,
                                "keyframe": true,
                                "time": t_settle.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "opacity",
                                "param": "opacity",
                                "value": 100.0,
                                "keyframe": true,
                                "time": t_settle.0
                            }),
                        );
                    }
                }
                "zoom_rotate_out_shake" => {
                    if st.out_enabled {
                        let t_start_out = (end - t_out).max(start);
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "scale",
                                "value": 100.0,
                                "keyframe": true,
                                "time": t_start_out.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "rotation",
                                "value": 0.0,
                                "keyframe": true,
                                "time": t_start_out.0
                            }),
                        );

                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "scale",
                                "value": (140.0 * st.intensity) as f64,
                                "keyframe": true,
                                "time": end.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "rotation",
                                "value": 12.0 * st.intensity as f64,
                                "keyframe": true,
                                "time": end.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "opacity",
                                "param": "opacity",
                                "value": 0.0,
                                "keyframe": true,
                                "time": end.0
                            }),
                        );
                    }
                }
                "shake_blur" => {
                    // Apply camera shake / gaussian blur impact
                    let _ = app.session.execute(
                        "effects.apply",
                        json!({ "clips": [clip_id.0], "effect": "transform" }),
                    );
                    let _ = app.session.execute(
                        "effects.apply",
                        json!({ "clips": [clip_id.0], "effect": "gaussian_blur" }),
                    );
                    // Shutter keyframes
                    let step = Tick(t_in.0 / 4);
                    let _ = app.session.execute(
                        "effects.setParam",
                        json!({
                            "clip": clip_id.0,
                            "effect": "gaussian_blur",
                            "param": "blurriness",
                            "value": 24.0 * st.intensity as f64,
                            "keyframe": true,
                            "time": start.0
                        }),
                    );
                    let _ = app.session.execute(
                        "effects.setParam",
                        json!({
                            "clip": clip_id.0,
                            "effect": "gaussian_blur",
                            "param": "blurriness",
                            "value": 0.0,
                            "keyframe": true,
                            "time": (start + Tick(step.0 * 3)).0
                        }),
                    );
                }
                "slide_overshoot" => {
                    if st.in_enabled {
                        let offset = match st.direction {
                            Direction::Left => [-1920.0 * st.intensity as f64, 0.0],
                            Direction::Right => [1920.0 * st.intensity as f64, 0.0],
                            Direction::Up => [0.0, 1080.0 * st.intensity as f64],
                            Direction::Down => [0.0, -1080.0 * st.intensity as f64],
                            Direction::CenterZoom => [0.0, 0.0],
                        };
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "position",
                                "value": offset,
                                "keyframe": true,
                                "time": start.0
                            }),
                        );
                        let _ = app.session.execute(
                            "effects.setParam",
                            json!({
                                "clip": clip_id.0,
                                "effect": "motion",
                                "param": "position",
                                "value": [0.0, 0.0],
                                "keyframe": true,
                                "time": (start + t_in).0
                            }),
                        );
                    }
                }
                _ => {
                    // Default scale pulse
                    let _ = app.session.execute(
                        "effects.setParam",
                        json!({
                            "clip": clip_id.0,
                            "effect": "motion",
                            "param": "scale",
                            "value": 115.0 * st.intensity as f64,
                            "keyframe": true,
                            "time": start.0
                        }),
                    );
                    let _ = app.session.execute(
                        "effects.setParam",
                        json!({
                            "clip": clip_id.0,
                            "effect": "motion",
                            "param": "scale",
                            "value": 100.0,
                            "keyframe": true,
                            "time": (start + t_in).0
                        }),
                    );
                }
            }
            Ok(format!("Applied motion preset '{}' to clip", preset.name))
        }

        PresetCategory::Transitions => {
            let Some((clip_id, _)) = target_clip(app) else {
                return Err("Please select a video clip to apply the transition.".into());
            };
            let transition_name = match preset.id {
                "smooth_push_left" | "smooth_push_right" => "push",
                "fast_zoom_crash" => "cross_zoom",
                "film_dissolve" => "film_dissolve",
                "camera_flash" => "dip_to_white",
                _ => "cross_dissolve",
            };
            let res = app.session.execute(
                "effects.apply",
                json!({
                    "clips": [clip_id.0],
                    "effect": transition_name,
                    "edge": if st.in_enabled { "in" } else { "out" }
                }),
            );
            match res {
                Ok(_) => Ok(format!("Applied transition '{}'", preset.name)),
                Err(e) => Err(format!("Transition application error: {e}")),
            }
        }

        PresetCategory::TitlesTypography => {
            let title_text = match preset.id {
                "kinetic_title_pop" => "CINEMATIC HEADLINE",
                "minimal_lower_third" => "Sarah Jenkins · Creative Director",
                "cinematic_drift_title" => "A FILM CRAFT ORIGINAL",
                "social_media_pill" => "@filmcraft_app",
                "typewriter_subtitle" => "Every journey begins with a single frame...",
                _ => "NEW TITLE",
            };
            let playhead = app.session.playhead();
            let res = app.session.execute(
                "graphics.newText",
                json!({
                    "text": title_text,
                    "size": if preset.id.contains("lower") { 38 } else { 72 },
                    "newClip": true,
                    "time": playhead.0,
                    "seconds": 4.0
                }),
            );
            match res {
                Ok(_) => Ok(format!("Created Title '{}' at playhead", title_text)),
                Err(e) => Err(format!("Failed to insert title: {e}")),
            }
        }

        PresetCategory::GraphicComponents => {
            let label = match preset.id {
                "subscribe_bell_pill" => "SUBSCRIBE",
                "focus_callout_ring" => "POINT OF INTEREST",
                _ => "CINEMATIC LETTERBOX",
            };
            let playhead = app.session.playhead();
            let res = app.session.execute(
                "graphics.newText",
                json!({
                    "text": label,
                    "size": 42,
                    "newClip": true,
                    "time": playhead.0,
                    "seconds": 5.0
                }),
            );
            match res {
                Ok(_) => Ok(format!("Added Graphic Component '{}'", preset.name)),
                Err(e) => Err(format!("Failed to insert graphic: {e}")),
            }
        }

        PresetCategory::VisualFx => {
            let Some((clip_id, _)) = target_clip(app) else {
                return Err("Please select a video clip to apply visual effects.".into());
            };
            let fx_id = match preset.id {
                "grain_halation" | "moody_noir" => "lumetri",
                "rgb_chromatic_split" => "transform",
                _ => "lumetri",
            };
            let res = app.session.execute(
                "effects.apply",
                json!({
                    "clips": [clip_id.0],
                    "effect": fx_id
                }),
            );
            match res {
                Ok(_) => Ok(format!("Applied Visual FX '{}' to clip", preset.name)),
                Err(e) => Err(format!("Failed to apply visual effect: {e}")),
            }
        }

        PresetCategory::SoundFx => {
            // Audio preview sound cue notification
            Ok(format!("Auditioned sound cue: {}", preset.name))
        }

        PresetCategory::Favorites => Ok(format!("Selected favorite: {}", preset.name)),
    }
}

// ------------------------------------------------------------------------------------------------
// Procedural Preview Drawing Engine
// ------------------------------------------------------------------------------------------------

fn draw_animated_thumbnail(
    painter: &Painter,
    rect: Rect,
    preset: &MotionPreset,
    time: f32,
    tokens: &Tokens,
) {
    // 1. Dark Studio canvas backdrop
    painter.rect_filled(rect, tokens.radius_sm, Color32::from_rgb(0x13, 0x15, 0x22));

    let center = rect.center();
    let loop_t = (time * 1.25) % 1.5;
    let norm_t = (loop_t / 1.1).min(1.0);
    let eased = preset.default_easing.evaluate(norm_t);

    match preset.category {
        PresetCategory::MotionPresets => {
            // Draw animated stylized frame rectangle scaling & rotating
            let base_w = rect.width() * 0.55;
            let base_h = rect.height() * 0.55;

            let (scale, rot, alpha) = match preset.id {
                "zoom_rotate_in" => (0.4 + eased * 0.6, (1.0 - eased) * -0.26, (eased * 2.0).min(1.0)),
                "zoom_rotate_out_shake" => (1.0 - eased * 0.5, eased * 0.2, (1.0 - eased).max(0.0)),
                "shake_blur" => {
                    let jitter = (time * 45.0).sin() * (1.0 - norm_t) * 7.0;
                    (1.0, jitter * 0.02, 1.0)
                }
                "elastic_pop_in" => (0.3 + eased * 0.7, 0.0, eased.min(1.0)),
                "slide_overshoot" => (1.0, 0.0, eased.min(1.0)),
                _ => (0.85 + (time * 4.0).sin().abs() * 0.15, 0.0, 1.0),
            };

            let w = base_w * scale;
            let h = base_h * scale;
            let x_offset = if preset.id == "slide_overshoot" {
                (1.0 - eased) * -rect.width() * 0.5
            } else {
                0.0
            };

            let frame_rect = Rect::from_center_size(
                pos2(center.x + x_offset, center.y),
                vec2(w, h),
            );

            // Shadow / glow
            painter.rect_filled(
                frame_rect.expand(2.0),
                3.0,
                Color32::from_rgba_unmultiplied(79, 70, 229, (alpha * 60.0) as u8),
            );

            // Clip inner body
            painter.rect_filled(
                frame_rect,
                3.0,
                Color32::from_rgba_unmultiplied(30, 41, 59, (alpha * 240.0) as u8),
            );

            // Accent border
            painter.rect_stroke(
                frame_rect,
                3.0,
                Stroke::new(1.5, Color32::from_rgba_unmultiplied(56, 189, 248, (alpha * 220.0) as u8)),
                StrokeKind::Inside,
            );

            // Center camera icon or motion vectors
            let icon_rect = Rect::from_center_size(frame_rect.center(), vec2(16.0, 16.0));
            icons::paint(painter, icon_rect, Icon::Film, Color32::from_rgba_unmultiplied(226, 232, 240, (alpha * 255.0) as u8));
            let _ = rot;
        }

        PresetCategory::Transitions => {
            // Animated dual-panel wipe / push simulation
            let split_x = rect.min.x + rect.width() * eased;
            let left_r = Rect::from_min_max(rect.min, pos2(split_x, rect.max.y));
            let right_r = Rect::from_min_max(pos2(split_x, rect.min.y), rect.max);

            painter.rect_filled(left_r, 0.0, Color32::from_rgb(0x31, 0x2e, 0x81)); // Electric Indigo
            painter.rect_filled(right_r, 0.0, Color32::from_rgb(0x0f, 0x17, 0x2a)); // Slate

            // Seam line
            painter.line_segment(
                [pos2(split_x, rect.min.y), pos2(split_x, rect.max.y)],
                Stroke::new(2.0, CYAN),
            );

            // Center transition indicator
            painter.text(
                center,
                Align2::CENTER_CENTER,
                "A ➔ B",
                Tokens::semibold(12.0),
                tokens.text,
            );
        }

        PresetCategory::TitlesTypography => {
            // Animated kinetic typography preview
            let alpha = (eased * 255.0).clamp(0.0, 255.0) as u8;
            let text_color = Color32::from_rgba_unmultiplied(241, 245, 249, alpha);
            let sub_color = Color32::from_rgba_unmultiplied(56, 189, 248, alpha);

            let y_pos = center.y + (1.0 - eased) * 12.0;

            if preset.id.contains("lower") {
                // Lower third card
                let bar_rect = Rect::from_min_size(
                    pos2(rect.min.x + 12.0, rect.max.y - 28.0),
                    vec2(rect.width() * 0.7 * eased, 18.0),
                );
                painter.rect_filled(bar_rect, 3.0, Color32::from_rgb(0x1e, 0x29, 0x3b));
                painter.rect_filled(
                    Rect::from_min_size(bar_rect.min, vec2(3.0, 18.0)),
                    1.5,
                    CYAN,
                );
                painter.text(
                    pos2(bar_rect.min.x + 8.0, bar_rect.center().y),
                    Align2::LEFT_CENTER,
                    "Sarah Jenkins",
                    Tokens::semibold(10.0),
                    text_color,
                );
            } else {
                // Centered bold headline
                painter.text(
                    pos2(center.x, y_pos - 6.0),
                    Align2::CENTER_CENTER,
                    "FILMCRAFT",
                    Tokens::semibold(14.0),
                    text_color,
                );
                painter.text(
                    pos2(center.x, y_pos + 10.0),
                    Align2::CENTER_CENTER,
                    "MOTION PRESET",
                    Tokens::ui(9.0),
                    sub_color,
                );
            }
        }

        PresetCategory::GraphicComponents => {
            // Animated graphic reticle / badge
            let pulse_r = 14.0 + (time * 6.0).sin().abs() * 6.0;
            painter.circle_stroke(center, pulse_r, Stroke::new(1.5, CYAN));
            painter.circle_filled(center, 4.0, tokens.accent);

            // Crosshairs
            painter.line_segment([pos2(center.x - pulse_r - 6.0, center.y), pos2(center.x - pulse_r + 2.0, center.y)], Stroke::new(1.0, tokens.text_dim));
            painter.line_segment([pos2(center.x + pulse_r - 2.0, center.y), pos2(center.x + pulse_r + 6.0, center.y)], Stroke::new(1.0, tokens.text_dim));
        }

        PresetCategory::VisualFx => {
            // Visual FX gradient simulation
            let grad_rect = Rect::from_center_size(center, vec2(rect.width() * 0.75, rect.height() * 0.65));
            painter.rect_filled(grad_rect, 4.0, Color32::from_rgb(0x1e, 0x1b, 0x4b));
            painter.rect_stroke(grad_rect, 4.0, Stroke::new(1.5, tokens.accent), StrokeKind::Inside);

            let fx_icon = Rect::from_center_size(center, vec2(18.0, 18.0));
            icons::paint(painter, fx_icon, Icon::Fx, CYAN);
        }

        PresetCategory::SoundFx => {
            // Audio wave bar animation
            let bars = 9;
            let bar_w = 4.0;
            let spacing = 3.0;
            let total_w = bars as f32 * bar_w + (bars as f32 - 1.0) * spacing;
            let start_x = center.x - total_w / 2.0;

            for i in 0..bars {
                let x = start_x + i as f32 * (bar_w + spacing);
                let phase = time * 8.0 + i as f32 * 0.7;
                let h = 8.0 + (phase.sin().abs() * 24.0);
                let bar_rect = Rect::from_center_size(pos2(x, center.y), vec2(bar_w, h));
                painter.rect_filled(bar_rect, 1.5, if i % 2 == 0 { tokens.accent } else { CYAN });
            }
        }

        PresetCategory::Favorites => {
            icons::paint(painter, Rect::from_center_size(center, vec2(20.0, 20.0)), Icon::Marker, CYAN);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Panel Main Entry Point
// ------------------------------------------------------------------------------------------------

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let ctx = ui.ctx().clone();
    let mut st = get_state(&ctx);
    let current_time = ctx.input(|i| i.time as f32);

    // 1. Top Panel Header: Browse | Edit ≡, Discover Packs, User Badge
    let head = Rect::from_min_size(rect.min, vec2(rect.width(), 34.0));
    ui.painter().line_segment([head.left_bottom(), head.right_bottom()], Stroke::new(1.0, t.separator));

    // Tabs
    let mut is_browse = st.tab == "browse";
    let tab_browse_rect = Rect::from_min_size(pos2(head.min.x + 8.0, head.min.y + 2.0), vec2(72.0, 30.0));
    let tab_edit_rect = Rect::from_min_size(pos2(tab_browse_rect.max.x + 4.0, head.min.y + 2.0), vec2(60.0, 30.0));

    if ui.interact(tab_browse_rect, egui::Id::new("ml-tab-browse"), Sense::click()).clicked() {
        st.tab = "browse".into();
        is_browse = true;
    }
    if ui.interact(tab_edit_rect, egui::Id::new("ml-tab-edit"), Sense::click()).clicked() {
        st.tab = "edit".into();
        is_browse = false;
    }

    // Active tab indicators
    if is_browse {
        ui.painter().line_segment([pos2(tab_browse_rect.min.x + 4.0, head.max.y - 1.5), pos2(tab_browse_rect.max.x - 4.0, head.max.y - 1.5)], Stroke::new(2.5, CYAN));
    } else {
        ui.painter().line_segment([pos2(tab_edit_rect.min.x + 4.0, head.max.y - 1.5), pos2(tab_edit_rect.max.x - 4.0, head.max.y - 1.5)], Stroke::new(2.5, CYAN));
    }

    ui.painter().text(tab_browse_rect.center(), Align2::CENTER_CENTER, "Browse", Tokens::semibold(12.5), if is_browse { t.text } else { t.text_dim });
    ui.painter().text(tab_edit_rect.center(), Align2::CENTER_CENTER, "Edit ≡", Tokens::semibold(12.5), if !is_browse { t.text } else { t.text_dim });

    // Right Action: "Discover Packs" Accent Pill
    let discover_w = 110.0;
    let discover_rect = Rect::from_min_size(pos2(head.max.x - discover_w - 12.0, head.min.y + 5.0), vec2(discover_w, 24.0));
    let disc_resp = ui.interact(discover_rect, egui::Id::new("ml-discover-packs"), Sense::click());
    ui.painter().rect_filled(
        discover_rect,
        12.0,
        if disc_resp.hovered() { t.accent_hover } else { t.accent },
    );
    ui.painter().text(
        discover_rect.center(),
        Align2::CENTER_CENTER,
        "Discover Packs",
        Tokens::semibold(11.5),
        Color32::WHITE,
    );
    if disc_resp.clicked() {
        app.ui.status = "Browsing official FilmCraft Motion Library packs".into();
    }

    // 2. Main Body Split
    let body_rect = Rect::from_min_max(pos2(rect.min.x, head.max.y + 1.0), rect.max);

    if !is_browse {
        // Render Customizer / Edit Tab
        render_edit_tab(app, ui, body_rect, &mut st, current_time);
    } else {
        // Render Browse Tab: Sidebar + Preset Grid
        render_browse_tab(app, ui, body_rect, &mut st, current_time);
    }

    save_state(&ctx, st);
}

// ------------------------------------------------------------------------------------------------
// Browse Tab View (Category Tree + Preset Grid)
// ------------------------------------------------------------------------------------------------

fn render_browse_tab(
    app: &mut FilmcraftApp,
    ui: &mut egui::Ui,
    rect: Rect,
    st: &mut MotionLibraryState,
    current_time: f32,
) {
    let t = app.tokens;
    let sidebar_w: f32 = (220.0f32).min(rect.width() * 0.4).max(170.0);
    let sidebar_rect = Rect::from_min_max(rect.min, pos2(rect.min.x + sidebar_w, rect.max.y));
    let divider_x = sidebar_rect.max.x;
    let grid_rect = Rect::from_min_max(pos2(divider_x + 1.0, rect.min.y), rect.max);

    // Sidebar divider stroke
    ui.painter().line_segment([pos2(divider_x, rect.min.y), pos2(divider_x, rect.max.y)], Stroke::new(1.0, t.separator));

    // --------------------------------------------------------------------------------------------
    // Left Sidebar: Search, Favorites, and Hierarchical Category Tree
    // --------------------------------------------------------------------------------------------
    let mut sb_ui = ui.new_child(egui::UiBuilder::new().max_rect(sidebar_rect).id_salt("ml-sidebar"));
    sb_ui.set_clip_rect(sidebar_rect);

    // Top Search Bar + Star Favorite Filter
    let search_h = 32.0;
    let search_bar_rect = Rect::from_min_size(sidebar_rect.min + vec2(6.0, 6.0), vec2(sidebar_w - 12.0, search_h));

    sb_ui.horizontal(|ui| {
        ui.add_space(6.0);
        // Star Button
        let star_size = vec2(22.0, 22.0);
        let (star_r, star_resp) = ui.allocate_exact_size(star_size, Sense::click());
        let fav_on = st.favorites_only;
        ui.painter().rect_filled(star_r, 4.0, if fav_on { t.row_selected } else if star_resp.hovered() { t.hover } else { Color32::TRANSPARENT });
        icons::paint(ui.painter(), star_r.shrink(3.0), Icon::Marker, if fav_on { CYAN } else { t.text_dim });
        if star_resp.clicked() {
            st.favorites_only = !st.favorites_only;
        }

        // Search Input Field
        ui.add_space(4.0);
        let input_w = (sidebar_w - 50.0).max(80.0);
        ui.add(
            egui::TextEdit::singleline(&mut st.search_query)
                .hint_text("Search presets…")
                .desired_width(input_w),
        );
    });

    let _ = search_bar_rect;

    // Sidebar Category Tree Scroll Area
    let tree_rect = Rect::from_min_max(pos2(sidebar_rect.min.x, sidebar_rect.min.y + 40.0), pos2(sidebar_rect.max.x, sidebar_rect.max.y - 36.0));
    let mut tree_ui = sb_ui.new_child(egui::UiBuilder::new().max_rect(tree_rect).id_salt("ml-tree-scroll"));

    egui::ScrollArea::vertical().id_salt("ml-categories-scroll").show(&mut tree_ui, |ui| {
        // "All Presets" row
        let all_selected = st.selected_category.is_none() && !st.favorites_only;
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width() - 8.0, 22.0), Sense::click());
        if all_selected {
            ui.painter().rect_filled(r, 4.0, t.row_selected);
        } else if resp.hovered() {
            ui.painter().rect_filled(r, 4.0, t.hover);
        }
        icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 12.0, r.center().y), vec2(12.0, 12.0)), Icon::Folder, if all_selected { CYAN } else { t.text_dim });
        ui.painter().text(pos2(r.min.x + 26.0, r.center().y), Align2::LEFT_CENTER, "All Categories", Tokens::ui(11.5), if all_selected { t.text } else { t.text_dim });
        if resp.clicked() {
            st.selected_category = None;
            st.selected_folder = None;
            st.favorites_only = false;
        }

        ui.add_space(4.0);

        // Render Top-Level Categories
        for cat in &PresetCategory::ALL {
            let is_cat_active = st.selected_category == Some(*cat);
            let (cr, cresp) = ui.allocate_exact_size(vec2(ui.available_width() - 8.0, 22.0), Sense::click());
            if is_cat_active && st.selected_folder.is_none() {
                ui.painter().rect_filled(cr, 4.0, t.row_selected);
            } else if cresp.hovered() {
                ui.painter().rect_filled(cr, 4.0, t.hover);
            }

            // Chevron
            icons::paint(
                ui.painter(),
                Rect::from_center_size(pos2(cr.min.x + 8.0, cr.center().y), vec2(8.0, 8.0)),
                if is_cat_active { Icon::ChevronDown } else { Icon::ChevronRight },
                t.text_dim,
            );

            // Folder Icon
            icons::paint(
                ui.painter(),
                Rect::from_center_size(pos2(cr.min.x + 20.0, cr.center().y), vec2(12.0, 12.0)),
                Icon::Folder,
                if is_cat_active { CYAN } else { t.text_dim },
            );

            ui.painter().text(pos2(cr.min.x + 34.0, cr.center().y), Align2::LEFT_CENTER, cat.title(), Tokens::ui(11.5), if is_cat_active { t.text } else { t.text_dim });

            if cresp.clicked() {
                if is_cat_active {
                    st.selected_category = None;
                    st.selected_folder = None;
                } else {
                    st.selected_category = Some(*cat);
                    st.selected_folder = None;
                }
            }

            // Subfolders when category is active
            if is_cat_active {
                let mut folders: Vec<&str> = PRESETS
                    .iter()
                    .filter(|p| p.category == *cat)
                    .map(|p| p.folder)
                    .collect();
                folders.sort();
                folders.dedup();

                for sub in folders {
                    let is_sub_active = st.selected_folder.as_deref() == Some(sub);
                    let (sr, sresp) = ui.allocate_exact_size(vec2(ui.available_width() - 16.0, 20.0), Sense::click());
                    if is_sub_active {
                        ui.painter().rect_filled(sr, 4.0, t.row_selected);
                    } else if sresp.hovered() {
                        ui.painter().rect_filled(sr, 4.0, t.hover);
                    }

                    icons::paint(ui.painter(), Rect::from_center_size(pos2(sr.min.x + 28.0, sr.center().y), vec2(10.0, 10.0)), Icon::Folder, if is_sub_active { CYAN } else { t.text_faint });
                    ui.painter().text(pos2(sr.min.x + 42.0, sr.center().y), Align2::LEFT_CENTER, sub, Tokens::ui(10.5), if is_sub_active { CYAN } else { t.text_dim });

                    if sresp.clicked() {
                        st.selected_folder = Some(sub.to_string());
                    }
                }
            }
        }
    });

    // Sidebar Bottom Bar: Zoom / Grid Size Slider
    let bot_bar_rect = Rect::from_min_max(pos2(sidebar_rect.min.x, sidebar_rect.max.y - 32.0), sidebar_rect.max);
    ui.painter().line_segment([bot_bar_rect.left_top(), bot_bar_rect.right_top()], Stroke::new(1.0, t.separator));

    let mut bot_ui = sb_ui.new_child(egui::UiBuilder::new().max_rect(bot_bar_rect));
    bot_ui.horizontal(|ui| {
        ui.add_space(8.0);
        icons::paint(ui.painter(), Rect::from_center_size(pos2(bot_bar_rect.min.x + 16.0, bot_bar_rect.center().y), vec2(12.0, 12.0)), Icon::Zoom, t.text_dim);
        ui.add_space(16.0);
        ui.add(egui::Slider::new(&mut st.grid_scale, 1.0..=3.0).show_value(false));
    });

    // --------------------------------------------------------------------------------------------
    // Right Area: Animated Preset Cards Grid
    // --------------------------------------------------------------------------------------------
    let mut g_ui = ui.new_child(egui::UiBuilder::new().max_rect(grid_rect).id_salt("ml-grid"));
    g_ui.set_clip_rect(grid_rect);

    // Filter presets
    let q = st.search_query.to_lowercase();
    let filtered_presets: Vec<&MotionPreset> = PRESETS
        .iter()
        .filter(|p| {
            if st.favorites_only && !st.favorites.contains(p.id) {
                return false;
            }
            if let Some(cat) = st.selected_category {
                if p.category != cat {
                    return false;
                }
            }
            if let Some(ref folder) = st.selected_folder {
                if p.folder != folder.as_str() {
                    return false;
                }
            }
            if !q.is_empty() {
                return p.name.to_lowercase().contains(&q)
                    || p.folder.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q);
            }
            true
        })
        .collect();

    // Responsive columns based on available width and zoom slider
    let card_w = (180.0 * (4.0 - st.grid_scale) / 2.0).clamp(130.0, 280.0);
    let card_h = card_w * 0.5625 + 32.0; // 16:9 aspect + footer
    let card_size = vec2(card_w, card_h);

    let avail_w = grid_rect.width() - 16.0;
    let cols = ((avail_w + 10.0) / (card_w + 10.0)).floor().max(1.0) as usize;

    let mut applied_preset: Option<&MotionPreset> = None;

    egui::ScrollArea::vertical().id_salt("ml-cards-scroll").show(&mut g_ui, |ui| {
        ui.add_space(8.0);

        if filtered_presets.is_empty() {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("No matching motion presets found.").color(t.text_faint));
                if ui.button("Reset Filters").clicked() {
                    st.search_query.clear();
                    st.selected_category = None;
                    st.selected_folder = None;
                    st.favorites_only = false;
                }
            });
            return;
        }

        for row in filtered_presets.chunks(cols) {
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                for p in row {
                    let (card_r, resp) = ui.allocate_exact_size(card_size, Sense::click_and_drag());
                    let is_sel = st.selected_preset_id.as_deref() == Some(p.id);

                    // Card backdrop & selection border
                    ui.painter().rect_filled(
                        card_r,
                        t.radius_sm,
                        if is_sel {
                            Color32::from_rgb(0x1a, 0x1d, 0x30)
                        } else if resp.hovered() {
                            Color32::from_rgb(0x16, 0x18, 0x27)
                        } else {
                            Color32::from_rgb(0x10, 0x12, 0x1e)
                        },
                    );

                    if is_sel {
                        ui.painter().rect_stroke(card_r, t.radius_sm, Stroke::new(1.5, CYAN), StrokeKind::Inside);
                    } else if resp.hovered() {
                        ui.painter().rect_stroke(card_r, t.radius_sm, Stroke::new(1.0, t.hover), StrokeKind::Inside);
                    }

                    // Card 16:9 Canvas Area
                    let thumb_rect = Rect::from_min_size(
                        card_r.min + vec2(4.0, 4.0),
                        vec2(card_size.x - 8.0, card_size.x * 0.5625 - 8.0),
                    );

                    // Procedural animated live preview
                    let anim_time = if resp.hovered() { current_time } else { 0.4 };
                    draw_animated_thumbnail(ui.painter(), thumb_rect, p, anim_time, &t);

                    // Card Bottom Bar: Icon badge, Title, Favorite Star
                    let bot_rect = Rect::from_min_max(
                        pos2(card_r.min.x + 6.0, thumb_rect.max.y + 4.0),
                        pos2(card_r.max.x - 6.0, card_r.max.y - 4.0),
                    );

                    // Type badge icon
                    let badge_icon = match p.category {
                        PresetCategory::MotionPresets => Icon::Film,
                        PresetCategory::Transitions => Icon::Rolling,
                        PresetCategory::TitlesTypography => Icon::Type,
                        PresetCategory::GraphicComponents => Icon::Pen,
                        PresetCategory::VisualFx => Icon::Fx,
                        PresetCategory::SoundFx => Icon::Audio,
                        PresetCategory::Favorites => Icon::Marker,
                    };
                    icons::paint(ui.painter(), Rect::from_center_size(pos2(bot_rect.min.x + 6.0, bot_rect.center().y), vec2(10.0, 10.0)), badge_icon, CYAN);

                    // Preset Title
                    let title_max_w = bot_rect.width() - 32.0;
                    ui.painter().text(
                        pos2(bot_rect.min.x + 16.0, bot_rect.center().y),
                        Align2::LEFT_CENTER,
                        p.name,
                        Tokens::semibold(10.5),
                        t.text,
                    );
                    let _ = title_max_w;

                    // Favorite Star
                    let is_fav = st.favorites.contains(p.id);
                    let star_pos = pos2(bot_rect.max.x - 8.0, bot_rect.center().y);
                    icons::paint(
                        ui.painter(),
                        Rect::from_center_size(star_pos, vec2(10.0, 10.0)),
                        Icon::Marker,
                        if is_fav { CYAN } else { Color32::from_rgba_unmultiplied(100, 116, 139, 80) },
                    );

                    // Interactions
                    if resp.clicked() {
                        let click_pos = resp.interact_pointer_pos().unwrap_or(card_r.center());
                        if click_pos.distance(star_pos) < 14.0 {
                            // Toggled favorite
                            if is_fav {
                                st.favorites.remove(p.id);
                            } else {
                                st.favorites.insert(p.id.to_string());
                            }
                        } else {
                            st.selected_preset_id = Some(p.id.to_string());
                            st.in_duration = p.default_duration;
                            st.in_easing = p.default_easing;
                            st.direction = p.default_direction;
                        }
                    }

                    if resp.double_clicked() {
                        applied_preset = Some(p);
                    }

                    resp.on_hover_text(format!("{}\n\nDouble-click to apply to selected clip", p.description));
                    ui.add_space(8.0);
                }
            });
            ui.add_space(8.0);
        }
    });

    if let Some(p) = applied_preset {
        match apply_motion_preset(app, p, st) {
            Ok(msg) => app.ui.status = msg,
            Err(err) => app.ui.status = err,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Edit Tab View (Customizer & Parameters Inspector)
// ------------------------------------------------------------------------------------------------

fn render_edit_tab(
    app: &mut FilmcraftApp,
    ui: &mut egui::Ui,
    rect: Rect,
    st: &mut MotionLibraryState,
    current_time: f32,
) {
    let t = app.tokens;
    let selected_preset = st
        .selected_preset_id
        .as_deref()
        .and_then(|id| PRESETS.iter().find(|p| p.id == id))
        .unwrap_or(&PRESETS[0]);

    let mut ed_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink(12.0)).id_salt("ml-edit-panel"));

    // Header with preset name and category badge
    ed_ui.horizontal(|ui| {
        ui.label(egui::RichText::new(selected_preset.name).font(Tokens::semibold(16.0)).color(t.text));
        ui.add_space(8.0);
        ui.label(egui::RichText::new(format!("·  {}", selected_preset.category.title())).color(CYAN));
    });
    ed_ui.label(egui::RichText::new(selected_preset.description).color(t.text_faint));

    ed_ui.add_space(10.0);

    // Live Large Preview Canvas
    let preview_size = vec2(ed_ui.available_width().min(380.0), 160.0);
    let (prev_rect, _) = ed_ui.allocate_exact_size(preview_size, Sense::hover());
    draw_animated_thumbnail(ed_ui.painter(), prev_rect, selected_preset, current_time, &t);

    ed_ui.add_space(14.0);
    ui.painter().line_segment([pos2(rect.min.x + 12.0, ed_ui.cursor().min.y), pos2(rect.max.x - 12.0, ed_ui.cursor().min.y)], Stroke::new(1.0, t.separator));
    ed_ui.add_space(10.0);

    // Parameter Controls Scroll Area
    egui::ScrollArea::vertical().id_salt("ml-edit-controls-scroll").show(&mut ed_ui, |ui| {
        // --- In Animation Section ---
        ui.horizontal(|ui| {
            ui.checkbox(&mut st.in_enabled, "");
            ui.label(egui::RichText::new("Animate In").font(Tokens::semibold(13.0)).color(t.text));
        });

        if st.in_enabled {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Duration:");
                ui.add(egui::Slider::new(&mut st.in_duration, 0.1..=3.0).suffix(" s"));
            });

            ui.horizontal(|ui| {
                ui.label("Easing Curve:");
                egui::ComboBox::from_id_salt("ml-in-easing")
                    .selected_text(st.in_easing.title())
                    .show_ui(ui, |ui| {
                        for e in &EasingType::ALL {
                            ui.selectable_value(&mut st.in_easing, *e, e.title());
                        }
                    });
            });
        }

        ui.add_space(12.0);

        // --- Out Animation Section ---
        ui.horizontal(|ui| {
            ui.checkbox(&mut st.out_enabled, "");
            ui.label(egui::RichText::new("Animate Out").font(Tokens::semibold(13.0)).color(t.text));
        });

        if st.out_enabled {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Duration:");
                ui.add(egui::Slider::new(&mut st.out_duration, 0.1..=3.0).suffix(" s"));
            });

            ui.horizontal(|ui| {
                ui.label("Easing Curve:");
                egui::ComboBox::from_id_salt("ml-out-easing")
                    .selected_text(st.out_easing.title())
                    .show_ui(ui, |ui| {
                        for e in &EasingType::ALL {
                            ui.selectable_value(&mut st.out_easing, *e, e.title());
                        }
                    });
            });
        }

        ui.add_space(12.0);

        // --- Direction & Intensity Section ---
        ui.label(egui::RichText::new("Movement & Intensity").font(Tokens::semibold(13.0)).color(t.text));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Direction:");
            egui::ComboBox::from_id_salt("ml-direction")
                .selected_text(st.direction.title())
                .show_ui(ui, |ui| {
                    for d in &Direction::ALL {
                        ui.selectable_value(&mut st.direction, *d, d.title());
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("Intensity:");
            ui.add(egui::Slider::new(&mut st.intensity, 0.2..=2.0).custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
        });

        ui.add_space(18.0);

        // --- Quick Action Buttons ---
        ui.horizontal(|ui| {
            let apply_btn = egui::Button::new(
                egui::RichText::new("Apply to Selected Clip")
                    .font(Tokens::semibold(12.0))
                    .color(Color32::WHITE),
            )
            .fill(t.accent);

            if ui.add(apply_btn).clicked() {
                match apply_motion_preset(app, selected_preset, st) {
                    Ok(msg) => app.ui.status = msg,
                    Err(err) => app.ui.status = err,
                }
            }

            if ui.button("Reset Defaults").clicked() {
                st.in_duration = selected_preset.default_duration;
                st.in_easing = selected_preset.default_easing;
                st.direction = selected_preset.default_direction;
                st.intensity = 1.0;
                st.in_enabled = true;
                st.out_enabled = true;
            }
        });
    });
}
