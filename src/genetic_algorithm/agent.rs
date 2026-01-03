use std::{fs::File, io::{ErrorKind, Read, Write}};

use rand::Rng;

// Layers
#[derive(Clone)]
pub struct Layer {
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>
}

impl Layer {
    pub fn compute(&self, values: Vec<f64>) -> Vec<f64> {
        let mut final_values: Vec<f64> = vec![];

        for (i, inp) in self.weights.iter().enumerate() {
            let mut total: f64 = 0.0;

            for (j, weight) in inp.iter().enumerate() {
                total += weight * values[j];
            }

            total += self.biases[i];
            final_values.push(total);
        }

        final_values
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();

        for i in 0..self.weights.len() {
            for j in 0..self.weights[i].len() {
                self.weights[i][j] += rng.gen_range((-1.0 * mutation_rate)..mutation_rate);
            }

            self.biases[i] += rng.gen_range((-1.0 * mutation_rate)..mutation_rate);
        }
    }
}


// Activation Functions

pub trait ActivationFunctionClone {
    fn clone_box(&self) -> Box<dyn ActivationFunction>;
}

pub trait ActivationFunction: ActivationFunctionClone {
    fn calculate(&self, value: f64) -> f64;
}

#[derive(Clone)]
pub struct Sigmoid {}

#[derive(Clone)]
pub struct ReLU {}

impl ActivationFunction for Sigmoid {
    fn calculate(&self, value: f64) -> f64 {
        1.0 / (1.0 + f64::exp(-1.0 * value))
    }
}

impl ActivationFunctionClone for Sigmoid {
    fn clone_box(&self) -> Box<dyn ActivationFunction> {
        Box::new(self.clone())
    }
}

impl ActivationFunction for ReLU {
    fn calculate(&self, value: f64) -> f64 {
        value.max(0.0)
    }
}

impl ActivationFunctionClone for ReLU {
    fn clone_box(&self) -> Box<dyn ActivationFunction> {
        Box::new(self.clone())
    }
}


impl Clone for Box<dyn ActivationFunction> {
    fn clone(&self) -> Box<dyn ActivationFunction> {
        self.clone_box()
    }
}


#[derive(Clone)]
pub struct Agent {
    pub layers: Vec<Layer>,
    pub activation_function: Box<dyn ActivationFunction>
}



impl Agent {
    pub fn compute(&self, values: Vec<f64>) -> Vec<f64> {
        let mut current_inputs: Vec<f64> = values;

        for layer in self.layers.iter() {
            current_inputs = layer.compute(current_inputs);
            
            let mut activated_inputs: Vec<f64> = vec![];

            for value in current_inputs.iter() {
                activated_inputs.push(self.activation_function.calculate(*value));
            }

            current_inputs = activated_inputs;
        }

        current_inputs
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        for layer in self.layers.iter_mut() {
            layer.mutate(mutation_rate);
        }
    }

    pub fn save(&self, file_name: String) {
        let mut file = File::create(file_name).expect("Error lodaing file.");

        // Saving each layer.
        for layer in self.layers.iter() {
            let node_count: f64 = layer.biases.len() as f64;
            let input_count: f64 = layer.weights[0].len() as f64;

            file.write_all(&node_count.to_le_bytes()).expect("Error writing node count to file.");
            file.write_all(&input_count.to_le_bytes()).expect("Error writing input count to file.");

            // Writing weights.
            for node in &layer.weights {
                let weights_bytes: Vec<u8> = node.iter().flat_map(|x| x.to_le_bytes()).collect();
                file.write_all(&weights_bytes).expect("Error writing weight to file.");
            }

            // Writing biases.
            let bias_bytes: Vec<u8> = layer.biases.iter().flat_map(|x| x.to_le_bytes()).collect();
            file.write_all(&bias_bytes).expect("Error writing weight to file.");
        }
    }

    pub fn load(&mut self, target_file: String) {
        let mut file = File::open(target_file).expect("Error encountered when trying to loda file.");
        let mut values: Vec<f64> = Vec::new();
        let mut buffer = [0u8; 8];

        loop {
            match file.read_exact(&mut buffer) {
                Ok(()) => {
                    let value = f64::from_le_bytes(buffer);
                    values.push(value);
                }

                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }

                Err(e) => {
                    print!("Error encountered when reading file: {e}");
                    break;
                },
            }
        }

        let mut current_index: usize = 0;
        self.layers = vec![];

        while current_index < values.len() {
            let node_count: usize = values[current_index] as usize;
            let input_count: usize = values[current_index + 1] as usize;

            current_index += 2;
            let mut new_layer_weights: Vec<Vec<f64>> = vec![];

            for _ in 0..node_count {
                let mut new_node: Vec<f64> = vec![];

                for _ in 0..input_count {
                    new_node.push(values[current_index]);
                    current_index += 1
                }

                new_layer_weights.push(new_node);
            }
            
            let mut new_layer_biases: Vec<f64> = vec![];

            for _ in 0..node_count {
                new_layer_biases.push(values[current_index]);
                current_index += 1;
            }

            self.layers.push(Layer {
                weights: new_layer_weights,
                biases: new_layer_biases
            });
        }
    }
}