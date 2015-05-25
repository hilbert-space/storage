//! Packed matrices.
//!
//! Data are stored in the [format][1] adopted by [LAPACK][2].
//!
//! [1]: http://www.netlib.org/lapack/lug/node123.html
//! [2]: http://www.netlib.org/lapack

use num::{Num, Zero};

use matrix::dense;

/// A packed matrix.
#[derive(Debug)]
pub struct Matrix<T> {
    /// The number of rows or columns.
    pub size: usize,
    /// The storage format.
    pub format: Format,
    /// The data stored in the column-major order.
    pub data: Vec<T>,
}

/// The storage format of a packed matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    /// The lower triangular format.
    Lower,
    /// The upper triangular format.
    Upper,
}

impl<T> From<Matrix<T>> for dense::Matrix<T> where T: Copy + Num {
    fn from(matrix: Matrix<T>) -> dense::Matrix<T> {
        let Matrix { size, format, ref data } = matrix;

        debug_assert_eq!(data.len(), size * (size + 1) / 2);

        let mut dense = dense::Matrix {
            rows: size,
            columns: size,
            data: vec![Zero::zero(); size * size],
        };

        match format {
            Format::Lower => {
                let mut k = 0;
                for j in 0..size {
                    for i in j..size {
                        dense.data[j * size + i] = data[k];
                        k += 1;
                    }
                }
            },
            Format::Upper => {
                let mut k = 0;
                for j in 0..size {
                    for i in 0..(j + 1) {
                        dense.data[j * size + i] = data[k];
                        k += 1;
                    }
                }
            },
        }

        dense
    }
}

#[cfg(test)]
mod tests {
    use assert;
    use matrix::dense;

    #[test]
    fn into_lower_dense() {
        let matrix = super::Matrix {
            size: 4,
            format: super::Format::Lower,
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let matrix: dense::Matrix<f64> = matrix.into();

        assert::equal(&matrix[..], &vec![
            1.0, 2.0, 3.0,  4.0,
            0.0, 5.0, 6.0,  7.0,
            0.0, 0.0, 8.0,  9.0,
            0.0, 0.0, 0.0, 10.0,
        ]);
    }

    #[test]
    fn into_upper_dense() {
        let matrix = super::Matrix {
            size: 4,
            format: super::Format::Upper,
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let matrix: dense::Matrix<f64> = matrix.into();

        assert::equal(&matrix[..], &vec![
            1.0, 0.0, 0.0,  0.0,
            2.0, 3.0, 0.0,  0.0,
            4.0, 5.0, 6.0,  0.0,
            7.0, 8.0, 9.0, 10.0,
        ]);
    }
}
