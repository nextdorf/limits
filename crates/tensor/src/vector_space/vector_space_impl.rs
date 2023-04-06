use std::ops::{Mul, MulAssign};

use num_complex::{Complex32, Complex64};

use crate::{wrapper_deref, GenVectorSpace};

use super::VectorSpace;


wrapper_deref!(GenVectorSpace, VectorSpace);

impl<T: GenVectorSpace> Mul<T::Field> for VectorSpace<T> {
  type Output = VectorSpace<T>;

  fn mul(self, rhs: T::Field) -> VectorSpace<T> {
    VectorSpace(self.0 * rhs)
  }
}

impl<T: GenVectorSpace> MulAssign<T::Field> for VectorSpace<T> {
  fn mul_assign(&mut self, rhs: T::Field) {
    self.mul_scalar_assign(rhs)
  }
}


#[macro_export]
macro_rules! vector_space_impl {
  ($(($t:ty, $u:ty))*) => ($(
    impl GenVectorSpace for $t {
      type Field = $u;

      fn mul_scalar_assign(&mut self, rhs: Self::Field) {
        *self *= rhs
      }
    }
  )*)
}

vector_space_impl! { (f32, f32) (f64, f64) (Complex32, f32) (Complex64, f64) }



