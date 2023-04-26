use tensor_derive::{num_traits_zero_path, gen_abel_group_path, AbelGroupWrapper};
pub use tensor_derive::{PlainGroupWrapper};
use crate::{GenGroup, GenAbelGroup, WrapperDeref, GroupWrapper};
pub use num_traits::{Zero, One, Inv};

#[derive(/*WrapperDeref,*/ AbelGroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[gen_abel_group_path(GenAbelGroup)]
#[num_traits_zero_path(Zero)]
pub struct AbelGroup<T: GenAbelGroup>(pub T);

#[derive(AbelGroupWrapper)]
#[num_traits_zero_path(Zero)]
#[gen_abel_group_path(GenAbelGroup)]
struct MultiGroup<T: GenAbelGroup> {
  a: (T, T),
  b: T,
  c: T,
  d: AbelGroup<T>,
  e: [T; qq()],
}

// #[derive(WrapperDeref, GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[gen_group_path(GenGroup)]
// #[num_traits_zero_path(Zero)]
// struct NGroup<const N:usize, T: GenGroup>(pub [T; N]);

const fn qq() -> usize {
  5
}
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

fn ppp() {
  let [a, b, c] = [1i32,2,3].map(AbelGroup);
  let ab = a+b;

}

