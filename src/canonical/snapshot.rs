use std::fmt::{Debug, Formatter};
use crate::canonical::XYWH;

/// Although all fields are public, it is recommended not to modify them directly
#[allow(unused)]
pub struct ScreenInfo {
    pub name: String,
    pub is_primary: bool,
    pub xywh: XYWH,
    pub sf: f32,
    /// The raw pixels of the screen image in RGBA format.
    ///
    /// That is, 4 bytes per pixel. (Length = width * height * 4.)
    pub rgba_pixels: Vec<u8>,
}

#[allow(unused)]
impl ScreenInfo {
    pub fn new(name: impl Into<String>, is_primary: bool, xywh: XYWH, sf: f32, rgba_pixels: Vec<u8>) -> ScreenInfo {
        ScreenInfo { name: name.into(), is_primary, xywh, sf, rgba_pixels }
    }
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

/// Although all fields are public, it is recommended not to modify them directly
#[allow(unused)]
impl AppInfo {
    pub fn new(
        name: impl Into<String>,
        title: impl Into<String>,
        is_minimized: bool,
        xywh: XYWH,
    ) -> AppInfo {
        AppInfo { name: name.into(), title: title.into(), is_minimized, xywh }
    }
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
    xywh: XYWH,
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

    pub fn xywh(&self) -> XYWH {
        self.xywh
    }
}