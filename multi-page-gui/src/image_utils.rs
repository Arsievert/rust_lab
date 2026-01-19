use eframe::egui;
use std::path::PathBuf;

pub fn load_image(ui: &mut egui::Ui, path: &str) -> Option<egui::TextureHandle> {
    let mut path_buf = PathBuf::new();
    path_buf.push(path);

    let image = match image::open(&path_buf) {
        Ok(img) => img,
        Err(_) => return None,
    };

    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    Some(ui.ctx().load_texture(
        "background_image",
        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
        egui::TextureOptions::LINEAR,
    ))
}
