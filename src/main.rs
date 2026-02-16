use std::sync::Arc;

use nos_autoclicker::app::ui::AutoclickerApp;
use nos_autoclicker::engine::click_engine::ClickEngine;
use nos_autoclicker::infra;

fn main() -> eframe::Result<()> {
    // Initialize telemetry
    infra::telemetry::init_telemetry();
    tracing::info!("nos-autoclicker starting");

    // Create shared engine
    let engine = Arc::new(ClickEngine::new());

    // Launch eframe native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_min_inner_size([380.0, 400.0])
            .with_title("NOS Autoclicker"),
        ..Default::default()
    };

    eframe::run_native(
        "NOS Autoclicker",
        options,
        Box::new(move |_cc| Ok(Box::new(AutoclickerApp::new(engine)))),
    )
}
