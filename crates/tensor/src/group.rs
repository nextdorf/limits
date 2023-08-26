// pub mod cyclic;
// pub mod permutation;

use ::core::marker::PhantomData;

use crate::{AbelGroupWrapper, GenAbelGroup, GenGroup, GroupWrapper, WrapperDeref};
use crate::macros::{wrapper_deref, derive_generic};
pub use num_traits::{Inv, One, Zero};

// use tensor_derive_new::{gen_group_path, num_traits_inv_path, num_traits_one_path, wrap_deref};
// pub use cyclic::Cyclic;
// pub use permutation::Permutation;

pub use crate::macros::{
  inv_path,
  one_path,
  zero_path,
  neg_path,
  unit_types,
};


// #[derive(WrapperDeref, AbelGroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(AbelGroupWrapper, WrapperDeref)]
#[unit_types(PhantomData<Kind>)]
// #[derive_generic()]
#[zero_path(Zero)]
#[wrapper_deref]
// pub struct AbelGroup<Kind, T>(pub T, PhantomData<Kind>) where T: GenAbelGroup<Kind>;
pub struct AbelGroup<Kind, T>(pub i32, PhantomData<(T, Kind)>) where T: GenAbelGroup<Kind>;


// #[derive(WrapperDeref, GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// // #[derive(WrapperDeref)]
// #[gen_group_path(GenGroup)]
// #[num_traits_one_path(One)]
// #[num_traits_inv_path(Inv)]
// #[wrap_deref]
// pub struct Group<Kind, T: GenGroup<Kind>>(pub T, PhantomData<Kind>);


// #[derive(WrapperDeref, GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[derive(WrapperDeref)]
/*
#[gen_group_path(GenGroup)]
#[num_traits_one_path(One)]
#[num_traits_inv_path(Inv)]
struct NGroup<const N:usize, T: GenGroup>(pub [T; N]);
*/

// struct NType(pub i32);
// fn _qqq() -> [NType; 4] {
//   // let a = (5,).0;
//   let a = [1,2,3].map(NType);
//   let b = [4,5,6].map(NType);
//   let b = &b;
//   let a_it = a.into_iter();
//   let b_it = b.into_iter();
//   let mut ab_it = a_it.zip(b_it).map(|(a, b)| NType(a.0+b.0));
//   [(); 4].map(|()| ab_it.next().unwrap())
// }


// impl<T: GenGroup<K> + ToString, K> ToString for Group<K, T> {
//   fn to_string(&self) -> String {
//     self.deref().to_string()
//   }
// }

// impl<T: GenAbelGroup<K> + ToString, K> ToString for AbelGroup<K, T> {
//   fn to_string(&self) -> String {
//     self.0.to_string()
//   }
// }

