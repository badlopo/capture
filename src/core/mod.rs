use xcap::{Monitor, Window, XCapError};
use crate::canonical::{AppInfo, ScreenInfo, Snapshot};

pub struct Core {}

impl Core {
    fn _take_snapshot() -> Result<Snapshot, XCapError> {
        // monitor info
        let monitors = Monitor::all()?;

        // info & screenshot of each monitor
        let mut screens = vec![];
        for monitor in monitors {
            screens.push(ScreenInfo::new(
                monitor.name(),
                monitor.is_primary(),
                (monitor.x(), monitor.y(), monitor.width(), monitor.height()),
                monitor.capture_image()?,
            ));
        }

        // window info
        let windows = Window::all()?;

        // info & screenshot of each window
        let mut apps = vec![];
        for window in windows {
            apps.push(AppInfo::new(
                window.app_name(),
                window.title(),
                window.is_minimized(),
                (window.x(), window.y(), window.width(), window.height()),
                window.capture_image()?,
            ));
        }

        Ok(Snapshot::new(screens, apps))
    }

    pub fn take_snapshot() -> Result<Snapshot, String> {
        Core::_take_snapshot().map_err(|e| format!("{:?}", e))
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn take_snapshot_test() {
        let now = std::time::Instant::now();

        match Core::take_snapshot() {
            Ok(snapshot) => {
                println!("Snapshot: {:#?}", snapshot);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        println!("Elapsed: {:?}", now.elapsed());
    }

    #[test]
    fn misc() {
        let now = std::time::Instant::now();

        let mut screens = vec![];
        match Monitor::all() {
            Ok(monitors) => {
                for monitor in monitors {
                    screens.push(ScreenInfo::new(
                        monitor.name(),
                        monitor.is_primary(),
                        (monitor.x(), monitor.y(), monitor.width(), monitor.height()),
                        monitor.capture_image().unwrap(),
                    ));
                }
            }
            Err(err) => {
                println!("Fail! {}", err);
            }
        }
        println!("Screens: {:#?}", screens);

        for (idx, screen) in screens.iter().enumerate() {
            screen.save(format!("screen_{}.png", idx)).unwrap();
            println!("{}, elapsed: {:?}", idx, now.elapsed());
        }
        // match Window::all() {
        //     Ok(windows) => {
        //         println!("{}", windows.len());
        //     }
        //     Err(err) => {
        //         println!("Fail! {}", err);
        //     }
        // }

        println!("Elapsed: {:?}", now.elapsed());
    }
}