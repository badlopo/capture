use std::fmt::{Debug, Display, Formatter};
use image::RgbaImage;
use crate::canonical::XYWH;

pub struct ScreenInfo {
    name: String,
    is_primary: bool,
    xywh: XYWH,
    img: RgbaImage,
}

impl ScreenInfo {
    pub fn new(name: impl Into<String>, is_primary: bool, xywh: XYWH, img: RgbaImage) -> ScreenInfo {
        ScreenInfo { name: name.into(), is_primary, xywh, img }
    }

    pub fn img(&self) -> &RgbaImage { &self.img }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        self.img.save(path).map_err(|err| format!("Error saving image: {:?}", err))
    }
}

impl Debug for ScreenInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: primary = {}, position = ({},{}), size = {}x{}",
            self.name,
            self.is_primary,
            self.xywh.0,
            self.xywh.1,
            self.xywh.2,
            self.xywh.3
        )
    }
}

pub struct AppInfo {
    name: String,
    title: String,
    is_minimized: bool,
    xywh: XYWH,
    img: RgbaImage,
}

impl AppInfo {
    pub fn new(
        name: impl Into<String>,
        title: impl Into<String>,
        is_minimized: bool,
        xywh: XYWH,
        img: RgbaImage,
    ) -> AppInfo {
        AppInfo { name: name.into(), title: title.into(), is_minimized, xywh, img }
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
#[derive(Debug)]
pub struct Snapshot {
    pub screens: Vec<ScreenInfo>,
    pub apps: Vec<AppInfo>,
}

impl Snapshot {
    pub fn new(screens: Vec<ScreenInfo>, apps: Vec<AppInfo>) -> Snapshot {
        Snapshot { screens, apps }
    }
}