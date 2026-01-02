pub mod genetic_algorithm;
pub mod train;
pub mod tic_tac_toe;

use eframe::egui;

#[derive(Default)]
struct MainApp {}

impl eframe::App for MainApp {
    fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading("Hello World!");
        });
    }
}

fn main_loop() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("Tic Tac Toe", options, Box::new(|_cc| Ok(Box::new(MainApp::default()))))
}

fn main() -> eframe::Result<()> {
    main_loop()
}