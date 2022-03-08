pub fn sigmoid_singleton(x: f32) -> f32 {
    1. / (1. + 2.718281828_f32.powf(-x))
}

pub fn sigmoid_derivative(x: f32) -> f32 {
    sigmoid_singleton(x) * (1. - sigmoid_singleton(x))
}

pub fn sigmoid_derivative_vector(x: &Vec<f32>) -> Vec<f32> {
    x.iter().map(|i| sigmoid_derivative(*i)).collect::<Vec<f32>>()
}

pub struct RandomNumberGenerator {
    current: f32,
}

impl RandomNumberGenerator {
    const P1: f32 = 99923.;
    const P2: f32 = 65557.;
    const W: f32 = 1024.; // Is this necessary?

    pub fn with_seed(seed: f32) -> Self {
        Self {
            current: seed % Self::W,
        }
    }

    /// Generates a random `f32` between 0. and 1.
    pub fn gen(&mut self) -> f32 {
        self.current = (Self::P1 * self.current + Self::P2) % Self::W;
        self.current / Self::W
    }
}

#[test]
fn sigmoid_singleton_0_0point5() {
    assert_eq!(sigmoid_singleton(0.), 0.5);
}

#[test]
fn sigmoid_singleton_999_1() {
    assert_eq!(sigmoid_singleton(999.), 1.);
}

#[test]
fn sigmoid_singleton_neg999_0() {
    assert_eq!(sigmoid_singleton(-999.), 0.);
}

#[test]
fn sigmoid_derivative_0_0point5() {
    assert_eq!(sigmoid_derivative(0.), 0.25);
}

#[test]
fn sigmoid_derivative_999_0() {
    assert_eq!(sigmoid_derivative(-999.), 0.);
}

#[test]
fn sigmoid_derivative_neg999_0() {
    assert_eq!(sigmoid_derivative(-999.), 0.);
}
