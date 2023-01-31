use std::{marker::PhantomData, ops::Mul};

use crate::{multivar::{MultiVar, DualMultiVar, Index}, num::{Zero, One}, vector::UnitVec};

use super::{Fct, DirDiffable, Var, cst::ZeroFct, Diffable};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdFct<X>(PhantomData<X>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdMatrix<I>(PhantomData<I>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DIdFct<X>(PhantomData<X>);


impl<X> Fct for IdFct<X> {
  type X = X;
  type Y = X;

  fn eval_fct(&self, x: X) -> X {
    x
  }
}


impl<X> Diffable for IdFct<Var<X>> where X: Clone {
  type Args = Var<X>;
  type DF;

  fn diff(&self, _: ()) -> Self::DF {
    todo!()
  }
}


