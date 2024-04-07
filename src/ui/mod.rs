use egui::ViewportBuilder;
use crate::ui::app::MyApp;

mod app;

/// FIXME: test only, use for not expose too much to outside mod
pub fn ui_test() {
    let option = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([100.0, 100.0])
            .with_inner_size([3000.0, 2000.0]),
        ..Default::default()
    };

    eframe::run_native(
        "large window test",
        option,
        Box::new(|ctx| Box::new(MyApp::new())),
    ).unwrap();
}