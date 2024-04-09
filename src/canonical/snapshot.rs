use std::fmt::{Debug, Formatter};
use crate::canonical::XYWH;

#[allow(unused)]
pub struct ScreenInfo {
    name: String,
    is_primary: bool,
    xywh: XYWH,
    sf: f32,
    /// The raw pixels of the screen image in RGBA format.
    ///
    /// That is, 4 bytes per pixel. (Length = width * height * 4.)
    rgba_pixels: Vec<u8>,
}

#[allow(unused)]
impl ScreenInfo {
    pub fn new(name: impl Into<String>, is_primary: bool, xywh: XYWH, sf: f32, rgba_pixels: Vec<u8>) -> ScreenInfo {
        ScreenInfo { name: name.into(), is_primary, xywh, sf, rgba_pixels }
    }

    /// The raw pixels of the screen image in RGBA format.
    pub fn pixels(&self) -> &Vec<u8> { &self.rgba_pixels }
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
    name: String,
    title: String,
    is_minimized: bool,
    xywh: XYWH,
}

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
    pub screens: Vec<ScreenInfo>,
    pub apps: Vec<AppInfo>,
}

#[allow(unused)]
impl Snapshot {
    pub fn new(screens: Vec<ScreenInfo>, apps: Vec<AppInfo>) -> Snapshot {
        Snapshot { screens, apps }
    }
}