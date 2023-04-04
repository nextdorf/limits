mod num_field;
mod field_impl;
pub mod ops;


pub use num_field::*;

use num_traits::Zero;

use ops::VectorSpaceFullOps;


pub trait VectorSpace: Zero + VectorSpaceFullOps<Self::Field> {
  type Field: NumField;
}


