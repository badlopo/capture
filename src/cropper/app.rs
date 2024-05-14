use egui::{Frame, Color32, Context, Key, ViewportCommand, Image, Rect, Pos2, Vec2, Ui, Rounding, CursorIcon};
use crate::canonical::{Snapshot};
use crate::cropper::config::CropperConfig;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PositionRelation {
    Inside,
    Outside,
    /// we use something like one-hot encoding to represent the position relation.
    ///
    /// THAT IS:
    ///
    /// | Position     | Code |
    /// |--------------|------|
    /// | top-left     | 5    |
    /// | top-right    | 3    |
    /// | bottom-right | 8    |
    /// | bottom-left  | 10   |
    /// | top          | 1    |
    /// | right        | 2    |
    /// | bottom       | 6    |
    /// | left         | 4    |
    Edge(u8),
}

impl From<PositionRelation> for CursorIcon {
    fn from(value: PositionRelation) -> Self {
        match value {
            PositionRelation::Inside => CursorIcon::Move,
            PositionRelation::Outside => CursorIcon::Default,
            PositionRelation::Edge(code) => match code {
                5 => CursorIcon::ResizeNorthWest,
                3 => CursorIcon::ResizeNorthEast,
                8 => CursorIcon::ResizeSouthEast,
                10 => CursorIcon::ResizeSouthWest,
                1 => CursorIcon::ResizeNorth,
                2 => CursorIcon::ResizeEast,
                6 => CursorIcon::ResizeSouth,
                4 => CursorIcon::ResizeWest,
                _ => unreachable!("this code should not be reached")
            }
        }
    }
}

fn get_position_relation(bounding: Rect, point: Pos2) -> PositionRelation {
    let Pos2 { x: px, y: py } = point;
    let Rect { min: Pos2 { x: bxl, y: byt }, max: Pos2 { x: bxr, y: byb } } = bounding;

    let mut code = 0u8;
    if px == bxl {
        code += 4;
    } else if px == bxr {
        code += 2;
    }
    if py == byt {
        code += 1;
    } else if py == byb {
        code += 6;
    }

    if code == 0 {
        if px > bxl && px < bxr && py > byt && py < byb {
            PositionRelation::Inside
        } else {
            PositionRelation::Outside
        }
    } else {
        PositionRelation::Edge(code)
    }
}

fn apply_resize(rect: Rect, modify: Vec2, code: u8) -> Rect {
    let Rect { min, max } = rect;

    match code {
        5 => Rect::from_two_pos(min + modify, max),
        3 => Rect::from_two_pos(Pos2::new(min.x, min.y + modify.y), Pos2::new(max.x + modify.x, max.y)),
        8 => Rect::from_two_pos(min, max + modify),
        10 => Rect::from_two_pos(Pos2::new(min.x + modify.x, min.y), Pos2::new(max.x, max.y + modify.y)),
        1 => Rect::from_two_pos(Pos2::new(min.x, min.y + modify.y), max),
        2 => Rect::from_two_pos(min, Pos2::new(max.x + modify.x, max.y)),
        6 => Rect::from_two_pos(min, Pos2::new(max.x, max.y + modify.y)),
        4 => Rect::from_two_pos(Pos2::new(min.x + modify.x, min.y), max),
        _ => unreachable!("this code should not be reached")
    }
}

#[derive(Eq, PartialEq, Debug)]
enum AppState {
    // primary button is up, no crop area
    Idle,
    /// primary button is down, crop area is updating.
    /// - (start point)
    Cropping(Pos2),
    /// primary button is up, crop area is fixed
    Cropped,
    /// primary button is down/released, but we ignore the event and do nothing
    /// this case happens when the primary button is pressed outside the crop area
    Ignored,
    /// primary button is down, crop area is moving
    /// - (crop area, start point)
    Moving(Rect, Pos2),
    /// primary button is down, crop area is resizing
    /// - (crop area, start point, code)
    Resizing(Rect, Pos2, u8),
}

struct Helper {
    /// bottom-right position of the application window
    max_point: Pos2,
    /// (name, position, size, data)
    fragments: Vec<(String, Pos2, Vec2, Vec<u8>)>,
    mask_color: Color32,

    /// state of the application
    app_state: AppState,

    /// rect of the crop area
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
            app_state: AppState::Idle,
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

            // TODO: resize handles *8

