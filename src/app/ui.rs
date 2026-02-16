use egui::{CentralPanel, RichText, ScrollArea, Ui};
use std::sync::Arc;

use crate::app::theme;
use crate::app::view_model::ViewModel;
use crate::domain::settings::{
    ClickAction, PositionMode, ScopeMode, ThemeVariant, CPS_MAX, CPS_MIN,
    TIMER_MIN_SECONDS, TIMER_MAX_SECONDS, RunState,
};
use crate::engine::click_engine::ClickEngine;
use crate::infra::input_driver;

/// The main eframe application.
pub struct AutoclickerApp {
    pub view_model: ViewModel,
}

impl AutoclickerApp {
    pub fn new(engine: Arc<ClickEngine>) -> Self {
        Self {
            view_model: ViewModel::new(engine),
        }
    }
}

impl eframe::App for AutoclickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        theme::apply_theme(ctx, self.view_model.active_profile.theme_variant);

        // Request continuous repaints while running
        if self.view_model.is_running() {
            ctx.request_repaint();
        }

        // Refresh status from engine
        self.view_model.refresh_status();

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("⚗ NOS AUTOCLICKER")
                        .color(theme::neon_green())
                        .size(24.0),
                );
                ui.add_space(4.0);
            });

            ui.separator();

            // ── Status Bar ────────────────────────────────────────────
            render_status_bar(ui, &self.view_model);

            ui.separator();
            ui.add_space(4.0);

            // ── Click Configuration ───────────────────────────────────
            render_click_config(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Scope Mode ────────────────────────────────────────────
            render_scope_mode(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Conditions (Stationary-Start & Stop-After) ────────────
            render_conditions(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Controls ──────────────────────────────────────────────
            render_controls(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Profiles ──────────────────────────────────────────────
            render_profiles(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Safety Settings ───────────────────────────────────────
            render_safety(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Theme ─────────────────────────────────────────────────
            render_theme_selector(ui, &mut self.view_model);

            ui.add_space(8.0);

            // ── Event Log ─────────────────────────────────────────────
            render_event_log(ui, &self.view_model);

            ui.add_space(8.0);

            // ── Validation Error ──────────────────────────────────────
            if let Some(ref error) = self.view_model.validation_error {
                ui.colored_label(theme::error_red(), format!("⚠ {error}"));
            }
        });
    }
}

fn render_status_bar(ui: &mut Ui, vm: &ViewModel) {
    ui.horizontal(|ui| {
        let state_color = match vm.engine.current_state() {
            crate::domain::settings::RunState::Idle => theme::text_secondary(),
            crate::domain::settings::RunState::Running => theme::neon_green(),
            crate::domain::settings::RunState::WaitingCondition => theme::warning_amber(),
            crate::domain::settings::RunState::Stopped => theme::error_red(),
            crate::domain::settings::RunState::Error => theme::error_red(),
        };

        ui.label(RichText::new("Status:").color(theme::text_secondary()));
        ui.label(RichText::new(&vm.status_text).color(state_color).strong());

        if !vm.stop_reason_text.is_empty() {
            ui.label(
                RichText::new(format!("({})", vm.stop_reason_text))
                    .color(theme::text_secondary())
                    .small(),
            );
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("Scope: {}", vm.active_profile.scope_mode))
                    .color(theme::text_secondary())
                    .small(),
            );
        });
    });

    // Show click count when running or stopped
    if let Ok(session_guard) = vm.engine.session.lock() {
        if let Some(ref session) = *session_guard {
            if session.total_clicks > 0 {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Total clicks: {}", session.total_clicks))
                            .color(theme::text_secondary())
                            .small(),
                    );
                });
            }
        }
    }
}

