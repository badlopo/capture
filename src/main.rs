#![windows_subsystem = "windows"]

mod canonical;
mod cropper;
mod snapper;

// fn main() {
//     cropper::Cropper::exec(Default::default()).unwrap();
// }

// 窗口大于屏幕时, resize 会导致窗口被剪切为屏幕大小 (突变)
fn main() {
    use eframe::epaint::Color32;
    use egui::{Frame, Key, ViewportBuilder, ViewportCommand};

    let option = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(true)
            .with_position([10.0, 10.0])
            .with_inner_size([4000.0, 3000.0]),
        ..Default::default()
    };

    eframe::run_simple_native(
        "my-app",
        option,
        |ctx, _frame| {
            egui::CentralPanel::default()
                .frame(Frame::none().fill(Color32::WHITE))
                .show(ctx, |ui| {
                    if ui.button("close").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }

                    if ctx.input(|i| i.key_pressed(Key::Escape)) {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }

                    // 空格键居中
                    if ctx.input(|i| i.key_pressed(Key::Space)) {
                        ctx.send_viewport_cmd(ViewportCommand::center_on_screen(ctx).unwrap());
                        println!("{:?}", ctx.screen_rect());
                    }
                });
        },
    ).unwrap();
}

// https://github.com/emilk/egui/issues/4458
// fn main() {
//     use eframe::epaint::Color32;
//     use egui::{Frame, Key, ViewportBuilder, ViewportCommand};
//
//     let option = eframe::NativeOptions {
//         viewport: ViewportBuilder::default()
//             .with_taskbar(false)
//             .with_decorations(true)
//             .with_always_on_top()  // <= uncomment this line to reproduce the bug
//             .with_position([10.0, 10.0])
//             // replace with the screen size for the actual use
//             .with_inner_size([4000.0, 3000.0]),
//         ..Default::default()
//     };
//
//     eframe::run_simple_native(
//         "my-app",
//         option,
//         |ctx, _frame| {
//             egui::CentralPanel::default()
//                 .frame(Frame::none().fill(Color32::WHITE))
//                 .show(ctx, |ui| {
//                     if ui.button("close").clicked() {
//                         ctx.send_viewport_cmd(ViewportCommand::Close);
//                     }
//
//                     if ctx.input(|i| i.key_pressed(Key::Escape)) {
//                         ctx.send_viewport_cmd(ViewportCommand::Close);
//                     }
//                 });
//         },
//     ).unwrap();
// }

// https://github.com/emilk/egui/issues/4468
// fn main() {
//     use eframe::epaint::Color32;
//     use egui::{Frame, ViewportBuilder, ViewportCommand};
//
//     let option = eframe::NativeOptions {
//         viewport: ViewportBuilder::default()
//             .with_active(true)  // initially focused
//             .with_position([100.0, 100.0])
//             .with_inner_size([800.0, 600.0]),
//         ..Default::default()
//     };
//
//     let mut ready = false;
//
//     eframe::run_simple_native(
//         "my-app",
//         option,
//         move |ctx, _frame| {
//             egui::CentralPanel::default()
//                 .frame(Frame::none().fill(Color32::WHITE))
//                 .show(ctx, |ui| {
//                     if ui.button("Close").clicked() {
//                         ctx.send_viewport_cmd(ViewportCommand::Close);
//                     }
//
//                     // when the focused turns to true, the listener will be ready
//                     if !ready && ctx.input(|i| i.focused) {
//                         ready = true;
//                     }
//                     // when the listener is ready, it will close the window when it loses focus
//                     if ready && ctx.input(|i| !i.focused) {
//                         ctx.send_viewport_cmd(ViewportCommand::Close);
//                     }
//                 });
//         },
//     ).unwrap();
// }