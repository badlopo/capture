// #![windows_subsystem = "windows"]

mod canonical;
mod cropper;
mod snapper;

fn main() {
    cropper::Cropper::ui_test();
}