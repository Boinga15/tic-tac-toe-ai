use rand::Rng;

use crate::{genetic_algorithm::{agent::{ActivationFunction, Agent}, factory::generate_agent}, tic_tac_toe::game::Board};

pub struct Trainer {
    pub agents: Vec<(Agent, f64)>, // Agent, Fitness
    pub agent_count: usize,
    pub current_generation: usize
}

impl Trainer {
    pub fn create_initial_generation<F>(&mut self, agent_count: usize, input_count: u32, output_count: u32, hidden_layer_count: u32, nodes_per_hidden_layer: u32, activation_function: F, random_range: f64) where F: Fn() -> Box<dyn ActivationFunction>, {
        self.current_generation = 1;
        self.agents = vec![];

        for _i in 0..agent_count {
            self.agents.push((generate_agent(input_count, output_count, hidden_layer_count, nodes_per_hidden_layer, activation_function(), random_range), 0.0));
        }       

        self.agent_count = agent_count; 
    }


    pub fn fit_generation(&mut self, min_trials_per_agent: usize) {
        let mut rng = rand::thread_rng();
        let len = self.agents.len();

        let mut trials: Vec<usize> = self.agents.iter().map(|_x| 0).collect();

        for i in 0..len {
            while trials[i] <= min_trials_per_agent {
                // Run a trial with this and another agent.
                let chosen_index = rng.gen_range(0..len);

                if chosen_index == i {
                    continue;
                }

                if let Ok([current_agent, chosen_agent]) = self.agents.get_disjoint_mut([i, chosen_index]) {
                    // Now run the game.
                    let mut board: Board = Board {
                        board: [
                            [0, 0, 0],
                            [0, 0, 0],
                            [0, 0, 0]
                        ]
                    };

                    let mut chosen_turn = rng.gen_bool(0.5);

                    while board.get_board_state() == 0 {
                        let input_vector: Vec<f64> = {
                            let mut h_vector: Vec<f64> = vec![];

                            for i in 0..board.board.len() {
                                for j in 0..board.board[i].len() {
                                    h_vector.push(board.board[i][j] as f64);
                                }
                            }

                            h_vector
                        };

                        if chosen_turn {
                            let probabilities: Vec<f64> = chosen_agent.0.compute(input_vector);
                            board.probability_input(probabilities, true, 1);
                        } else {
                            let probabilities: Vec<f64> = current_agent.0.compute(input_vector);
                            board.probability_input(probabilities, true, -1);
                        }

                        chosen_turn = !chosen_turn;
                    }

                    // Both AIs have now played a game.
                    trials[i] += 1;
                    trials[chosen_index] += 1;

                    match board.get_board_state() {
                        1 => {
                            chosen_agent.1 += 3.0;
                            current_agent.1 -= 1.0;
                        }

                        -1 => {
                            chosen_agent.1 -= 1.0;
                            current_agent.1 += 3.0;
                        }

                        2 => {
                            chosen_agent.1 += 1.0;
                            current_agent.1 += 1.0;
                        }

                        _ => {}
                    }
                };
            }
        }

        // Adjust fitness to number of trials.
        for (i, agent) in self.agents.iter_mut().enumerate() {
            agent.1 /= trials[i] as f64;
        }

        // Finally, sort and store agents.
        self.agents.sort_by(|a, b| b.1.total_cmp(&a.1));

        // DEBUG - Print fitness.
        for agent in self.agents.iter() {
            let fitness = agent.1;
            println!("{fitness}");
        }
        println!("========================================");
    }


    pub fn mutate_generation<F>(&mut self, winning_selection_size: usize, new_generation_size: usize, mutation_rate: f64, input_count: u32, output_count: u32, hidden_layer_count: u32, nodes_per_hidden_layer: u32, activation_function: F, random_range: f64) where F: Fn() -> Box<dyn ActivationFunction>, {
        let mut new_agent_list: Vec<(Agent, f64)> = vec![];        
        
        for i in 0..winning_selection_size {
            let new_agent = self.agents[i].clone().0;
            new_agent_list.push((new_agent, 0.0));
        }

        // New Generation.
        for _i in 0..new_generation_size {
            new_agent_list.push((generate_agent(input_count, output_count, hidden_layer_count, nodes_per_hidden_layer, activation_function(), random_range), 0.0));
        }

        // Mutate Winners.
        let mut c_index: usize = 0;

        while new_agent_list.len() < self.agent_count {
            let mut new_agent: Agent = self.agents[c_index].clone().0;
            new_agent.mutate(mutation_rate);

            new_agent_list.push((new_agent, 0.0));

            c_index += 1;

            if c_index >= winning_selection_size {
                c_index = 0;
            }
        }

        self.agents = new_agent_list;
    }
}