mod app;

use egui::ViewportBuilder;
use crate::cropper::app::{CropApp};
use crate::snapper::Snapper;

pub struct Cropper;

impl Cropper {
    /// Take a snapshot and crop it with interactive UI
    ///
    /// # Arguments
    ///
    /// * `auto_bounding` - Whether to automatically bounding the application window when the mouse passes over it.
    pub fn exec(auto_bounding: bool) -> Result<(), String> {
        let snapshot = Snapper::take_snapshot(auto_bounding)?;

        let option = eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_position([5.0, 5.0])
                .with_inner_size([4470.0, 2510.0])
                // .with_clamp_size_to_monitor_size(false)
                .with_decorations(false)
                .with_always_on_top(),
            ..Default::default()
        };

        eframe::run_native(
            "a large window app",
            option,
            Box::new(|ctx| Box::new(CropApp::new(snapshot))),
        ).unwrap();

        Ok(())
    }
}