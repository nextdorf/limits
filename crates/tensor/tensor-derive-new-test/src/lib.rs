use std::marker::PhantomData;
use tensor_traits::{Zero, One, GenGroup, num_field::NumAdd};
pub use tensor_derive_new::*;

// #[derive(WrapperDeref)]
// #[wrapper_deref(value)]
// pub struct A<T> {
//   internal: PhantomData<T>,
//   pub value: u32,
// }



// #[derive(WrapperDeref)]
// #[wrapper_deref]
// pub struct B(pub (), pub u32);


struct SomeUnit;

#[derive(GroupWrapper)]
#[unit_types(PhantomData<T>, SomeUnit)]
#[derive_generic(NumAdd)]
#[zero_path(Zero)]
#[one_path(One)]
pub struct G<T>(pub i32, i8, PhantomData<T>, SomeUnit);
// pub struct G(pub i32, i8, ());


// impl<T, B> ::core::ops::Mul<&B> for G<T> where B: ::core::borrow::Borrow<G<T>> {
//   type Output = G<T>;
//   #[inline]
//   fn mul(self, rhs: &B) -> G<T> {
//     G(self.0.mult_ref(&rhs.borrow().0), self.1.mult_ref(&rhs.borrow().1), PhantomData, SomeUnit)
//   }
// }

