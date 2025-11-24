use crate::genetic_algorithm::{agent::{Agent, Sigmoid}, factory::{generate_agent}};

pub mod genetic_algorithm;

fn main() {
    let test_agent: Agent = generate_agent(3, 2, 5, 10, Box::new(Sigmoid {}), 5.0);

    let result: Vec<f64> = test_agent.compute(vec![0.4, 0.6, -1.2]);
    print!("{result:?}");
}