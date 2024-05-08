use egui::{Frame, Color32, Context, Key, ViewportCommand, Image, Rect, Pos2, Vec2, Ui, Rounding};
use crate::canonical::{Snapshot};
use crate::cropper::config::CropperConfig;

/// get the 4 corners (TL, TR, BR, BL) of the bounding-box from any 2 points (p1, p2)
fn get_4_corners(p1: Pos2, p2: Pos2) -> [Pos2; 4] {
    let xl = p1.x.min(p2.x);
    let xr = p1.x.max(p2.x);
    let yt = p1.y.min(p2.y);
    let yb = p1.y.max(p2.y);

    [
        Pos2::new(xl, yt),
        Pos2::new(xr, yt),
        Pos2::new(xr, yb),
        Pos2::new(xl, yb),
    ]
}

struct Helper {
    /// bottom-right position of the application window
    b_r: Pos2,

    /// (name, position, size, data)
    fragments: Vec<(String, Pos2, Vec2, Vec<u8>)>,

    cover_crop: bool,
    mask_color: Color32,

    crop_from: Option<Pos2>,
    crop_to: Option<Pos2>,
}

impl Helper {
    pub fn new(snapshot: Snapshot, config: CropperConfig) -> Helper {
        // offset to apply from screen coordinates to in-app coordinates
        let (offset_x, offset_y, app_w, app_h) = snapshot.xywh;

        // fragments to draw
        let mut fragments = vec![];
        for screen in &snapshot.screens {
            let (x, y, w, h) = screen.xywh;
            fragments.push((
                screen.name.clone(),
                Pos2::new((x - offset_x) as f32, (y - offset_y) as f32),
                Vec2::new(w as f32, h as f32),
                screen.buffer(),
            ));
        }

        Helper {
            b_r: Pos2::new(app_w as f32, app_h as f32),
            fragments,
            cover_crop: config.selection_mode,
            mask_color: config.get_mask_color(),
            crop_from: None,
            crop_to: None,
        }
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

    pub fn draw_crop(&self, ui: &mut Ui) {
        if self.crop_from.is_none() || self.crop_to.is_none() {
            return;
        }

        let from = self.crop_from.unwrap();
        let to = self.crop_to.unwrap();

        if self.cover_crop {
            ui.painter().rect_filled(
                Rect::from_two_pos(from, to),
                Rounding::ZERO,
                self.mask_color,
            );
        } else {
            // TODO: diagram
            let pa = Pos2::ZERO;
            let pb = Pos2::new(self.b_r.x, 0.0);
            let pc = self.b_r;
            let pd = Pos2::new(0.0, self.b_r.y);
            let [p1, p2, p3, p4] = get_4_corners(from, to);

            let parts = [
                Rect::from_two_pos(pa, p2),
                Rect::from_two_pos(pb, p3),
                Rect::from_two_pos(pc, p4),
                Rect::from_two_pos(pd, p1)
            ];
            for part in parts.into_iter() {
                ui.painter().rect_filled(part, Rounding::ZERO, self.mask_color);
            }
        }
    }

    pub fn start_crop(&mut self, at: Option<Pos2>) {
        // clamp crop area inside dimension
        self.crop_from = at.and_then(|p| Some(p.clamp(Pos2::ZERO, self.b_r)));
    }
    pub fn update_crop(&mut self, at: Option<Pos2>) {
        // clamp crop area inside dimension
        self.crop_to = at.and_then(|p| Some(p.clamp(Pos2::ZERO, self.b_r)));
    }
}

pub struct CropApp {
    helper: Helper,
}

impl CropApp {
    pub fn new(snapshot: Snapshot, config: CropperConfig) -> CropApp {
        let helper = Helper::new(snapshot, config);
        CropApp {
            helper,
        }
    }
}

impl eframe::App for CropApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE))
            .show(ctx, |ui| {
                if ctx.input(|i| i.pointer.primary_pressed()) {
                    let pos = ctx.pointer_interact_pos();
                    self.helper.start_crop(pos);
                }
                if ctx.input(|i| i.pointer.primary_down()) {
                    let pos = ctx.pointer_interact_pos();
                    self.helper.update_crop(pos);
                }

                self.helper.draw_screens(ui);
                self.helper.draw_crop(ui);

                // TODO: draw operation UI
                // ctx.send_viewport_cmd(ViewportCommand::Screenshot);

                // exit conditions
                // 1. press 'Esc' key
                // 2. lose focus -- TODO: https://github.com/emilk/egui/issues/4468
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
    }
}