fn render_click_config(ui: &mut Ui, vm: &mut ViewModel) {
    ui.label(RichText::new("Click Configuration").color(theme::neon_green()).strong());
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label("Click Action:");
        egui::ComboBox::from_id_salt("click_action")
            .selected_text(vm.active_profile.click_action.to_string())
            .show_ui(ui, |ui| {
                for action in [
                    ClickAction::Left,
                    ClickAction::Right,
                    ClickAction::Middle,
                    ClickAction::Double,
                ] {
                    ui.selectable_value(
                        &mut vm.active_profile.click_action,
                        action,
                        action.to_string(),
                    );
                }
            });
    });

    ui.horizontal(|ui| {
        ui.label("Clicks/sec (CPS):");
        let response = ui.text_edit_singleline(&mut vm.cps_text);
        ui.label(
            RichText::new(format!("[{CPS_MIN}–{CPS_MAX}]"))
                .color(theme::text_secondary())
                .small(),
        );
        if response.changed() {
            // Validate CPS on change
            vm.validate_and_update();
        }
    });

    ui.horizontal(|ui| {
        ui.label("Jitter %:");
        ui.add(egui::Slider::new(
            &mut vm.active_profile.jitter_percent,
            0..=30,
        ));
    });

    ui.horizontal(|ui| {
        ui.label("Position:");
        egui::ComboBox::from_id_salt("position_mode")
            .selected_text(if vm.active_profile.position_mode == PositionMode::Cursor {
                "Current cursor"
            } else {
                "Fixed position"
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut vm.active_profile.position_mode,
                    PositionMode::Cursor,
                    "Current cursor",
                );
                ui.selectable_value(
                    &mut vm.active_profile.position_mode,
                    PositionMode::Fixed,
                    "Fixed position",
                );
            });
    });

    if vm.active_profile.position_mode == PositionMode::Fixed {
        ui.horizontal(|ui| {
            ui.label("X:");
            let mut x = vm.active_profile.fixed_position_x.unwrap_or(0);
            ui.add(egui::DragValue::new(&mut x));
            vm.active_profile.fixed_position_x = Some(x);

            ui.label("Y:");
            let mut y = vm.active_profile.fixed_position_y.unwrap_or(0);
            ui.add(egui::DragValue::new(&mut y));
            vm.active_profile.fixed_position_y = Some(y);
        });
    }
}

fn render_scope_mode(ui: &mut Ui, vm: &mut ViewModel) {
    ui.horizontal(|ui| {
        ui.label("Scope:");
        egui::ComboBox::from_id_salt("scope_mode")
            .selected_text(vm.active_profile.scope_mode.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut vm.active_profile.scope_mode,
                    ScopeMode::FocusedOnly,
                    "Focused Only",
                );
                ui.selectable_value(
                    &mut vm.active_profile.scope_mode,
                    ScopeMode::Global,
                    "Global",
                );
            });
    });
}

fn render_conditions(ui: &mut Ui, vm: &mut ViewModel) {
    ui.label(RichText::new("Conditions").color(theme::neon_green()).strong());
    ui.add_space(4.0);

    // ── Stationary-start ──────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.checkbox(&mut vm.active_profile.stationary_start_enabled, "Stationary Start");
        if vm.active_profile.stationary_start_enabled {
            let mut secs = vm.active_profile.stationary_seconds.unwrap_or(TIMER_MIN_SECONDS);
            ui.add(
                egui::DragValue::new(&mut secs)
                    .range(TIMER_MIN_SECONDS as f64..=TIMER_MAX_SECONDS as f64)
                    .suffix("s"),
            );
            vm.active_profile.stationary_seconds = Some(secs);
        }
    });

    if vm.active_profile.stationary_start_enabled {
        // Show waiting indicator when applicable
        let state = vm.engine.current_state();
        if state == RunState::WaitingCondition {
            ui.label(
                RichText::new("⏳ Waiting for stationary…")
                    .color(theme::warning_amber())
                    .small(),
            );
        }
    }

    // ── Stop-after ────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.checkbox(&mut vm.active_profile.stop_after_enabled, "Stop After");
        if vm.active_profile.stop_after_enabled {
            let mut secs = vm.active_profile.stop_after_seconds.unwrap_or(TIMER_MIN_SECONDS);
            ui.add(
                egui::DragValue::new(&mut secs)
                    .range(TIMER_MIN_SECONDS as f64..=TIMER_MAX_SECONDS as f64)
                    .suffix("s"),
            );
            vm.active_profile.stop_after_seconds = Some(secs);
        }
    });

    // Show countdown when running with timed stop
    if vm.active_profile.stop_after_enabled && vm.is_running() {
        if let Ok(session_guard) = vm.engine.session.lock() {
            if let Some(ref session) = *session_guard {
                let elapsed = chrono::Utc::now()
                    .signed_duration_since(session.started_at)
                    .num_seconds()
                    .max(0) as u32;
                let remaining = vm
                    .active_profile
                    .stop_after_seconds
                    .unwrap_or(0)
                    .saturating_sub(elapsed);
                ui.label(
                    RichText::new(format!("⏱ Auto-stop in {remaining}s"))
                        .color(theme::warning_amber())
                        .small(),
                );
            }
        }
    }
}

