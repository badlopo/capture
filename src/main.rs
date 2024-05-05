// #![windows_subsystem = "windows"]

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
//             // .with_always_on_top()  // <= uncomment this line to reproduce the bug
//             .with_position([10.0, 10.0])
//             // replace with the screen size for the actual use
//             .with_inner_size([1920.0, 1080.0]),
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