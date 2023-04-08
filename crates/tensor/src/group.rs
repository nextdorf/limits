// mod group_impl;
// use std::{ops::{Add, Neg, Sub}, mem::replace};
// use num_traits::Zero;

// pub trait GenGroup: Zero + Add<Self, Output = Self> + Sub<Self, Output = Self> + Neg<Output = Self> {
//   fn ref_add(&self, rhs: Self) -> Self;
//   fn add_ref(self, rhs: &Self) -> Self;
//   fn ref_add_ref(&self, rhs: &Self) -> Self;
//   fn add_assign(&mut self, rhs: Self) {
//     *self = replace(self, Self::zero()).add(rhs)
//   }
//   fn add_assign_ref(&mut self, rhs: &Self) {
//     *self = replace(self, Self::zero()).add_ref(rhs)
//   }

//   fn ref_sub(&self, rhs: Self) -> Self;
//   fn sub_ref(self, rhs: &Self) -> Self;
//   fn ref_sub_ref(&self, rhs: &Self) -> Self;
//   fn sub_assign(&mut self, rhs: Self) {
//     *self = replace(self, Self::zero()).sub(rhs)
//   }
//   fn sub_assign_ref(&mut self, rhs: &Self) {
//     *self = replace(self, Self::zero()).sub_ref(rhs)
//   }

//   fn ref_neg(&self) -> Self;
// }


pub use tensor_derive::GroupWrapper;
use crate::{GenGroup, WrapperDeref};

#[derive(WrapperDeref, GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<T: GenGroup>(pub T);


// impl<T: GenGroup> std::ops::Deref for Group<T> {
//   type Target = T;

//   fn deref(&self) -> &T {
//     &self.0
//   }
// }

// impl<T: GenGroup> std::ops::DerefMut for Group<T> {
//   fn deref_mut(&mut self) -> &mut T {
//     &mut self.0
//   }
// }


