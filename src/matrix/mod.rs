//! Matrix storage schemes.

mod band;
mod compressed;
mod dense;
mod diagonal;
mod packed;

pub use self::band::Matrix as Band;
pub use self::compressed::Matrix as Compressed;
pub use self::compressed::Format as CompressedFormat;
pub use self::dense::Matrix as Dense;
pub use self::diagonal::Matrix as Diagonal;
pub use self::packed::Matrix as Packed;
pub use self::packed::Format as PackedFormat;
