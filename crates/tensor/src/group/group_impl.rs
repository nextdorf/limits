use std::ops::{Add, AddAssign, SubAssign};

use num_complex::{Complex32, Complex64};

use crate::wrapper_deref;

use super::{GenGroup, Group};


wrapper_deref!(GenGroup, Group);


impl<T: GenGroup> Add<Group<T>> for Group<T> {
  type Output = Group<T>;

  fn add(self, rhs: Self) -> Group<T> {
    Group(self.0.add(rhs.0))
  }
}

impl<T: GenGroup, R: AsRef<Group<T>>> Add<R> for Group<T> {
  type Output = Group<T>;

  fn add(self, rhs: R) -> Group<T> {
    Group(self.0.add_ref(rhs.as_ref()))
  }
}

impl<T: GenGroup> Add<Group<T>> for &Group<T> {
  type Output = Group<T>;

  fn add(self, rhs: Group<T>) -> Group<T> {
    Group(self.ref_add(rhs.0))
  }
}

impl<T: GenGroup, R: AsRef<Group<T>>> Add<R> for &Group<T> {
  type Output = Group<T>;

  fn add(self, rhs: R) -> Group<T> {
    Group(self.ref_add_ref(rhs.as_ref()))
  }
}


impl<T: GenGroup> AddAssign<Group<T>> for Group<T> {
  fn add_assign(&mut self, rhs: Group<T>) {
    self.0.add_assign(rhs.0)
  }
}

impl<T: GenGroup, R: AsRef<Group<T>>> AddAssign<R> for Group<T> {
  fn add_assign(&mut self, rhs: R) {
    self.add_assign_ref(rhs.as_ref())
  }
}

impl<T: GenGroup> SubAssign<Group<T>> for Group<T> {
  fn sub_assign(&mut self, rhs: Group<T>) {
    self.0.sub_assign(rhs.0)
  }
}

impl<T: GenGroup, R: AsRef<Group<T>>> SubAssign<R> for Group<T> {
  fn sub_assign(&mut self, rhs: R) {
    self.sub_assign_ref(rhs.as_ref())
  }
}



#[macro_export]
macro_rules! group_impl {
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

group_impl! { i8 i16 i32 i64 i128 f32 f64 Complex32 Complex64 }

