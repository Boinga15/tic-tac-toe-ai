use crate::genetic_algorithm::{agent::Sigmoid, trainer::Trainer};

pub fn train() {
    let mut trainer = Trainer {
        agents: vec![], // Agent, Fitness
        agent_count: 0,
        current_generation: 0
    };

    trainer.create_initial_generation(50, 9, 9, 120, 50, || Box::new(Sigmoid {}), 3.0);

    for epoch in 1..=10000 {
        println!("Epoch #{epoch}:");
        trainer.fit_generation(5);
        trainer.mutate_generation(5, 5, 2.0, 9, 9, 5, 5, || Box::new(Sigmoid {}), 3.0);

        if epoch % 1000 == 0 {
            trainer.agents[0].0.save(format!("checkpoint_{epoch}.bin"));
        }
    }
}