/*
#[cfg(test)]
mod tests {
  use std::fmt::Debug;
  use num_traits::Zero;
  use tensor_traits::num_field::NumAdd;

  use tensor_derive::{
    gen_abel_group_path,
    num_traits_zero_path,
    gen_group_path,
    // num_traits_one_path,
    // num_traits_inv_path,
    AbelGroupWrapper,
  };


  #[derive(AbelGroupWrapper, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
  #[num_traits_zero_path(Zero)]
  #[gen_abel_group_path(super::GenAbelGroup)]
  struct MultiGroup<T: super::GenAbelGroup<NumAdd>> {
    a: (T, T),
    b: T,
    c: T,
    d: super::AbelGroup<NumAdd, T>,
    e: [T; qq()],
  }

  const fn qq() -> usize { 5 }
  
  #[derive(AbelGroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[gen_abel_group_path(super::GenAbelGroup)]
  #[gen_group_path(super::GenGroup)]
  #[num_traits_zero_path(super::Zero)]
  // #[num_traits_one_path(super::One)]
  // #[num_traits_inv_path(super::Inv)]
  
  pub struct G {
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub i128: i128,
  }

  #[test]
  fn abel_0() {
    assert_eq!(G::zero(), 0.into())
  }

  #[test]
  fn multi_0() {
    assert_eq!(MultiGroup::<i8>::zero(), MultiGroup { a: (0, 0), b: 0, c: 0, d: super::AbelGroup::new(0), e: [0; qq()] });
    assert_eq!(MultiGroup::<i16>::zero(), MultiGroup { a: (0, 0), b: 0, c: 0, d: super::AbelGroup::new(0), e: [0; qq()] });
    assert_eq!(MultiGroup::<i32>::zero(), MultiGroup { a: (0, 0), b: 0, c: 0, d: super::AbelGroup::new(0), e: [0; qq()] });
    assert_eq!(MultiGroup::<i64>::zero(), MultiGroup { a: (0, 0), b: 0, c: 0, d: super::AbelGroup::new(0), e: [0; qq()] });
    assert_eq!(MultiGroup::<i128>::zero(), MultiGroup { a: (0, 0), b: 0, c: 0, d: super::AbelGroup::new(0), e: [0; qq()] });
    assert_eq!(MultiGroup::<f32>::zero(), MultiGroup { a: (0., 0.), b: 0., c: 0., d: super::AbelGroup::new(0.), e: [0.; qq()] });
    assert_eq!(MultiGroup::<f64>::zero(), MultiGroup { a: (0., 0.), b: 0., c: 0., d: super::AbelGroup::new(0.), e: [0.; qq()] });
  }

  #[test]
  fn abel_add() {
    let a = G::new(12, 34, -4, 22, 512);
    let b = G::new(-5, -44, 2, 1, 133);

    let a_add_b = a + b;
    let b_add_a = b + a;
    let a_sub_b = a - b;
    let b_sub_a = b - a;
    let neg_a_add_neg_b = (-a) + (-b);
    let neg_a_add_b = -(a_add_b);
    let neg_a_sub_b = -(a_sub_b);
    let neg_b_sub_a = -(b_sub_a);
    let neg_neg_a = -(-a);
    let a_add_0 = a + G::zero();
    let a_sub_0 = a - G::zero();
    let a_sub_a = a - a;

    assert_eq!(a_add_b.values(), (a.i8+b.i8, a.i16+b.i16, a.i32+b.i32, a.i64+b.i64, a.i128+b.i128));
    assert_eq!(a_add_b.values(), b_add_a.values());

    assert_eq!(a_sub_b.values(), (a.i8-b.i8, a.i16-b.i16, a.i32-b.i32, a.i64-b.i64, a.i128-b.i128));
    assert_eq!(b_sub_a.values(), (b.i8-a.i8, b.i16-a.i16, b.i32-a.i32, b.i64-a.i64, b.i128-a.i128));
    assert_eq!(b_sub_a.values(), neg_a_sub_b.values());
    assert_eq!(a_sub_b.values(), neg_b_sub_a.values());
    assert_eq!(neg_a_add_neg_b.values(), neg_a_add_b.values());
    assert_eq!(a.values(), neg_neg_a.values());

    assert_eq!(a_add_0.values(), a.values());
    assert_eq!(a_sub_0.values(), a.values());
    assert_eq!(a_sub_a.values(), G::zero().values());
  }

  #[test]
  fn multi_add() {
    fn inner<T: super::GenAbelGroup<NumAdd> + From<i8> + PartialEq + Debug + Clone>(x: &MultiGroup<i8>, y: &MultiGroup<i8>) {
      let a: MultiGroup<T> = x.clone().into();
      let b: MultiGroup<T> = y.clone().into();

      let g0 = MultiGroup::zero();

      assert_eq!(&a + &b, &b + &a);
      assert_eq!(&a - &b, -(&b - &a));
      assert_eq!((-&a) + (-&b), -(&a + &b));
      assert_eq!(a.clone(), -(-&a));
      assert_eq!(&a + g0.clone(), a.clone());
      assert_eq!(&a - g0.clone(), a.clone());
      assert_eq!(&a - &a, g0.clone());
    }

    let a = MultiGroup {
      a: (-7, 9),
      b: 42,
      c: 18,
      d: super::AbelGroup::new(-1),
      e: [4, 2, -6, 0, 77],
    };
    let b = MultiGroup {
      a: (22, 42),
      b: -13,
      c: -64,
      d: super::AbelGroup::new(66),
      e: [50, 49, 48, 47, 46],
    };

    inner::<i8>(&a, &b);
    inner::<i16>(&a, &b);
    inner::<i32>(&a, &b);
    inner::<i64>(&a, &b);
    inner::<i128>(&a, &b);
    inner::<f32>(&a, &b);
    inner::<f64>(&a, &b);
  }


  impl G {
    pub const fn new(i8: i8, i16: i16, i32: i32, i64: i64, i128: i128) -> Self{
      Self { i8, i16, i32, i64, i128 }
    }

    pub const fn new8(i8: i8) -> Self{
      Self::new(i8, i8 as _, i8 as _, i8 as _, i8 as _)
    }

    pub const fn new0() -> Self{
      Self::new8(0)
    }

    pub const fn values(&self) -> (i8, i16, i32, i64, i128) {
      let Self { i8, i16, i32, i64, i128 } = *self;
      (i8, i16, i32, i64, i128)
    }
  }

  impl<T: super::GenAbelGroup<NumAdd>> MultiGroup<T> {
    pub fn into<U: super::GenAbelGroup<NumAdd>>(self) -> MultiGroup<U> where T: Into<U> {
      MultiGroup::from(self)
    }
    pub fn from<U: super::GenAbelGroup<NumAdd> + Into<T>>(value: MultiGroup<U>) -> Self {
      Self {
        a: (value.a.0.into(), value.a.1.into()),
        b: value.b.into(),
        c: value.c.into(),
        d: super::AbelGroup::from(value.d.0),
        e: value.e.map(Into::into)
      }
    }
  }

  impl From<i8> for G {
    fn from(value: i8) -> Self {
      Self::new8(value)
    }
  }

  impl Default for G {
    fn default() -> Self {
      Self::new0()
    }
  }
}
*/
