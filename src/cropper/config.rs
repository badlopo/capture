/// config for cropper
pub struct CropperConfig {
    /// whether to automatically bounding the application window when the mouse passes over it
    pub auto_bounding: bool,

    /// whether to highlight the crop area
    /// - true (highlight mode): the crop area is displayed as is, the rest of the screen is covered with mask color
    /// - false (selection mode): the crop area is covered with mask color, the rest of the screen is displayed as is
    ///
    /// Default to `true`
    pub selection_mode: bool,
    /// mask color, in RGBA format. Default to [0, 0, 0, 128]
    pub mask_color: [u8; 4],
}

impl Default for CropperConfig {
    fn default() -> CropperConfig {
        CropperConfig {
            auto_bounding: false,
            selection_mode: true,
            mask_color: [0, 0, 0, 128],
        }
    }
}

impl CropperConfig {
    pub fn get_mask_color(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(
            self.mask_color[0],
            self.mask_color[1],
            self.mask_color[2],
            self.mask_color[3],
        )
    }
}