fn render_controls(ui: &mut Ui, vm: &mut ViewModel) {
    ui.label(RichText::new("Controls").color(theme::neon_green()).strong());
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        let is_running = vm.is_running();

        if !is_running {
            if ui
                .button(RichText::new("▶ START").color(theme::neon_green()).strong())
                .clicked()
            {
                if vm.validate_and_update() {
                    let profile = vm.active_profile.clone();
                    let click_action = profile.click_action;
                    let scope_mode = profile.scope_mode;
                    let position_mode = profile.position_mode;
                    let fixed_x = profile.fixed_position_x;
                    let fixed_y = profile.fixed_position_y;

                    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> =
                        Arc::new(move || {
                            // Check focus if needed
                            if scope_mode == ScopeMode::FocusedOnly
                                && !input_driver::is_target_window_focused()
                            {
                                return false;
                            }

                            // Move to fixed position if needed
                            if position_mode == PositionMode::Fixed {
                                if let (Some(x), Some(y)) = (fixed_x, fixed_y) {
                                    let _ = input_driver::move_to(x, y);
                                }
                            }

                            input_driver::perform_click(click_action).is_ok()
                        });

                    if let Err(e) = vm.engine.start(&profile, click_fn) {
                        vm.validation_error = Some(e.to_string());
                    }
                }
            }
        } else {
            if ui
                .button(RichText::new("⏹ STOP").color(theme::error_red()).strong())
                .clicked()
            {
                if let Err(e) = vm.engine.stop() {
                    vm.validation_error = Some(e.to_string());
                }
            }
        }

        ui.add_space(8.0);

        if ui
            .button(RichText::new("⚡ PANIC STOP").color(theme::warning_amber()).strong())
            .clicked()
        {
            let _ = vm.engine.panic_stop();
        }
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!(
                "Hotkeys: Start={} | Stop={} | Panic={}",
                vm.active_profile.start_hotkey,
                vm.active_profile.stop_hotkey,
                vm.active_profile.panic_hotkey
            ))
            .color(theme::text_secondary())
            .small(),
        );
    });
}

fn render_profiles(ui: &mut Ui, vm: &mut ViewModel) {
    ui.label(RichText::new("Profiles").color(theme::neon_green()).strong());
    ui.add_space(4.0);

    // Profile selector
    let profile_names: Vec<String> = vm.profiles.iter().map(|p| p.name.clone()).collect();
    let current_name = if vm.selected_profile_idx < profile_names.len() {
        profile_names[vm.selected_profile_idx].clone()
    } else {
        "None".to_string()
    };

    ui.horizontal(|ui| {
        ui.label("Active:");
        egui::ComboBox::from_id_salt("profile_selector")
            .selected_text(&current_name)
            .show_ui(ui, |ui| {
                for (idx, name) in profile_names.iter().enumerate() {
                    if ui.selectable_label(vm.selected_profile_idx == idx, name).clicked() {
                        vm.switch_profile(idx);
                    }
                }
            });
    });

    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut vm.new_profile_name);
        if ui.button("Create").clicked() && !vm.new_profile_name.trim().is_empty() {
            let name = vm.new_profile_name.trim().to_string();
            vm.create_profile(name);
            vm.new_profile_name.clear();
        }
    });

    ui.horizontal(|ui| {
        if ui.button("Duplicate").clicked() {
            vm.duplicate_profile();
        }
        if ui.button("Save").clicked() {
            vm.save_active_profile();
        }
        if vm.profiles.len() > 1 && ui.button("Delete").clicked() {
            vm.delete_profile(vm.selected_profile_idx);
        }
    });
}

