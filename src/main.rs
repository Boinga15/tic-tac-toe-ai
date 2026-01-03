pub mod genetic_algorithm;
pub mod train;
pub mod tic_tac_toe;

use eframe::egui;

use crate::{genetic_algorithm::agent::{Agent, Sigmoid}, tic_tac_toe::game::Board};

struct MainApp {
    ai_model: Agent,
    board: Board,
    wins: usize,
    losses: usize
}

impl MainApp {
    fn new(model_name: String) -> MainApp {
        let mut ai_model: Agent = Agent {
            layers: vec![],
            activation_function: Box::new(Sigmoid {})
        };

        ai_model.load(model_name);

        MainApp {
            ai_model: ai_model,
            board: Board {
                board: [
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ]
            },

            wins: 0,
            losses: 0
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading("Tic Tac Toe");
            ui.add_space(10.0);

            for row in 0..3 {
                ui.horizontal(|ui| {
                    for col in 0..3 {
                        let cell = &mut self.board.board[row][col];

                        let label = match cell {
                            -1 => "O",
                            0 => " ",
                            1 => "X",
                            _ => " "
                        };

                        let cell_button = ui.add_sized([60.0, 60.0], egui::Button::new(label));

                        if cell_button.clicked() && *cell == 0 {
                            *cell += 1;

                            if self.board.get_board_state() != 0 {
                                match self.board.get_board_state() {
                                    -1 => { self.losses += 1 },
                                    1 => { self.wins += 1 },
                                    _ => {}
                                }

                                self.board.board = [
                                    [0, 0, 0],
                                    [0, 0, 0],
                                    [0, 0, 0],
                                ]
                            }

                            let input_vector: Vec<f64> = {
                                let mut h_vector: Vec<f64> = vec![];

                                for i in 0..3 {
                                    for j in 0..3 {
                                        h_vector.push(self.board.board[i][j] as f64);
                                    }
                                }

                                h_vector
                            };

                            let probabilities = &mut self.ai_model.compute(input_vector);
                            self.board.probability_input(probabilities.to_vec(), true, -1);

                            if self.board.get_board_state() != 0 {
                                match self.board.get_board_state() {
                                    -1 => { self.losses += 1 },
                                    1 => { self.wins += 1 },
                                    _ => {}
                                }

                                self.board.board = [
                                    [0, 0, 0],
                                    [0, 0, 0],
                                    [0, 0, 0],
                                ]
                            }
                        }
                    }
                });
            }

            ui.add_space(10.0);

            let wins = self.wins;
            let losses = self.losses;
            ui.label(format!("Wins: {wins}, Losses: {losses}"));
        });
    }
}

fn main_loop() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("Tic Tac Toe", options, Box::new(|_cc| Ok(Box::new(MainApp::new("model.bin".to_string())))))
}

fn main() -> eframe::Result<()> {
    main_loop()
}