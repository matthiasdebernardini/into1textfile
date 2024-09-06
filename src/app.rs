use crate::{list_files, process_files};
use egui::TextEdit;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Into1TextFileApp {
    input_text: String,
    output_text: String,
}

impl Default for Into1TextFileApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
        }
    }
}

impl Into1TextFileApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for Into1TextFileApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Enter text: ");
                ui.text_edit_singleline(&mut self.input_text);
            });
            if ui.button("Process Text").clicked() {
                let path = PathBuf::from(self.input_text.as_str());
                let dir_path = PathBuf::from(path);
                let files = list_files(&dir_path).unwrap();
                self.output_text = process_files(files).unwrap();
            }
            ui.separator();
            if ui.button("Copy to Clipboard").clicked() {
                ui.output_mut(|mut o| o.copied_text = self.output_text.clone())
            }
            ui.separator();
            ui.add(
                TextEdit::multiline(&mut self.output_text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(10),
            );
        });
    }
}
