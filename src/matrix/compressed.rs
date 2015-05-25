use num::{Num, Zero};

use matrix::dense;

/// A compressed matrix.
///
/// Data are stored in one of the following formats:
///
/// * the [compressed-row][1] format or
/// * the [compressed-column][2] format.
///
/// [1]: http://netlib.org/linalg/html_templates/node91.html
/// [2]: http://netlib.org/linalg/html_templates/node92.html
#[derive(Debug)]
pub struct Matrix<T> {
    /// The number of rows.
    pub rows: usize,
    /// The number of columns.
    pub columns: usize,
    /// The number of nonzero elements.
    pub nonzeros: usize,
    /// The storage format.
    pub format: Format,
    /// The values of the nonzero elements.
    pub data: Vec<T>,
    /// The indices of columns (rows) the nonzero elements.
    pub indices: Vec<usize>,
    /// The offsets of columns (rows) such that the values and indices of the `i`th column (row)
    /// are stored starting from `data[j]` and `indices[j]`, respectively, where `j = offsets[i]`.
    /// The vector has one additional element, which is always equal to `nonzeros`, that is,
    /// `offsets[columns] = nonzeros` (`offsets[rows] = nonzeros`).
    pub offsets: Vec<usize>,
}

/// The storage format of a compressed matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    /// The compressed-row format.
    Row,
    /// The compressed-column format.
    Column,
}

impl<T> From<Matrix<T>> for dense::Matrix<T> where T: Copy + Num {
    fn from(matrix: Matrix<T>) -> dense::Matrix<T> {
        let Matrix { rows, columns, nonzeros, format, ref data, ref indices, ref offsets } = matrix;

        debug_assert_eq!(data.len(), nonzeros);
        debug_assert_eq!(indices.len(), nonzeros);

        let mut dense = dense::Matrix {
            rows: rows,
            columns: columns,
            data: vec![Zero::zero(); rows * columns],
        };

        match format {
            Format::Row => {
                debug_assert_eq!(offsets.len(), rows + 1);
                for i in 0..rows {
                    for k in offsets[i]..offsets[i + 1] {
                        let j = indices[k];
                        dense.data[j * rows + i] = data[k];
                    }
                }
            },
            Format::Column => {
                debug_assert_eq!(offsets.len(), columns + 1);
                for j in 0..columns {
                    for k in offsets[j]..offsets[j + 1] {
                        let i = indices[k];
                        dense.data[j * rows + i] = data[k];
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
    fn into_dense() {
        let matrix = super::Matrix {
            rows: 5,
            columns: 3,
            nonzeros: 3,
            format: super::Format::Column,
            data: vec![1.0, 2.0, 3.0],
            indices: vec![0, 1, 2],
            offsets: vec![0, 1, 2, 3],
        };

        let matrix: dense::Matrix<f64> = matrix.into();

        assert::equal(&matrix[..], &vec![
            1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 3.0, 0.0, 0.0,
        ]);
    }
}
