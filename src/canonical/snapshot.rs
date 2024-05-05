use std::fmt::{Debug, Formatter};
use std::io::Cursor;
use image::{ImageFormat, RgbaImage};
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

impl ScreenInfo {
    /// Get the buffer of the screen image in PNG format
    ///
    /// Note: this is costly, use it wisely
    pub fn buffer(&self) -> Vec<u8> {
        let mut buffer = Cursor::new(vec![]);
        self.rgba_image.write_to(&mut buffer, ImageFormat::Png).unwrap();
        buffer.into_inner()
    }


    /// Get the raw pixels of the screen image in RGBA format.
    ///
    /// That is, 4 bytes per pixel. (Length = width * height * 4.)
    pub fn pixels(&self) -> &Vec<u8> {
        self.rgba_image.as_raw()
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