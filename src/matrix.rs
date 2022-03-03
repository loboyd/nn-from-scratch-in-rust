use crate::utils::sigmoid_singleton;
use crate::utils::RandomNumberGenerator;

#[derive(Clone)]
pub struct Matrix {
    pub n_rows: usize,
    pub n_cols: usize,
    pub data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn zeros(n_rows: usize, n_cols: usize) -> Self {
        Self {
            n_rows: n_rows,
            n_cols: n_cols,
            data: (0..n_rows)
                .map(|_| (0..n_cols).map(|_| 0.).collect::<Vec<f32>>())
                .collect::<Vec<Vec<f32>>>(),
        }
    }

    pub fn random(n_rows: usize, n_cols: usize, rng: &mut RandomNumberGenerator) -> Self {
        Self {
            n_rows: n_rows,
            n_cols: n_cols,
            data: (0..n_rows)
                .map(|_| (0..n_cols).map(|_| 2. * rng.gen() - 1.).collect::<Vec<f32>>())
                .collect::<Vec<Vec<f32>>>(),
        }
    }

    // TODO: Should this just mutate `self`?
    pub fn sigmoid(self) -> Self {
        let mut new = self.clone();
        for row in 0..new.n_rows {
            for col in 0..new.n_cols {
                new.data[row][col] = sigmoid_singleton(new.data[row][col]);
            }
        }

        new
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut r = Self::zeros(self.n_rows, rhs.n_cols);
        for row in 0..self.n_rows {
            for col in 0..rhs.n_cols {
                for k in 0..self.n_cols {
                    r.data[row][col] += self.data[row][k] * rhs.data[k][col];
                }
            }
        }
        r
    }
}

impl std::ops::Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Matrix {
        let mut r = Matrix::zeros(self.n_rows, rhs.n_cols);
        for row in 0..self.n_rows {
            for col in 0..rhs.n_cols {
                for k in 0..self.n_cols {
                    r.data[row][col] += self.data[row][k] * rhs.data[k][col];
                }
            }
        }
        r
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|row| row
                    .iter()
                    .map(|col| format!("{:4.2}", col))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
