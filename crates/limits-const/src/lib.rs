#![feature(
  const_trait_impl,
  const_mut_refs,
  const_refs_to_cell,
  // generic_const_exprs,
  // adt_const_params,
)]

pub mod func;
pub mod ops;
pub mod num;

mod array_impl;
pub use array_impl::*;



