// #![windows_subsystem = "windows"]

mod canonical;
mod cropper;
mod snapper;

fn main() {
    cropper::Cropper::snap_and_crop().unwrap();
}

#[cfg(test)]
mod misc_test {
    use std::io::{Cursor, Read};
    use image;
    use image::{DynamicImage, RgbaImage};

    #[test]
    fn image_read_test() {
        // let img = image::load_from_memory(include_bytes!("../test/black.png")).unwrap();
        // println!("Image: {:?}", img);

        // let img = image::open("./test/black.png").unwrap().into_rgba8();
        // println!("Image: {:?}", img.into_raw());

        // let buffer = vec![];
        // let mut buffer = Cursor::new(buffer);
        // img.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
        // println!("Image: {:?}", buffer);
    }
}