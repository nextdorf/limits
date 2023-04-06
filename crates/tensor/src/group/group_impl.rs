use num_complex::{Complex32, Complex64};

use crate::wrapper_deref;

use super::{GenGroup, Group};


wrapper_deref!(GenGroup, Group);


#[macro_export]
macro_rules! wrapped_group_impl {
  ($gen:tt, $w:tt) => (
    impl<T: $gen> std::ops::Add<$w<T>> for $w<T> {
      type Output = $w<T>;
      
      fn add(self, rhs: Self) -> $w<T> {
        $w(self.0.add(rhs.0))
      }
    }
    
    impl<T: $gen, R: AsRef<$w<T>>> std::ops::Add<R> for $w<T> {
      type Output = $w<T>;
      
      fn add(self, rhs: R) -> $w<T> {
        $w(self.0.add_ref(rhs.as_ref()))
      }
    }
    
    impl<T: $gen> std::ops::Add<$w<T>> for &$w<T> {
      type Output = $w<T>;
      
      fn add(self, rhs: $w<T>) -> $w<T> {
        $w(self.ref_add(rhs.0))
      }
    }

    impl<T: $gen, R: AsRef<$w<T>>> std::ops::Add<R> for &$w<T> {
      type Output = $w<T>;
      
      fn add(self, rhs: R) -> $w<T> {
        $w(self.ref_add_ref(rhs.as_ref()))
      }
    }


    impl<T: $gen> std::ops::AddAssign<$w<T>> for $w<T> {
      fn add_assign(&mut self, rhs: $w<T>) {
        self.0.add_assign(rhs.0)
      }
    }

    impl<T: $gen, R: AsRef<$w<T>>> std::ops::AddAssign<R> for $w<T> {
      fn add_assign(&mut self, rhs: R) {
        self.add_assign_ref(rhs.as_ref())
      }
    }

    impl<T: $gen> std::ops::SubAssign<$w<T>> for $w<T> {
      fn sub_assign(&mut self, rhs: $w<T>) {
        self.0.sub_assign(rhs.0)
      }
    }

    impl<T: $gen, R: AsRef<$w<T>>> std::ops::SubAssign<R> for $w<T> {
      fn sub_assign(&mut self, rhs: R) {
        self.sub_assign_ref(rhs.as_ref())
      }
    }
  );
}

wrapped_group_impl!(GenGroup, Group);



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

gen_group_impl! { i8 i16 i32 i64 i128 f32 f64 Complex32 Complex64 }

