use tensor_derive::{num_traits_zero_path, gen_group_path};
pub use tensor_derive::{PlainGroupWrapper};
use crate::{GenGroup, WrapperDeref, GroupWrapper};
pub use num_traits::Zero;

#[derive(/*WrapperDeref,*/ GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[gen_group_path(GenGroup)]
#[num_traits_zero_path(Zero)]
pub struct Group<T: GenGroup>(pub T);

#[derive(GroupWrapper)]
#[num_traits_zero_path(Zero)]
#[gen_group_path(GenGroup)]
struct MultiGroup<T: GenGroup> {
  a: (T, T),
  b: T,
  c: T,
  d: Group<T>,
  e: [T; qq()],
}
const fn qq() -> usize {
  5
}
struct NType(pub i32);
fn qqq() -> [NType; 4] {
  // let a = (5,).0;
  let a = [1,2,3].map(NType);
  let b = [4,5,6].map(NType);
  let b = &b;
  let a_it = a.into_iter();
  let b_it = b.into_iter();
  let mut ab_it = a_it.zip(b_it).map(|(a, b)| NType(a.0+b.0));
  [(); 4].map(|()| ab_it.next().unwrap())
}

fn ppp() {
  let w = [true, true ,false];
  // w.contains(x)
  // w.con
}

