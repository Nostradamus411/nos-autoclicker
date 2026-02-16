use egui::{Color32, Stroke, Visuals, epaint::Shadow};

use crate::domain::settings::ThemeVariant;

// ── Neon Green "Mad Scientist" Palette ────────────────────────────────────────

const NEON_GREEN: Color32 = Color32::from_rgb(57, 255, 20);
const NEON_GREEN_DIM: Color32 = Color32::from_rgb(30, 180, 10);
const DARK_BG: Color32 = Color32::from_rgb(15, 15, 20);
const PANEL_BG: Color32 = Color32::from_rgb(22, 22, 30);
const WIDGET_BG: Color32 = Color32::from_rgb(30, 30, 40);
const TEXT_PRIMARY: Color32 = Color32::from_rgb(220, 255, 220);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(140, 200, 140);
const ERROR_RED: Color32 = Color32::from_rgb(255, 60, 60);
const WARNING_AMBER: Color32 = Color32::from_rgb(255, 180, 0);

// ── High Contrast Palette ─────────────────────────────────────────────────────

const HC_BG: Color32 = Color32::from_rgb(0, 0, 0);
const HC_TEXT: Color32 = Color32::from_rgb(255, 255, 255);
const HC_ACCENT: Color32 = Color32::from_rgb(0, 200, 255);
const HC_WIDGET_BG: Color32 = Color32::from_rgb(30, 30, 30);

/// Apply the selected theme to the egui context.
pub fn apply_theme(ctx: &egui::Context, variant: ThemeVariant) {
    match variant {
        ThemeVariant::MadScientistNeon => apply_neon_theme(ctx),
        ThemeVariant::HighContrast => apply_high_contrast_theme(ctx),
    }
}

fn apply_neon_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // Background colors
    visuals.panel_fill = PANEL_BG;
    visuals.window_fill = DARK_BG;
    visuals.extreme_bg_color = DARK_BG;

    // Widget styling
    visuals.widgets.noninteractive.bg_fill = WIDGET_BG;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_SECONDARY);

    visuals.widgets.inactive.bg_fill = WIDGET_BG;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 60, 40);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, NEON_GREEN);

    visuals.widgets.active.bg_fill = Color32::from_rgb(20, 80, 20);
    visuals.widgets.active.fg_stroke = Stroke::new(2.0, NEON_GREEN);

    visuals.widgets.open.bg_fill = WIDGET_BG;
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, NEON_GREEN);

    // Selection
    visuals.selection.bg_fill = Color32::from_rgba_premultiplied(57, 255, 20, 60);
    visuals.selection.stroke = Stroke::new(1.0, NEON_GREEN);

    // Window shadow - neon glow effect
    visuals.window_shadow = Shadow {
        offset: [0, 0].into(),
        blur: 12,
        spread: 2,
        color: Color32::from_rgba_premultiplied(57, 255, 20, 30),
    };

    visuals.window_stroke = Stroke::new(1.0, NEON_GREEN_DIM);

    ctx.set_visuals(visuals);
}

fn apply_high_contrast_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    visuals.panel_fill = HC_BG;
    visuals.window_fill = HC_BG;
    visuals.extreme_bg_color = HC_BG;

    visuals.widgets.noninteractive.bg_fill = HC_WIDGET_BG;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, HC_TEXT);
    visuals.widgets.inactive.bg_fill = HC_WIDGET_BG;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, HC_TEXT);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 50);
    visuals.widgets.hovered.fg_stroke = Stroke::new(2.0, HC_ACCENT);
    visuals.widgets.active.bg_fill = Color32::from_rgb(60, 60, 60);
    visuals.widgets.active.fg_stroke = Stroke::new(2.0, HC_ACCENT);

    visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0, 200, 255, 60);
    visuals.selection.stroke = Stroke::new(2.0, HC_ACCENT);

    ctx.set_visuals(visuals);
}

// Re-export palette colors for UI code
pub const fn neon_green() -> Color32 { NEON_GREEN }
pub const fn error_red() -> Color32 { ERROR_RED }
pub const fn warning_amber() -> Color32 { WARNING_AMBER }
pub const fn text_primary() -> Color32 { TEXT_PRIMARY }
pub const fn text_secondary() -> Color32 { TEXT_SECONDARY }
pub const fn dark_bg() -> Color32 { DARK_BG }
pub const fn panel_bg() -> Color32 { PANEL_BG }
