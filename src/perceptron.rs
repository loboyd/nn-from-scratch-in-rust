use crate::matrix::Matrix;
use crate::utils;

pub struct Perceptron {
    pub n_inputs: usize,
    pub n_outputs: usize,
    pub layers: Vec<Matrix>,
    n_layers: usize,
}

struct TrainingCache(Vec<LayerTrainingCache>);

struct LayerTrainingCache {
    weighted_input: Vec<f32>,
    activated_output: Vec<f32>,
}

impl Perceptron {
    pub fn new(n_inputs: usize, n_outputs: usize, layers: Vec<Matrix>) -> Self {
        let n_layers = layers.len();
        Perceptron {
            n_inputs: n_inputs,
            n_outputs: n_outputs,
            layers: layers,
            n_layers: n_layers,
        }
    }

    pub fn feed_forward(&self, input: Vec<f32>) -> Vec<f32> {
        let mut x = Matrix {
            n_rows: 1,
            n_cols: input.len(),
            data: vec![input.clone()],
        };

        for layer in &self.layers {
            x = (&x * layer).sigmoid();
        }

        x.data[0].clone()
    }

    /// Same thing as `self.feed_forward` but with caching
    fn feed_forward_train(&self, training_cache: &mut TrainingCache, input: &Vec<f32>) -> Vec<f32> {
        let mut x = Matrix {
            n_rows: 1,
            n_cols: input.len(),
            data: vec![input.clone()],
        };

        for layer in &self.layers {
            let weighted = &x * layer;
            let activated = weighted.clone().sigmoid();

            training_cache.0.push(LayerTrainingCache {
                weighted_input: weighted.data[0].clone(),
                activated_output: activated.data[0].clone(),
            });

            x = activated;
        }

        x.data[0].clone()
    }

    // TODO: Abstract this out
    /// Compute the cost according a given training pair
    fn cost(&self, input: Vec<f32>, label: Vec<f32>) -> f32 {
        let output = self.feed_forward(input);

        let diff_vector = label.iter().zip(output).map(|(x, y)| x - y).collect::<Vec<f32>>();

        diff_vector.iter().map(|x| x * x).sum::<f32>()
    }

    // TODO: Abstract this out
    fn grad_cost_last_layer(input: f32, last_layer_activation: f32) -> f32 {
        last_layer_activation - input
    }

    fn grad(&mut self, input: &Vec<f32>, label: Vec<f32>) {
        // set up training cache
        let mut training_cache = TrainingCache(Vec::new());

        // compute the output of the network
        let output = self.feed_forward_train(&mut training_cache, input);

        // compute the error of the last layer (vector-valued)

        let last_layer_activation = &training_cache.0[self.n_layers].activated_output;

        let grad_cost_wrt_last_layer_activation = input
            .iter()
            .zip(last_layer_activation)
            .map(|(x, y)| Self::grad_cost_last_layer(*x, *y))
            .collect::<Vec<f32>>();

        let grad_last_layer_activation_wrt_input = utils::sigmoid_derivative_vector(
            &training_cache.0[self.n_layers].weighted_input
        );

        // Hadamard product
        let last_layer_error = grad_cost_wrt_last_layer_activation
            .iter()
            .zip(grad_last_layer_activation_wrt_input)
            .map(|(x, y)| *x * y)
            .collect::<Vec<f32>>();

        // backpropagate the error

        // return the updates
    }
}
