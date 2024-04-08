use egui::ViewportBuilder;
use crate::cropper::app::MyApp;

mod app;

/// FIXME: test only, use for not expose too much to outside mod
pub fn ui_test() {
    let option = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([5.0, 5.0])
            .with_inner_size([4470.0, 2510.0])
            .with_clamp_size_to_monitor_size(false)
            .with_decorations(false)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "a large window app",
        option,
        Box::new(|ctx| Box::new(MyApp::new())),
    ).unwrap();
}