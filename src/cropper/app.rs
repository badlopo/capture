use egui::{Frame, Color32, Context, Key, ViewportCommand, Image, Rect, Pos2, Vec2};
use crate::canonical::{Snapshot, XYWH};
use crate::cropper::config::CropperConfig;

struct Helper {
    xywh: XYWH,
}

impl Helper {
    pub fn new(xywh: XYWH) -> Helper {
        Helper { xywh }
    }

    // /// Detect whether the point is in any app window. If so, return the window's bounding box.
    // pub fn auto_bound(&self, point: (i32, i32)) -> Option<XYWH> {
    //     todo!("auto_bound")
    // }
}

pub struct CropApp {
    snapshot: Snapshot,
    helper: Helper,

    mask_color: Color32,
}

impl CropApp {
    pub fn simple(snapshot: Snapshot) -> CropApp {
        CropApp::with_config(snapshot, CropperConfig::default())
    }

    // TODO: expose 'CropperConfig' to the user
    pub fn with_config(snapshot: Snapshot, config: CropperConfig) -> CropApp {
        let helper = Helper::new(snapshot.xywh);
        CropApp {
            snapshot,
            helper,
            mask_color: config.mask_color(),
        }
    }
}

impl eframe::App for CropApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE))
            .show(ctx, |ui| {
                // TODO: put all screens into the panel
                ui.put(
                    Rect::from_min_size(
                        Pos2::new(0.0, 0.0),
                        Vec2::new(1920.0, 1080.0),
                    ), Image::from_bytes(
                        "bytes://screen1.png",
                        include_bytes!("../../test/img.png"),
                    ),
                );

                // process input events
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
    }
}