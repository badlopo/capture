use egui::Color32;

/// config for cropper
pub struct CropperConfig {
    /// mask color, in RGBA format. Default to [0, 0, 0, 128]
    mask_color: [u8; 4],
}

impl Default for CropperConfig {
    fn default() -> CropperConfig {
        CropperConfig {
            mask_color: [0, 0, 0, 128],
        }
    }
}

impl CropperConfig {
    pub fn mask_color(&self) -> Color32 {
        Color32::from_rgba_premultiplied(
            self.mask_color[0],
            self.mask_color[1],
            self.mask_color[2],
            self.mask_color[3],
        )
    }
}