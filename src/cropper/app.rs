use eframe::egui::{Frame};
use egui::{Color32, Context, Key, ViewportCommand};
use crate::canonical::{ScreenInfo, Snapshot, XYWH};

struct CropperHelper {
    /// The snapshot to be cropped.
    snapshot: Snapshot,
    /// The bounding box of the cropping area.
    bounding: XYWH,
}

impl CropperHelper {
    pub fn new(snapshot: Snapshot) -> CropperHelper {
        let bounding = CropperHelper::bounding(&snapshot.screens);
        CropperHelper { snapshot, bounding }
    }

    /// Calculate the bounding box of the screens.
    fn bounding(screens: &Vec<ScreenInfo>) -> XYWH {
        let mut xx = (0, 0);
        let mut yy = (0, 0);

        for screen in screens {
            let (sx, sy, sw, sh) = screen.xywh;
            xx.0 = xx.0.min(sx);
            xx.1 = xx.1.max(sx + sw as i32);
            yy.0 = yy.0.min(sy);
            yy.1 = yy.1.max(sy + sh as i32);
        }

        (xx.0, yy.0, (xx.1 - xx.0) as u32, (yy.1 - yy.0) as u32)
    }

    /// Detect whether the point is in any app window. If so, return the window's bounding box.
    pub fn auto_bound(&self, point: (i32, i32)) -> Option<XYWH> {
        todo!("auto_bound")
    }
}

pub struct CropApp {
    helper: CropperHelper,
}

impl CropApp {
    pub fn new(snapshot: Snapshot) -> CropApp {
        CropApp {
            helper: CropperHelper::new(snapshot),
        }
    }
}

impl eframe::App for CropApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::DARK_GRAY))
            .show(ctx, |ui| {
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
    }
}