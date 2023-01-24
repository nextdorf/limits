use std::marker::PhantomData;

use crate::num::Zero;

use super::{Fct, Var, Diffable, MultiVar};

#[derive(Clone, Copy)]
pub struct CstFct<X>(pub X);

// #[derive(Clone, Copy)]
// pub struct DIdFct<F>(PhantomData<F>);

impl<X> const Fct for CstFct<X> where X: Copy {
  type X = Var<Var<()>>;
  type Y = X;

  fn eval_fct(&self, _: Var<Var<()>>) -> X {
    self.0
  }
}

impl<X> const Diffable for CstFct<X> where X: Copy + ~const Zero {
  type Args = Var<Var<()>>;
  type XVar = Var<()>;
  type DFct = CstFct<X>;
  type DF = Var<Self::DFct>;

  fn diff_fct(&self, _: ()) -> Var<CstFct<X>> {
    Var(CstFct(X::zero()))
  }
}


