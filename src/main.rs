use crate::genetic_algorithm::{agent::Sigmoid, trainer::Trainer};

pub mod genetic_algorithm;
pub mod tic_tac_toe;

fn main() {
    let mut trainer = Trainer {
        agents: vec![], // Agent, Fitness
        agent_count: 0,
        current_generation: 0
    };

    trainer.create_initial_generation(20, 9, 9, 50, 50, || Box::new(Sigmoid {}), 3.0);

    for epoch in 1..=10000 {
        println!("Epoch #{epoch}:");
        trainer.fit_generation(5);
        trainer.mutate_generation(5, 5, 2.0, 9, 9, 5, 5, || Box::new(Sigmoid {}), 3.0);
    }
}