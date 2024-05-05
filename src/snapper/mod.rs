use xcap::{Monitor, Window, XCapError};
use crate::canonical::{AppInfo, ScreenInfo, Snapshot};

pub struct Snapper;

impl Snapper {
    /// Take a snapshot of the screens.
    fn _screens() -> Result<Vec<ScreenInfo>, XCapError> {
        // monitor info
        let monitors = Monitor::all()?;

        // info & screenshot of each monitor
        let mut screens = vec![];
        for monitor in monitors {
            screens.push(ScreenInfo {
                name: monitor.name().into(),
                is_primary: monitor.is_primary(),
                xywh: (monitor.x(), monitor.y(), monitor.width(), monitor.height()),
                sf: monitor.scale_factor(),
                rgba_image: monitor.capture_image()?,
            });
        }

        Ok(screens)
    }

    /// Take a snapshot of the apps.
    fn _apps() -> Result<Vec<AppInfo>, XCapError> {
        // window info
        let windows = Window::all()?;

        // info & screenshot of each window
        let mut apps = vec![];
        for window in windows {
            apps.push(AppInfo {
                name: window.app_name().into(),
                title: window.title().into(),
                is_minimized: window.is_minimized(),
                xywh: (window.x(), window.y(), window.width(), window.height()),
            });
        }

        Ok(apps)
    }

    /// Take a snapshot of the screens and apps(if with_app_info is true).
    pub fn take_snapshot(with_app_info: bool) -> Result<Snapshot, String> {
        match Snapper::_screens() {
            Ok(screens) => if with_app_info {
                match Snapper::_apps() {
                    Ok(apps) => Ok(Snapshot::new(screens, apps)),
                    Err(err2) => Err(format!("{:?}", err2)),
                }
            } else {
                Ok(Snapshot::new(screens, vec![]))
            }
            Err(err1) => Err(format!("{:?}", err1))
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn take_snapshot_test() {
        let now = std::time::Instant::now();

        match Snapper::take_snapshot(false) {
            Ok(snapshot) => {
                let xywh = snapshot.xywh;

                println!("Snapshot: {:#?}", snapshot);
                println!("xywh: {:?}", xywh);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        println!("Elapsed: {:?}", now.elapsed());
    }
}