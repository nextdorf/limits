use crate::{multivar::{MultiVar, Index}, num::Zero};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var<X>(pub X);

impl Index for () {}


impl<X> MultiVar for Var<X> where X: Clone {
  type I = ();
  type X = X;

  fn elem_at_index(&self, _: ()) -> X {
    self.elem_at_index()
  }
}

impl<X: Zero> Zero for Var<X> {
  fn zero() -> Self {
    Self(X::zero())
  }
}

impl<X> Var<X> where X: Clone {
  pub fn elem_at_index(&self) -> X {
    self.0.clone()
  }
}




