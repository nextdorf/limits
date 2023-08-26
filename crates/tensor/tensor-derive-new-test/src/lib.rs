mod trait_impl;
mod test_abel_g_is_abelian;

use ::core::marker::PhantomData;
#[allow(unused_imports)]
use tensor_traits::{Zero, One, Inv, GenGroup, num_field::NumAdd};
pub use tensor_derive_new::*;
pub use trait_impl::SomeUnit;


#[derive(AbelGroupWrapper)]
#[unit_types(PhantomData<T>, SomeUnit)]
// #[derive_generic(NumAdd)]
#[zero_path(Zero)]
#[one_path(One)]
#[inv_path(Inv)]

pub struct AbelG<T>(pub i32, i8, PhantomData<T>, SomeUnit);
// pub struct G(pub i32, i8, ());