            // TODO: size indicator (width x height)
        }
    }

    pub fn update_cursor(&self, ctx: &Context) {
        match self.app_state {
            AppState::Cropped => {
                // if there is a crop area, we need to update the
                // cursor icon depending on the position relation
                if let Some(p) = ctx.pointer_interact_pos() {
                    ctx.output_mut(|o| o.cursor_icon = get_position_relation(self.crop_area.unwrap(), p).into());
                }
            }
            AppState::Moving(_, _) => {
                ctx.output_mut(|o| o.cursor_icon = CursorIcon::Move);
            }
            AppState::Resizing(_, _, code) => {
                ctx.output_mut(|o| o.cursor_icon = PositionRelation::Edge(code).into());
            }
            _ => {}
        }
    }

    pub fn handle_primary_pressed(&mut self, at: Option<Pos2>) {
        if let Some(p) = at {
            self.app_state = match self.app_state {
                AppState::Idle => AppState::Cropping(p),
                AppState::Cropped => {
                    // we need to check the position relation of the
                    // cursor to the crop area to determine the next state
                    let crop_area = self.crop_area.unwrap();
                    match get_position_relation(crop_area, p) {
                        PositionRelation::Inside => AppState::Moving(crop_area, p),
                        PositionRelation::Outside => AppState::Ignored,
                        PositionRelation::Edge(code) => AppState::Resizing(crop_area, p, code)
                    }
                }
                ref s @ _ => unreachable!("point pressed event should not happen in this app_state (state: {:?})", s),
            };
        }
    }

    pub fn handle_primary_down(&mut self, at: Option<Pos2>) {
        if let Some(p) = at {
            let constrained_p = p.clamp(Pos2::ZERO, self.max_point);
            match self.app_state {
                AppState::Cropping(p_start) => {
                    self.crop_area = Some(Rect::from_two_pos(p_start, constrained_p));
                }
                AppState::Moving(crop_area, p_start) => {
                    // translate the crop area by the difference between the current point and the start point
                    self.crop_area = Some(crop_area.translate(p - p_start));
                }
                AppState::Resizing(crop_area, p_start, code) => {
                    // resize the crop area by the difference between the current point and the start point
                    self.crop_area = Some(apply_resize(crop_area, p - p_start, code));
                }
                AppState::Ignored => {
                    // when the primary button is pressed outside the crop area.
                    // we do nothing in this case.
                }
                ref s @ _ => unreachable!("point down event should not happen in this app_state (state: {:?})", s)
            }
        }
    }

    pub fn handle_primary_released(&mut self) {
        self.app_state = match self.app_state {
            AppState::Cropping(_) | AppState::Moving(_, _) | AppState::Resizing(_, _, _) => AppState::Cropped,
            AppState::Ignored => AppState::Cropped,
            ref s @ _ => unreachable!("point released event should not happen in this app_state (state: {:?})", s),
        }
    }
}

pub struct CropApp {
    // due to https://github.com/emilk/egui/issues/4468, we have to use this flag to check if the app is ready
    ready: bool,
    helper: Helper,
}

impl CropApp {
    pub fn new(snapshot: Snapshot, config: CropperConfig) -> CropApp {
        let helper = Helper::new(snapshot, config);
        CropApp {
            ready: false,
            helper,
        }
    }
}

impl eframe::App for CropApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE))
            .show(ctx, |ui| {
                // update cursor icon
                self.helper.update_cursor(ctx);

                // handle primary button events
                if ctx.input(|i| i.pointer.primary_pressed()) {
                    let pos = ctx.pointer_interact_pos();
                    self.helper.handle_primary_pressed(pos);
                } else if ctx.input(|i| i.pointer.primary_down()) {
                    let pos = ctx.pointer_interact_pos();
                    self.helper.handle_primary_down(pos);
                } else if ctx.input(|i| i.pointer.primary_released()) {
                    self.helper.handle_primary_released();
                }

                // draw ui
                self.helper.draw_screens(ui);
                self.helper.draw_crop(ui);
                // TODO: draw operation UI

                // exit conditions
                // - TODO: press 'Enter' key
                // - press 'Esc' key
                // - lose focus
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
                if self.ready {
                    if !ctx.input(|i| i.focused) {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                } else {
                    if ctx.input(|i| i.focused) {
                        self.ready = true;
                    }
                }
            });
    }
}