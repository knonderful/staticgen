//! A crate for generating static-compatible source code from serializable data.
//!
//! See [`Serializer`] for more information.

mod model;
mod ser;

pub use ser::Enums;
pub use ser::Serializer;
pub use ser::Structs;
