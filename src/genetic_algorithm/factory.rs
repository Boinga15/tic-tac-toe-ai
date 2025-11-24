use rand::Rng;

use crate::genetic_algorithm::agent::{ActivationFunction, Agent, Layer};

pub fn generate_layer(previous_input_count: u32, node_count: u32, random_range: f64) -> Layer {
    let mut rng = rand::thread_rng();
    let mut weights: Vec<Vec<f64>> = vec![];
    let mut biases: Vec<f64> = vec![];

    for _i in 0..node_count {
        let mut new_weights: Vec<f64> = vec![];

        for _j in 0..previous_input_count {
            new_weights.push(rng.gen_range((-1.0 * random_range)..random_range));
        }
        biases.push(rng.gen_range((-1.0 * random_range)..random_range));
        weights.push(new_weights);
    }

    Layer {
        weights: weights,
        biases: biases
    }
}


pub fn generate_agent(input_count: u32, output_count: u32, hidden_layer_count: u32, nodes_per_hidden_layer: u32, activation_function: Box<dyn ActivationFunction>, random_range: f64) -> Agent {
    let mut layers: Vec<Layer> = vec![];

    for i in 0..(hidden_layer_count + 1) {
        let mut previous_input: u32 = hidden_layer_count;
        let mut node_count: u32 = nodes_per_hidden_layer;

        if i == 0 {
            previous_input = input_count;
        } else if i == hidden_layer_count {
            node_count = output_count;
        }

        layers.push(generate_layer(previous_input, node_count, random_range));
    }

    Agent {
        layers: layers,
        activation_function: activation_function
    }
}