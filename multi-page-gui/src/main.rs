mod image_utils;

use eframe::{egui, App, Frame};
use image_utils::load_image;
use egui::TextureHandle;

pub struct MyApp {
    background_texture: Option<TextureHandle>,
    current_page: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            background_texture: None,
            current_page: 0,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_background(ui);
            self.render_navigation_buttons(ui);
            self.render_current_page(ui);
        });
    }
}

impl MyApp {
    fn draw_background(&mut self, ui: &mut egui::Ui) {
        if self.background_texture.is_none() {
            self.background_texture = load_image(ui, "test.png");
        }

        if let Some(texture) = &self.background_texture {
            let available_size = ui.available_size();
            let background_rect = egui::Rect::from_min_size(ui.min_rect().min, available_size);

            ui.painter().image(
                texture.id(),
                background_rect,
                egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }

    fn render_navigation_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if create_button(ui, "Previous").clicked() && self.current_page > 0 {
                self.current_page -= 1;
            }
            if create_button(ui, "Next").clicked() && self.current_page < 1 {
                self.current_page += 1;
            }
        });
    }

    fn render_current_page(&self, ui: &mut egui::Ui) {
        match self.current_page {
            0 => display_page_1(ui),
            1 => display_page_2(ui),
            _ => { ui.label("Unknown Page"); }
        }
    }
}

fn create_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.button(label)
}

fn display_page_1(ui: &mut egui::Ui) {
    ui.label("Page 1: Content on background");
}

fn display_page_2(ui: &mut egui::Ui) {
    ui.label("Page 2: Content on background");
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "App with Background Image",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
