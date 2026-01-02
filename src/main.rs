pub mod genetic_algorithm;
pub mod train;
pub mod tic_tac_toe;

use eframe::egui;

use crate::{genetic_algorithm::agent::{Agent, Sigmoid}, train::train::train};

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

//  -> eframe::Result<()> (Add Pls)
fn main() {
    
    let mut new_agent: Agent = Agent {
        layers: vec![],
        activation_function: Box::new(Sigmoid {})
    };

    new_agent.load("model.bin".to_string());
    
    //train();
}