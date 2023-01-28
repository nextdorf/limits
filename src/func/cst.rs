use std::{marker::PhantomData};

use crate::{num::{Zero, One}, Auto};

use super::Fct;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CstFct<X>(pub X);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroFct;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneFct;


impl<X, Y> Fct for Auto<CstFct<Y>, X> where Y: Clone {
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

  pub fn auto_fct<XIn>(self) -> Auto<Self, XIn> {
    Auto::wrap(self)
  }
}

impl<X, Y> Fct for Auto<ZeroFct, (X, Y)> where Y: Zero {
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

  pub fn auto_fct<X, Y: Zero>(self) -> Auto<Self, (X, Y)> {
    Auto::wrap(self)
  }
}


impl<X, Y> Fct for Auto<OneFct, (X, Y)> where Y: One {
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

  pub fn auto_fct<X, Y: One>(self) -> Auto<Self, (X, Y)> {
    Auto::wrap(self)
  }
}


