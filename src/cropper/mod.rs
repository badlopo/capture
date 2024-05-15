mod app;
mod config;

use std::cell::RefCell;
use std::rc::Rc;
use app::{CropApp};
pub use config::CropperConfig;
use egui::ViewportBuilder;
use image::RgbaImage;
use crate::snapper::Snapper;

pub struct Cropper;

impl Cropper {
    /// Take a snapshot and crop it with interactive UI
    pub fn exec(cropper_config: CropperConfig) -> Result<Option<RgbaImage>, String> {
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

        let result: Rc<RefCell<Option<RgbaImage>>> = Rc::new(RefCell::new(None));
        eframe::run_native(
            "Capture",
            option,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Box::new(CropApp::new(snapshot, cropper_config, result.clone()))
            }),
        ).unwrap();

        // we make sure that 'result' only has a reference count of 1 at this point,
        // so use 'Rc::try_unwrap' to take ownership and 'into_inner' to get the value inside
        // Ok(Rc::try_unwrap(result).unwrap().into_inner())

        // use 'Rc::unwrap_or_clone' instead of 'Rc::try_unwrap' to ensure success
        Ok(Rc::unwrap_or_clone(result).unwrap().into_inner())
    }
}