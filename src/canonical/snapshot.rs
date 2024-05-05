use std::fmt::{Debug, Formatter};
use image::RgbaImage;
use crate::canonical::XYWH;

/// Although all fields are public, it is recommended not to modify them directly
#[allow(unused)]
pub struct ScreenInfo {
    pub name: String,
    pub is_primary: bool,
    pub xywh: XYWH,
    pub sf: f32,
    pub rgba_image: RgbaImage,
}

impl Debug for ScreenInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: primary = {}, position = ({},{}), size = {}x{}, sf = {}",
            self.name,
            self.is_primary,
            self.xywh.0,
            self.xywh.1,
            self.xywh.2,
            self.xywh.3,
            self.sf
        )
    }
}

#[allow(unused)]
pub struct AppInfo {
    pub name: String,
    pub title: String,
    pub is_minimized: bool,
    pub xywh: XYWH,
}

impl Debug for AppInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: title = \"{}\", minimize = {}, position = ({},{}), size = {}x{}",
            self.name,
            self.title,
            self.is_minimized,
            self.xywh.0,
            self.xywh.1,
            self.xywh.2,
            self.xywh.3
        )
    }
}

/// A snapshot of the current state of the monitor(s) and the app(s).
#[allow(unused)]
#[derive(Debug)]
pub struct Snapshot {
    pub xywh: XYWH,
    pub screens: Vec<ScreenInfo>,
    pub apps: Vec<AppInfo>,
}

#[allow(unused)]
impl Snapshot {
    pub fn new(screens: Vec<ScreenInfo>, apps: Vec<AppInfo>) -> Snapshot {
        if screens.is_empty() {
            panic!("No screen found");
        }

        let (x1, y1, x2, y2) = screens.iter().fold(
            (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
            |(x1, y1, x2, y2), screen| {
                let (x, y, w, h) = screen.xywh;
                (
                    x1.min(x),
                    y1.min(y),
                    x2.max(x + w as i32),
                    y2.max(y + h as i32),
                )
            },
        );
        let xywh: XYWH = (x1, y1, (x2 - x1) as u32, (y2 - y1) as u32);

        Snapshot { xywh, screens, apps }
    }
}