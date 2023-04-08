use crate::GenVectorSpace;

#[macro_export]
macro_rules! vector_space_impl {
  ($(($t:ty, $u:ty))*) => ($(
    impl GenVectorSpace for $t {
      type Field = $u;

      fn mul_scalar_assign(&mut self, rhs: Self::Field) {
        *self *= rhs
      }

      fn div_scalar_assign(&mut self, rhs: Self::Field) {
        *self /= rhs
      }
    }
  )*)
}

vector_space_impl! { (f32, f32) (f64, f64) }

#[cfg(feature = "complex")]
vector_space_impl! { (num_complex::Complex32, f32) (num_complex::Complex64, f64) }


