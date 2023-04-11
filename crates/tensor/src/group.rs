use tensor_derive::{num_traits_zero_path, gen_group_path};
pub use tensor_derive::{PlainGroupWrapper};
use crate::{GenGroup, WrapperDeref, GroupWrapper};
pub use num_traits::Zero;

// #[derive(/*WrapperDeref,*/ GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[num_traits_zero_path(Zero)]
// pub struct Group<T: GenGroup>(pub T);

#[derive(GroupWrapper)]
// #[num_traits_zero_path(Zero)]
#[gen_group_path(GenGroup)]
struct MultiGroup<T: GenGroup> {
  a: (T, T),
  b: T,
  c: T,
}