fn render_safety(ui: &mut Ui, vm: &mut ViewModel) {
    ui.label(RichText::new("Safety Controls").color(theme::neon_green()).strong());
    ui.add_space(4.0);

    // Max session
    ui.horizontal(|ui| {
        ui.checkbox(&mut vm.active_profile.max_session_enabled, "Max Session");
        if vm.active_profile.max_session_enabled {
            let mut secs = vm.active_profile.max_session_seconds.unwrap_or(TIMER_MIN_SECONDS);
            ui.add(
                egui::DragValue::new(&mut secs)
                    .range(TIMER_MIN_SECONDS as f64..=TIMER_MAX_SECONDS as f64)
                    .suffix("s"),
            );
            vm.active_profile.max_session_seconds = Some(secs);
        }
    });

    // Cooldown
    ui.horizontal(|ui| {
        ui.checkbox(&mut vm.active_profile.cooldown_enabled, "Cooldown");
        if vm.active_profile.cooldown_enabled {
            let mut secs = vm.active_profile.cooldown_seconds.unwrap_or(TIMER_MIN_SECONDS);
            ui.add(
                egui::DragValue::new(&mut secs)
                    .range(TIMER_MIN_SECONDS as f64..=TIMER_MAX_SECONDS as f64)
                    .suffix("s"),
            );
            vm.active_profile.cooldown_seconds = Some(secs);
        }
    });

    // Show cooldown status
    if vm.safety.is_cooldown_active() {
        let remaining = vm.safety.cooldown_remaining_secs();
        ui.label(
            RichText::new(format!("⏳ Cooldown: {remaining}s remaining"))
                .color(theme::warning_amber())
                .small(),
        );
    }
}

fn render_theme_selector(ui: &mut Ui, vm: &mut ViewModel) {
    ui.horizontal(|ui| {
        ui.label("Theme:");
        egui::ComboBox::from_id_salt("theme_selector")
            .selected_text(match vm.active_profile.theme_variant {
                ThemeVariant::MadScientistNeon => "Mad Scientist Neon",
                ThemeVariant::HighContrast => "High Contrast",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut vm.active_profile.theme_variant,
                    ThemeVariant::MadScientistNeon,
                    "Mad Scientist Neon",
                );
                ui.selectable_value(
                    &mut vm.active_profile.theme_variant,
                    ThemeVariant::HighContrast,
                    "High Contrast",
                );
            });
    });
}

fn render_event_log(ui: &mut Ui, vm: &ViewModel) {
    ui.label(RichText::new("Event Log").color(theme::neon_green()).strong());
    ui.add_space(4.0);

    let events = vm.recent_events(50);
    if events.is_empty() {
        ui.label(RichText::new("No events yet.").color(theme::text_secondary()).small());
        return;
    }

    ScrollArea::vertical()
        .max_height(120.0)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for event in events.iter().rev() {
                let color = match event.level {
                    crate::domain::events::EventLevel::Debug => theme::text_secondary(),
                    crate::domain::events::EventLevel::Info => theme::text_primary(),
                    crate::domain::events::EventLevel::Warn => theme::warning_amber(),
                    crate::domain::events::EventLevel::Error => theme::error_red(),
                };
                let time = event.timestamp.format("%H:%M:%S");
                ui.label(
                    RichText::new(format!("[{time}] {} — {}", event.event_type, event.message))
                        .color(color)
                        .small(),
                );
            }
        });
}
