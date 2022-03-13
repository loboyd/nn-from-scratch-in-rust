use crate::utils::sigmoid_singleton;
use crate::utils::RandomNumberGenerator;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn sigmoid(&self) -> Self {
        Self {
            n_rows: self.n_rows,
            n_cols: self.n_cols,
            data: self.data
                .iter()
                .map(|row| row.iter().map(|col| sigmoid_singleton(*col)).collect::<Vec<f32>>())
                .collect::<Vec<Vec<f32>>>(),
        }
    }

    // TODO: Maybe eventually create "virtual" TransposedMatrix type which has a reference to the
    //       data and just re-implements the multiplication function
    pub fn transpose(&self) -> Self {
        let mut output = Self::zeros(self.n_cols, self.n_rows);
        for i in 0..self.n_cols {
            for j in 0..self.n_rows {
                output.data[i][j] += self.data[j][i];
            }
        }

        output
    }

    pub fn to_vec(&self) -> Option<Vec<f32>> {
        match (self.n_rows, self.n_cols) {
            (1, 1) => Some(vec![self.data[0][0]]),
            (1, _) => Some(self.data[1].clone()),
            (_, 1) => Some((0..self.n_rows).map(|ind| self.data[ind][0]).collect::<Vec<f32>>()),
            _ => None,
        }
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

#[test]
fn matrix_transpose_check() {
    let matrix = Matrix {
        n_rows: 2,
        n_cols: 3,
        data: vec!(
            vec!(1., 2., 3.),
            vec!(4., 5., 6.),
        ),
    };

    let matrix_transposed = Matrix {
        n_rows: 3,
        n_cols: 2,
        data: vec!(
            vec!(1., 4.),
            vec!(2., 5.),
            vec!(3., 6.),
        ),
    };

    assert_eq!(matrix.transpose(), matrix_transposed);
}
