use egui::{Frame, Color32, Context, Key, ViewportCommand, Image, Rect, Pos2, Vec2, Ui};
use crate::canonical::{Snapshot};
use crate::cropper::config::CropperConfig;

struct Helper {
    fragments: Vec<(String, Pos2, Vec2, Vec<u8>)>,
}

impl Helper {
    pub fn new(snapshot: Snapshot) -> Helper {
        let mut fragments = vec![];
        for screen in &snapshot.screens {
            let (x, y, w, h) = screen.xywh;
            // TODO: apply transformation from screen coordinates to window coordinates
            fragments.push((
                screen.name.clone(),
                Pos2::new(x as f32, y as f32),
                Vec2::new(w as f32, h as f32),
                screen.buffer(),
            ));
        }

        Helper { fragments }
    }

    pub fn draw_screens(&self, ui: &mut Ui) {
        let fragments = self.fragments.clone();
        for fragment in fragments {
            let (name, pos, size, data) = fragment;
            ui.put(
                Rect::from_min_size(pos, size),
                Image::from_bytes(name, data),
            );
        }
    }
}

pub struct CropApp {
    helper: Helper,
}

impl CropApp {
    pub fn new(snapshot: Snapshot, _config: CropperConfig) -> CropApp {
        let helper = Helper::new(snapshot);
        CropApp { helper }
    }
}

impl eframe::App for CropApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE))
            .show(ctx, |ui| {
                // draw all screens
                self.helper.draw_screens(ui);

                // process input events
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
    }
}