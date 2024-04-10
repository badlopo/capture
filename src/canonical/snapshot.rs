use std::fmt::{Debug, Formatter};
use crate::canonical::XYWH;

#[allow(unused)]
/// although all fields are public, it is recommended not to modify them directly
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

#[allow(unused)]
/// although all fields are public, it is recommended not to modify them directly
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
    pub screens: Vec<ScreenInfo>,
    pub apps: Vec<AppInfo>,
}

#[allow(unused)]
impl Snapshot {
    pub fn new(screens: Vec<ScreenInfo>, apps: Vec<AppInfo>) -> Snapshot {
        Snapshot { screens, apps }
    }
}