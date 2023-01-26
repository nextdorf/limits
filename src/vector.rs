use std::marker::PhantomData;

use crate::{multivar::{MultiVar, Index, DualMultiVar}, func::Fct, num::{Zero, One}};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitVec<I>(pub I);


impl<I, X> Fct for (UnitVec<I>, PhantomData<X>) where I: Index + PartialEq, X: Zero + One {
  type X = I;
  type Y = X;

  fn eval_fct(&self, i: I) -> X {
    self.0.eval_fct(i)
  }
}


impl<I> UnitVec<I> where I: Index + PartialEq {
  pub fn eval_fct<X: Zero + One>(&self, i: I) -> X {
    if i == self.0 {
      X::one()
    } else {
      X::zero()
    }
  }

  pub fn if_eq_idx_then_fst_else_snd<'a, X>(&self, i: I, fst: &'a X, snd: &'a X) -> &'a X {
    if i == self.0 {
      &fst
    } else {
      &snd
    }
  }
}


impl<I, X> MultiVar for (UnitVec<I>, PhantomData<X>) where I: Index + PartialEq, X: Zero + One {
  type I = I;
  type X = X;

  fn elem_at_index(&self, i: I) -> X {
    self.0.eval_fct(i)
  }
}


// impl<Xs> DualMultiVar<UnitVec<Xs::I>> for Xs where Xs: MultiVar {
//   type DualI = Xs::I;

//   fn dot(&self, _: &DIdFct<Var<X>>) -> Xs::X {
//     self.0.clone()
//   }

//   fn try_as_dual(&self) -> DIdFct<Var<X>> {
//     DIdFct(PhantomData)
//   }
// }


