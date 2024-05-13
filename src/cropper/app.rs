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

enum State {
    // primary button is up, no crop area
    Idle,
    /// primary button is down, crop area is updating
    Cropping(Pos2),
    /// primary button is up, crop area is fixed
    Cropped,
    /// primary button is down, crop area is moving
    Moving(Pos2),
    // primary button is down, crop area is resizing
    // Resizing(Handle),
}

struct Helper {
    /// bottom-right position of the application window
    max_point: Pos2,
    /// (name, position, size, data)
    fragments: Vec<(String, Pos2, Vec2, Vec<u8>)>,
    mask_color: Color32,

    state: State,
    crop_area: Option<Rect>,
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
            max_point: Pos2::new(app_w as f32, app_h as f32),
            fragments,
            mask_color: config.get_mask_color(),
            state: State::Idle,
            crop_area: None,
        }
    }

    pub fn draw_screens(&self, ui: &mut Ui) {
        let fragments = self.fragments.clone();
        for fragment in fragments {
            let (name, pos, size, data) = fragment;
            ui.put(Rect::from_min_size(pos, size), Image::from_bytes(name, data));
        }
    }

    pub fn draw_crop(&self, ui: &mut Ui) {
        if let Some(rect) = self.crop_area {
            let tl_o = Pos2::ZERO;
            let tr_o = Pos2::new(self.max_point.x, 0.0);
            let br_o = self.max_point;
            let bl_o = Pos2::new(0.0, self.max_point.y);

            let parts = [
                Rect::from_two_pos(tl_o, rect.right_top()),
                Rect::from_two_pos(tr_o, rect.right_bottom()),
                Rect::from_two_pos(br_o, rect.left_bottom()),
                Rect::from_two_pos(bl_o, rect.left_top()),
            ];
            for part in parts.into_iter() {
                ui.painter().rect_filled(part, Rounding::ZERO, self.mask_color);
            }
        }
    }

    pub fn handle_primary_pressed(&mut self, at: Option<Pos2>) {
        if let Some(p) = at {
            self.state = match self.state {
                State::Idle => State::Cropping(p),
                State::Cropped => {
                    // TODO: 根据点和rect的位置状态转移
                    // 内部 => State::Moving
                    // 边缘 => State::Resizing
                    // 外部 => State::Cropped (无变化)
                    State::Moving(p)
                }
                _ => unreachable!("point pressed event should not happen in this state"),
            };
        }
    }

    pub fn handle_primary_down(&mut self, at: Option<Pos2>) {
        if let Some(p) = at {
            let constrained_p = p.clamp(Pos2::ZERO, self.max_point);
            match self.state {
                State::Cropping(p_start) => {
                    self.crop_area = Some(Rect::from_two_pos(p_start, constrained_p));
                }
                State::Moving(p_prev) => {
                    // translate the crop area by the delta of the current and previous points
                    self.crop_area = self.crop_area.map(|rect| rect.translate(p - p_prev));
                    // update the 'previous point'
                    self.state = State::Moving(p);
                }
                _ => unreachable!("point down event should not happen in this state")
            }
        }
    }

    pub fn handle_primary_released(&mut self) {
        self.state = match self.state {
            State::Cropping(_) | State::Moving(_) => State::Cropped,
            _ => unreachable!("point released event should not happen in this state"),
        }
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
                    self.helper.handle_primary_pressed(pos);
                }
                if ctx.input(|i| i.pointer.primary_down()) {
                    let pos = ctx.pointer_interact_pos();
                    self.helper.handle_primary_down(pos);
                }
                if ctx.input(|i| i.pointer.primary_released()) {
                    self.helper.handle_primary_released();
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