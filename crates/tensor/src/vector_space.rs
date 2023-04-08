mod finite;
mod vector_space_impl;

pub use finite::*;
use std::{ops::{Mul, Div}, mem::replace};
use crate::{NumField, GenGroup, wrapped_group_impl};


pub trait GenVectorSpace: GenGroup + Mul<Self::Field, Output = Self> + Div<Self::Field, Output = Self> {
  type Field: NumField;

  fn mul_scalar_assign(&mut self, rhs: Self::Field) {
    *self = replace(self, Self::zero()).mul(rhs)
  }

  fn div_scalar_assign(&mut self, rhs: Self::Field) {
    *self = replace(self, Self::zero()).div(rhs)
  }
}

pub struct VectorSpace<T: GenVectorSpace>(pub T);


wrapped_group_impl!(GenVectorSpace, VectorSpace);

impl<T: GenVectorSpace> From<T> for VectorSpace<T> {
  fn from(value: T) -> Self {
    Self(value)
  }
}
