mod snapshot;

pub use snapshot::{AppInfo, ScreenInfo, Snapshot};

/// x, y (top-left corner) and width, height
pub type XYWH = (i32, i32, u32, u32);

