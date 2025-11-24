use rand::Rng;

// Layers
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
pub trait ActivationFunction {
    fn calculate(&self, value: f64) -> f64;
}

pub struct Sigmoid {}
pub struct ReLU {}

impl ActivationFunction for Sigmoid {
    fn calculate(&self, value: f64) -> f64 {
        1.0 / (1.0 + f64::exp(-1.0 * value))
    }
}

impl ActivationFunction for ReLU {
    fn calculate(&self, value: f64) -> f64 {
        if value >= 0.0 {
            return value;
        }

        return 0.0;
    }
}


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
}