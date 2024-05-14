mod app;
mod config;

use app::{CropApp};
pub use config::CropperConfig;
use egui::ViewportBuilder;
use crate::snapper::Snapper;

pub struct Cropper;

impl Cropper {
    /// Take a snapshot and crop it with interactive UI
    pub fn exec(cropper_config: CropperConfig) -> Result<(), String> {
        let snapshot = Snapper::take_snapshot(cropper_config.auto_bounding)?;

        let (x, y, w, h) = snapshot.xywh;
        let option = eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_taskbar(false)
                .with_decorations(false)
                .with_always_on_top()
                .with_position([x as f32, y as f32])
                .with_inner_size([w as f32, h as f32]),
            ..Default::default()
        };

        eframe::run_native(
            "Capture",
            option,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Box::new(CropApp::new(snapshot, cropper_config))
            }),
        ).unwrap();

        Ok(())
    }
}