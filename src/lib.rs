//! Data storage schemes.

#[cfg(test)]
extern crate assert;

extern crate num;

mod matrix;

pub use matrix::band::Matrix as BandMatrix;
pub use matrix::compressed::Matrix as CompressedMatrix;
pub use matrix::compressed::Format as CompressedFormat;
pub use matrix::diagonal::Matrix as DiagonalMatrix;
pub use matrix::packed::Matrix as PackedMatrix;
pub use matrix::packed::Format as PackedFormat;
