use crate::{
  GenAbelGroup,
  GenGroup, num_field::{NumAdd, NumMul}
};

#[macro_export]
macro_rules! gen_abelian_group_impl {
  ($u:ty: $($t:ty)*) => ($(
    impl GenGroup<$u> for $t {
      fn mult(self, rhs: Self) -> Self { self + rhs }
      fn mult_ref(self, rhs: &Self) -> Self { self + rhs }
      fn ref_mult(&self, rhs: Self) -> Self { self + rhs }
      fn ref_mult_ref(&self, rhs: &Self) -> Self { self + rhs }
      fn mult_assign(&mut self, rhs: Self) { *self += rhs }
      fn mult_assign_ref(&mut self, rhs: &Self) { *self += rhs }
    
      fn mult_inv(self, rhs: Self) -> Self { self - rhs }
      fn mult_inv_ref(self, rhs: &Self) -> Self { self - rhs }
      fn ref_mult_inv(&self, rhs: Self) -> Self { self - rhs }
      fn ref_mult_inv_ref(&self, rhs: &Self) -> Self { self - rhs }
      fn mult_assign_inv(&mut self, rhs: Self) { *self -= rhs }
      fn mult_assign_inv_ref(&mut self, rhs: &Self) { *self -= rhs }
    
      fn inv(self) -> Self { -self }
      fn ref_inv(&self) -> Self { -self }
    
      fn unit() -> Self { num_traits::zero() }
      fn is_unit(&self) -> bool { num_traits::Zero::is_zero(self) }
      fn set_unit(&mut self) { num_traits::Zero::set_zero(self) }
    }

    impl GenAbelGroup<$u> for $t {}
  )*)
}

#[macro_export]
macro_rules! gen_mul_group_impl {
  ($u:ty: $($t:ty)*) => ($(
    impl GenGroup<$u> for $t {
      fn mult(self, rhs: Self) -> Self { self * rhs }
      fn mult_ref(self, rhs: &Self) -> Self { self * rhs }
      fn ref_mult(&self, rhs: Self) -> Self { self * rhs }
      fn ref_mult_ref(&self, rhs: &Self) -> Self { self * rhs }
      fn mult_assign(&mut self, rhs: Self) { *self *= rhs }
      fn mult_assign_ref(&mut self, rhs: &Self) { *self *= rhs }
    
      fn mult_inv(self, rhs: Self) -> Self { self / rhs }
      fn mult_inv_ref(self, rhs: &Self) -> Self { self / rhs }
      fn ref_mult_inv(&self, rhs: Self) -> Self { self / rhs }
      fn ref_mult_inv_ref(&self, rhs: &Self) -> Self { self / rhs }
      fn mult_assign_inv(&mut self, rhs: Self) { *self /= rhs }
      fn mult_assign_inv_ref(&mut self, rhs: &Self) { *self /= rhs }
    
      fn inv(self) -> Self { num_traits::Inv::inv(self) }
      fn ref_inv(&self) -> Self { num_traits::Inv::inv(self) }
    
      fn unit() -> Self { num_traits::one() }
      fn is_unit(&self) -> bool { num_traits::One::is_one(self) }
      fn set_unit(&mut self) { num_traits::One::set_one(self) }
    }
  )*)
}

#[macro_export]
macro_rules! unit_group_impl {
  ($u:ty: $($t:ty)*) => ($(
    impl GenGroup<$u> for $t {
      fn mult(self, _: Self) -> Self { Self }
      fn mult_ref(self, _: &Self) -> Self { Self }
      fn ref_mult(&self, _: Self) -> Self { Self }
      fn ref_mult_ref(&self, _: &Self) -> Self { Self }
      fn mult_assign(&mut self, _: Self) { }
      fn mult_assign_ref(&mut self, _: &Self) { }
    
      fn mult_inv(self, _: Self) -> Self { Self }
      fn mult_inv_ref(self, _: &Self) -> Self { Self }
      fn ref_mult_inv(&self, _: Self) -> Self { Self }
      fn ref_mult_inv_ref(&self, _: &Self) -> Self { Self }
      fn mult_assign_inv(&mut self, _: Self) { }
      fn mult_assign_inv_ref(&mut self, _: &Self) { }
    
      fn inv(self) -> Self { Self }
      fn ref_inv(&self) -> Self { Self }
    
      fn unit() -> Self { Self }
      fn is_unit(&self) -> bool { true }
      fn set_unit(&mut self) { }
    }
  )*);
  (generics<$($v:ident),*> $u:ty: $t:ty) => (
    impl<$($v),*> GenGroup<$u> for $t {
      fn mult(self, _: Self) -> Self { Self }
      fn mult_ref(self, _: &Self) -> Self { Self }
      fn ref_mult(&self, _: Self) -> Self { Self }
      fn ref_mult_ref(&self, _: &Self) -> Self { Self }
      fn mult_assign(&mut self, _: Self) { }
      fn mult_assign_ref(&mut self, _: &Self) { }
    
      fn mult_inv(self, _: Self) -> Self { Self }
      fn mult_inv_ref(self, _: &Self) -> Self { Self }
      fn ref_mult_inv(&self, _: Self) -> Self { Self }
      fn ref_mult_inv_ref(&self, _: &Self) -> Self { Self }
      fn mult_assign_inv(&mut self, _: Self) { }
      fn mult_assign_inv_ref(&mut self, _: &Self) { }
    
      fn inv(self) -> Self { Self }
      fn ref_inv(&self) -> Self { Self }
    
      fn unit() -> Self { Self }
      fn is_unit(&self) -> bool { true }
      fn set_unit(&mut self) { }
    }
  );
  ($u:ty) => (
    impl GenGroup<$u> for () {
      fn mult(self, _: Self) -> Self { }
      fn mult_ref(self, _: &Self) -> Self { }
      fn ref_mult(&self, _: Self) -> Self { }
      fn ref_mult_ref(&self, _: &Self) -> Self { }
      fn mult_assign(&mut self, _: Self) { }
      fn mult_assign_ref(&mut self, _: &Self) { }
    
      fn mult_inv(self, _: Self) -> Self { }
      fn mult_inv_ref(self, _: &Self) -> Self { }
      fn ref_mult_inv(&self, _: Self) -> Self { }
      fn ref_mult_inv_ref(&self, _: &Self) -> Self { }
      fn mult_assign_inv(&mut self, _: Self) { }
      fn mult_assign_inv_ref(&mut self, _: &Self) { }
    
      fn inv(self) -> Self { }
      fn ref_inv(&self) -> Self { }
    
      fn unit() -> Self { }
      fn is_unit(&self) -> bool { true }
      fn set_unit(&mut self) { }
    }
  );
}


gen_abelian_group_impl! { NumAdd: i8 i16 i32 i64 i128 f32 f64 }
gen_mul_group_impl! { NumMul: f32 f64 }

// unit_group_impl! {(): ::core::marker::PhantomData<()>}
// unit_group_impl! {generics<T> (): ::core::marker::PhantomData<T>}
unit_group_impl! {()}

#[cfg(feature = "complex")]
gen_abelian_group_impl! { NumAdd: num_complex::Complex32 num_complex::Complex64 }
#[cfg(feature = "complex")]
gen_mul_group_impl! { NumMul: num_complex::Complex32 num_complex::Complex64 }


