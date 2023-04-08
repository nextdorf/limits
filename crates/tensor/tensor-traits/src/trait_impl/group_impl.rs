use crate::GenGroup;

#[macro_export]
macro_rules! gen_group_impl {
  ($($t:ty)*) => ($(
    impl GenGroup for $t {
      fn ref_add(&self, rhs: Self) -> Self {
        self + rhs
      }
      fn add_ref(self, rhs: &Self) -> Self {
        self + rhs
      }
      fn ref_add_ref(&self, rhs: &Self) -> Self {
        self + rhs
      }
      fn add_assign(&mut self, rhs: Self) {
        *self += rhs
      }
      fn add_assign_ref(&mut self, rhs: &Self) {
        *self += rhs
      }

      fn ref_sub(&self, rhs: Self) -> Self{
        self - rhs
      }
      fn sub_ref(self, rhs: &Self) -> Self{
        self - rhs
      }
      fn ref_sub_ref(&self, rhs: &Self) -> Self{
        self - rhs
      }
      fn sub_assign(&mut self, rhs: Self) {
        *self -= rhs
      }
      fn sub_assign_ref(&mut self, rhs: &Self) {
        *self -= rhs
      }
    
      fn ref_neg(&self) -> Self {
        -self
      }
    }
  )*)
}

gen_group_impl! { i8 i16 i32 i64 i128 f32 f64 }

#[cfg(feature = "complex")]
gen_group_impl! { num_complex::Complex32 num_complex::Complex64 }

