pub fn sigmoid_singleton(x: f32) -> f32 {
    1. / (1. + 2.718281828_f32.powf(-x))
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
    assert!(sigmoid_singleton(0.) == 0.5);
}

#[test]
fn sigmoid_singleton_999_1() {
    assert!(sigmoid_singleton(999.) == 1.);
}

#[test]
fn sigmoid_singleton_neg999_0() {
    assert!(sigmoid_singleton(-999.) == 0.);
}
