mod skin;

use eframe::{egui, IconData};
use include_dir::{include_dir, Dir};
use skin::Skin;
use std::{fs, path::PathBuf};

const ASSETS: Dir = include_dir!("assets");

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct SkinFlipper {
    input: PathBuf,
    input_str: String,
    output_msg: String,
}

fn main() -> Result<(), eframe::Error> {
    let icon_bytes = image::load_from_memory(
        ASSETS
            .get_file("icon.ico")
            .unwrap() // unwrap because this should always be possible unless compiled by a fool
            .contents(),
    )
    .unwrap()
    .to_rgba8()
    .to_vec();

    let icon = IconData {
        rgba: icon_bytes,
        width: 32,
        height: 32,
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(360.0, 140.0)),
        icon_data: Some(icon),
        ..Default::default()
    };

    eframe::run_native(
        &format!("osu! Skin flipper {}", VERSION),
        options,
        Box::new(|_cc| Box::<SkinFlipper>::default()),
    )?;
    Ok(())
}

impl eframe::App for SkinFlipper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("osu! Skin flipper");
                ui.label("Enter the directory path:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input_str);

                    if ui
                        .button("Browse")
                        .on_hover_text("Browse for a directory")
                        .clicked()
                    {
                        let res = rfd::FileDialog::new().pick_folder();
                        match res {
                            Some(path) => {
                                self.input_str = path.to_string_lossy().to_string();
                                self.input = path
                            }
                            None => self.output_msg = "No directory selected".to_string(),
                        }
                    }
                });

                if ui.button("Start").clicked() {
                    self.output_msg = format!(
                        "Starting process with directory: {}",
                        self.input.to_string_lossy()
                    );
                    let skin = match Skin::new(self.input_str.clone()).create_new_skin() {
                        Ok(skin) => skin,
                        Err(e) => {
                            self.output_msg = format!("Error: {}", e);
                            return;
                        }
                    };

                    match skin.flip_default_numbers() {
                        Ok(_) => self.output_msg = format!("Successfully flipped skin"),
                        Err(e) => self.output_msg = format!("Error: {}", e),
                    }

                    match skin.into_osk() {
                        Ok(f) => {
                            self.output_msg =
                                format!("Successfully created osk at {}. Attempting import...", &f);
                            match opener::open(f) {
                                Ok(_) => self.output_msg = format!("Successfully imported osk"),
                                Err(e) => self.output_msg = format!("Error: {}", e),
                            }
                            fs::remove_dir_all(skin.location).unwrap();
                        }
                        Err(e) => self.output_msg = format!("Error: {}", e),
                    }
                }

                ui.group(|ui| {
                    ui.label(&self.output_msg);
                });
            });
        });
    }
}
