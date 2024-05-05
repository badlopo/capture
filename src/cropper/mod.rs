mod app;
mod config;

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
        let (x, y, w, h) = snapshot.xywh;

        let option = eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_taskbar(false)
                .with_decorations(false)
                .with_always_on_top()
                .with_position([x as f32, y as f32])
                // FIXME: bug with 'always_on_top' when 'with_inner_size' equals to screen size
                .with_inner_size([w as f32 - 1.0, h as f32]),
            ..Default::default()
        };

        eframe::run_native(
            "Capture",
            option,
            Box::new(|cc| {
                // install image loaders for egui
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Box::new(CropApp::simple(snapshot))
            }),
        ).unwrap();

        Ok(())
    }
}