#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::fs;

#[derive(Default)]
struct MyApp {
    text: String,
    picked_path: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text));
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = path.display().to_string();
                            println!("{:?}", self.picked_path);
                            
                            // Try to read the file
                            match fs::read_to_string(&self.picked_path) {
                                Ok(content) => {
                                    self.text = content;
                                }
                                Err(error) => {
                                    self.text = format!("Error reading file: {}", error);
                                    eframe::get_window_handle(ctx).unwrap().set_title("File Reader - Error");
                                }
                            }
                        }
                    }
                    if ui.button("Save").clicked() {
                        // functionality
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        // functionality
                    }
                    if ui.button("Copy").clicked() {
                        // functionality
                    }
                    if ui.button("Paste").clicked() {
                        // functionality
                    }
                    if ui.button("Encrypt").clicked() {
                        println!("encrypt....");
                    }
                })
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "File Reader",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}