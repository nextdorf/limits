use std::marker::PhantomData;

use crate::{multivar::{MultiVar, DualMultiVar}, num::Zero};

use super::{Fct, Diffable, Var, cst::ZeroFct};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdFct<X>(PhantomData<X>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DIdFct<X>(PhantomData<X>);


impl<X> Fct for IdFct<X> {
  type X = X;
  type Y = X;

  fn eval_fct(&self, x: X) -> X {
    x
  }
}


impl<X> Diffable<DIdFct<Var<X>>> for IdFct<Var<Var<X>>> where X: Clone + Zero {
  type Args = Var<Self::Xs>;
  type Xs = Var<X>;
  type DFct = (ZeroFct, PhantomData<(Var<Var<X>>, Var<Var<X>>)>);

  fn diff(&self, _: ()) -> DIdFct<Var<X>> {
    DIdFct(PhantomData)
  }
}


impl<X> IdFct<X> where X: Clone + Zero {
  pub fn diff(&self) -> DIdFct<Var<X>> {
    DIdFct(PhantomData)
  }

  pub fn dot_diff(&self, dir: &Var<X>) -> X {
    dir.dot(&self.diff())
  }
}

impl<Xs> MultiVar for DIdFct<Xs> where Xs: MultiVar {
  type I = Xs::I;
  // type X = ZeroFct;
  type X = (ZeroFct, PhantomData<(Var<Xs>, Var<Xs>)>);

  fn elem_at_index(&self, _: Xs::I) -> &(ZeroFct, PhantomData<(Var<Xs>, Var<Xs>)>) {
    &(ZeroFct, PhantomData)
  }
}

impl<X> DualMultiVar<DIdFct<Var<X>>> for Var<X> where X: Clone {
  type DualI = ();

  fn dot(&self, _: &DIdFct<Var<X>>) -> Self::X {
    self.0.clone()
  }

  fn as_dual(self) -> DIdFct<Var<X>> {
    DIdFct(PhantomData)
  }
}


