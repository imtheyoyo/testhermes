
use eframe::egui;
use rfd::FileDialog;
use std::fs;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Rust Text Reader",
        options,
        Box::new(|_cc| Box::new(FileReaderApp::default())),
    )
}

struct FileReaderApp {
    content: String,
    file_path: String,
}

impl Default for FileReaderApp {
    fn default() -> Self {
        Self {
            content: "Please open a file to see its content.".to_string(),
            file_path: "".to_string(),
        }
    }
}

impl eframe::App for FileReaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text File Reader");

            if ui.button("📂 Open File").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Text", &["txt", "rs", "md", "log"])
                    .pick_file() 
                {
                    match fs::read_to_string(&path) {
                        Ok(text) => {
                            self.content = text;
                            self.file_path = path.display().to_string();
                        }
                        Err(e) => {
                            self.content = format!("Error reading file: {}", e);
                        }
                    }
                }
            }

            if !self.file_path.is_empty() {
                ui.label(format!("File: {}", self.file_path));
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.content)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .lock_focus(true),
                );
            });
        });
    }
}
