// pub mod ops;
mod vector_space_impl;
use std::{ops::Mul, mem::replace};

use crate::{NumField, GenGroup};


pub trait GenVectorSpace: GenGroup + Mul<Self::Field, Output = Self> {
  type Field: NumField;

  fn mul_scalar_assign(&mut self, rhs: Self::Field) {
    *self = replace(self, Self::zero()).mul(rhs)
  }
}

pub struct VectorSpace<T: GenVectorSpace>(pub T);

