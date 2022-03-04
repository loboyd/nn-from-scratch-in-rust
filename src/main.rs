mod matrix;
mod mnist_reader;
mod perceptron;
mod utils;

use crate::matrix::Matrix;
use crate::mnist_reader::get_data;
use crate::perceptron::Perceptron;
use crate::utils::RandomNumberGenerator;

fn main() {
    let data = get_data();
    for x in data.iter().take(6) {
        println!("\n{}", x);
    }

    //let mut rng = RandomNumberGenerator::with_seed(98.);

    //let layer1 = Matrix::random(3, 16, &mut rng);
    //let layer2 = Matrix::random(16, 1, &mut rng);

    //let perceptron = Perceptron {
    //    n_inputs: 3,
    //    n_outputs: 1,
    //    layers: vec![layer1.clone(), layer2.clone()],
    //};

    //println!("layer 1:\n{}", layer1);
    //println!();
    //println!("layer 2:\n{}", layer2);

    //let output = perceptron.feed_forward(vec![2., 3., 4.]);

    //println!("{:?}", output);
}
