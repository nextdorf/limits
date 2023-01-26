use crate::{multivar::{MultiVar, Index}, num::Zero};

use super::Fct;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var<X>(pub X);

impl Index for () {}


// impl<F> Fct for F where F: Fct {
//   type X = Var<F::X>;
//   type Y = F::Y;

//   fn eval_fct(&self, x: Var<F::X>) -> F::Y {
//     self.eval_fct(x.0)
//   }
// }

impl<X> MultiVar for Var<X> {
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

impl<X> Var<X> {
  pub fn elem_at_index(&self) -> X {
    self.0
  }
}




