mod finite;

pub use finite::FiniteVS;
use std::{ops::{Mul, Div}, mem::replace};
use crate::{NumField, GenGroup};


pub trait GenVectorSpace: GenGroup + Mul<Self::Field, Output = Self> + Div<Self::Field, Output = Self> {
  type Field: NumField;

  fn mul_scalar_assign(&mut self, rhs: Self::Field) {
    *self = replace(self, Self::zero()).mul(rhs)
  }

  fn div_scalar_assign(&mut self, rhs: Self::Field) {
    *self = replace(self, Self::zero()).div(rhs)
  }
}

