use super::folder_gui::Application;

use std::env;

use eframe::{run_native, NativeOptions};
use egui::{pos2, vec2};

pub fn run_app() {
    let initial_window_size = Some(vec2(800.0, 600.0));
    let initial_window_pos = Some(pos2(
        (1920.0 / 2.0) - (800.0 / 2.0),
        (1080.0 / 2.0) - (600.0 / 2.0),
    ));

    let options = NativeOptions {
        initial_window_size,
        initial_window_pos,
        ..Default::default()
    };

    let current_dir = env::current_dir().unwrap();

    run_native(
        current_dir.display().to_string().as_str(),
        options,
        Box::new(|_cc| Box::new(Application::new())),
    )
}
