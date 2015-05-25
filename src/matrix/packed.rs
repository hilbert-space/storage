//! Packed matrices.
///
/// Data are stored in the [format][1] adopted by [LAPACK][2].
///
/// [1]: http://www.netlib.org/lapack/lug/node123.html
/// [2]: http://www.netlib.org/lapack

use matrix::dense;

/// A packed matrix.
#[derive(Debug)]
pub struct Matrix {
    /// The number of rows or columns.
    pub size: usize,
    /// The storage format.
    pub format: Format,
    /// The actual data.
    pub values: Vec<f64>,
}

/// The storage format of a packed matrix.
#[derive(Clone, Copy, Debug)]
pub enum Format {
    /// The lower triangular format.
    Lower,
    /// The upper triangular format.
    Upper,
}

impl From<Matrix> for dense::Matrix {
    fn from(matrix: Matrix) -> dense::Matrix {
        let Matrix { size, format, ref values } = matrix;

        debug_assert_eq!(values.len(), size * (size + 1) / 2);

        let mut dense = dense::Matrix {
            rows: size,
            columns: size,
            values: vec![0.0; size * size],
        };

        match format {
            Format::Lower => {
                let mut k = 0;
                for j in 0..size {
                    for i in j..size {
                        dense.values[j * size + i] = values[k];
                        k += 1;
                    }
                }
            },
            Format::Upper => {
                let mut k = 0;
                for j in 0..size {
                    for i in 0..(j + 1) {
                        dense.values[j * size + i] = values[k];
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
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let matrix: dense::Matrix = matrix.into();

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
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let matrix: dense::Matrix = matrix.into();

        assert::equal(&matrix[..], &vec![
            1.0, 0.0, 0.0,  0.0,
            2.0, 3.0, 0.0,  0.0,
            4.0, 5.0, 6.0,  0.0,
            7.0, 8.0, 9.0, 10.0,
        ]);
    }
}
