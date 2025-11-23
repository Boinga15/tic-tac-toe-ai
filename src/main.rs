use crate::genetic_algorithm::{agent::{Agent, Layer, Sigmoid}};

pub mod genetic_algorithm;

fn main() {
    let test_agent: Agent = Agent {
        activation_function: Box::new(Sigmoid {}),
        layers: vec![
            Layer {
                weights: vec![
                    vec![0.0, 1.5, -0.4],
                    vec![-0.8, -1.6, 0.5],
                    vec![0.8, 1.6, 1.2]
                ],
                biases: vec![1.1, 0.4, -0.7]
            },
            Layer {
                weights: vec![
                    vec![0.5, 2.2, -1.4],
                    vec![-3.2, 1.1, 0.0]
                ],
                biases: vec![-1.4, 0.1]
            }
        ]
    };

    let result: Vec<f64> = test_agent.compute(vec![0.4, 0.6, -1.2]);
    print!("{result:?}");
}