use std::{marker::PhantomData};

use crate::num::{Zero, One};

use super::Fct;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CstFct<X>(pub X);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroFct;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneFct;


impl<X, Y> Fct for (CstFct<Y>, PhantomData<X>) where Y: Clone {
  type X = X;
  type Y = Y;

  fn eval_fct(&self, _: X) -> Y {
    self.0.eval_fct()
  }
}

impl<X: Clone> CstFct<X> {
  pub fn eval_fct(&self) -> X {
    self.0.clone()
  }
}

impl<X, Y> Fct for (ZeroFct, PhantomData<(X, Y)>) where Y: Zero {
  type X = X;
  type Y = Y;

  fn eval_fct(&self, _: X) -> Y {
    self.0.eval_fct()
  }
}

impl ZeroFct {
  pub fn eval_fct<X: Zero>(&self) -> X {
    X::zero()
  }
}


impl<X, Y> Fct for (OneFct, PhantomData<(X, Y)>) where Y: One {
  type X = X;
  type Y = Y;

  fn eval_fct(&self, _: X) -> Y {
    self.0.eval_fct()
  }
}

impl OneFct {
  pub fn eval_fct<X: One>(&self) -> X {
    X::one()
  }
}


