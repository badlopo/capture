use eframe::egui::{Frame};
use egui::{Color32, Context, Key, ViewportCommand};

pub struct MyApp {}

impl MyApp {
    pub fn new() -> MyApp {
        MyApp {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::DARK_GRAY))
            .show(ctx, |ui| {
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
    }
}