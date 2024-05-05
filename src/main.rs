#![windows_subsystem = "windows"]

mod canonical;
mod cropper;
mod snapper;

fn main() {
    cropper::Cropper::exec(Default::default()).unwrap();
}

// https://github.com/emilk/egui/issues/4451
// fn main() {
//     use eframe::epaint::Color32;
//     use egui::{Frame, Key, ViewportBuilder, ViewportCommand};
//
//     let option = eframe::NativeOptions {
//         viewport: ViewportBuilder::default()
//             .with_taskbar(false)
//             .with_decorations(false)
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
//                     if ui.button("Close").clicked() {
//                         ctx.send_viewport_cmd(ViewportCommand::Close);
//                     }
//                     if ui.button("Maximize").clicked() {
//                         ui.ctx()
//                             .send_viewport_cmd(ViewportCommand::Maximized(true));
//                     }
//                     if ui.button("UnMaximize").clicked() {
//                         ui.ctx()
//                             .send_viewport_cmd(ViewportCommand::Maximized(false));
//                     }
//                     if ui.button("set innersize to 500x500").clicked() {
//                         ui.ctx().send_viewport_cmd(ViewportCommand::InnerSize(
//                             egui::Vec2::new(500.0, 500.0)
//                         ));
//                     }
//
//                     if ctx.input(|i| i.key_pressed(Key::Escape)) {
//                         ctx.send_viewport_cmd(ViewportCommand::Close);
//                     }
//                 });
//         },
//     ).unwrap();
// }