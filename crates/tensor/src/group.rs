use tensor_derive::num_traits_export;
pub use tensor_derive::{PlainGroupWrapper};
use crate::{GenGroup, WrapperDeref, GroupWrapper};
pub use num_traits::Zero;

#[derive(/*WrapperDeref,*/ GroupWrapper, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[num_traits_export(Zero)]
pub struct Group<T: GenGroup>(pub T);


