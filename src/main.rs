#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::fs;

#[derive(Default)]
struct MyApp {
    text: String,
    picked_path: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::default().ui(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Ouvrir").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = path.display().to_string();
                            match fs::read_to_string(&self.picked_path) {
                                Ok(content) => {
                                    self.text = content;
                                }
                                Err(error) => {
                                    self.text = format!("Erreur de lecture : {error}");
                                }
                            }
                        }
                    }
                    ui.separator();
                    if ui.button("Quitter").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            if self.picked_path.is_empty() {
                ui.add_space(20.0);
                ui.weak("Cliquez sur Fichier > Ouvrir pour sélectionner un fichier texte.");
            } else {
                ui.label(&self.picked_path);
                ui.separator();
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text));
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(800.0, 600.0))
            .with_title("Lecteur de fichiers texte"),
        ..Default::default()
    };
    eframe::run_native(
        "Lecteur de fichiers texte",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
