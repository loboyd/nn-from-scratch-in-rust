mod matrix;
mod mnist_reader;
mod perceptron;
mod utils;

use crate::matrix::Matrix;
use crate::mnist_reader::get_data;
use crate::perceptron::Perceptron;
use crate::utils::RandomNumberGenerator;

fn main() {
    // get the MNIST images
    let training_images = get_data();

    //for image in training_images.iter().take(6) {
    //    println!("\n{}", image);
    //}

    let mut rng = RandomNumberGenerator::with_seed(98.);

    // build layers
    let layer1 = Matrix::random(28*28, 16, &mut rng); // read the 28*28 pixels of the image
    let layer2 = Matrix::random(16, 10, &mut rng); // give the output as a choice of ten

    // build net
    let perceptron = Perceptron {
        n_inputs: 28*28,
        n_outputs: 10,
        layers: vec![layer1.clone(), layer2.clone()],
    };

    // pass first training image through net
    let output = perceptron.feed_forward(training_images[0].data.clone());
    println!("{:?}", output);
}
