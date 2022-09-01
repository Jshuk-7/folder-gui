use egui::{Button, Key};

use std::env;
use std::fs;

pub struct Application {
    folders: Vec<String>,
    current_dir: String,
    last_dir: String,
    visiting_dir: bool,
}

impl Application {
    pub fn new() -> Self {
        let mut folders = vec![];

        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let dir_name = entry.file_name();
                        folders.push(dir_name.into_string().unwrap())
                    }
                }
            }
        }

        Self {
            folders,
            current_dir: String::new(),
            last_dir: String::new(),
            visiting_dir: false,
        }
    }

    fn main(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if !self.visiting_dir {
            let current_dir = env::current_dir().unwrap();
            ui.heading(current_dir.display().to_string());

            ui.add_space(20.0);
            ui.separator();

            for dir in self.folders.iter() {
                let button = ui.add_sized([250.0, 20.0], Button::new(dir));

                if button.clicked() {
                    self.visiting_dir = true;
                    self.last_dir = self.current_dir.clone();
                    self.current_dir = dir.clone();
                }

                ui.separator();
            }
        } else {
            ui.heading(&self.current_dir);
            ui.add_space(10.0);

            let button = ui.add_sized([75.0, 20.0], Button::new("<-- Go Back"));

            if button.clicked() {
                self.visiting_dir = false;
                //let dir = self.last_dir.clone();
                //self.visit_dir(ui, &dir);
            }

            let dir = self.current_dir.clone();

            self.visit_dir(ui, &dir)
        }

        let button = ui.add_sized([75.0, 20.0], Button::new("Exit"));

        if button.clicked() || ui.input().key_pressed(Key::Escape) {
            frame.close()
        }
    }

    fn visit_dir(&mut self, ui: &mut egui::Ui, dir: &str) {
        if let Ok(entries) = fs::read_dir(dir) {
            ui.add_space(20.0);
            ui.separator();

            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let dir_name = entry.file_name().into_string().unwrap();

                        let button = ui.add_sized([250.0, 20.0], Button::new(&dir_name));

                        if button.clicked() {
                            self.current_dir += &format!("/{}", &dir_name);

                            let dir = self.current_dir.clone();

                            //* Recursive Call
                            self.visit_dir(ui, dir.as_str())
                        }

                        ui.separator();
                    }
                }
            }
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, move |ui| {
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.set_max_width(600.0);

                    self.main(ui, frame)
                })
            })
        });
    }
}
