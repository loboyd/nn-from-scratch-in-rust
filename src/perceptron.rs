use crate::matrix::Matrix;

pub struct Perceptron {
    pub n_inputs: usize,
    pub n_outputs: usize,
    pub layers: Vec<Matrix>,
}

impl Perceptron {
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
}
