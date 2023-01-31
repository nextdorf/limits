use std::{marker::PhantomData};

use crate::{num::{Zero, One}, Auto};

use super::Fct;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CstFct<X>(pub X);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroFct;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneFct;


impl<X, Y> Fct<X, Y> for CstFct<Y> where Y: Clone {
  fn eval_fct(&self, _: X) -> Y {
    self.eval_fct()
  }
}

impl<X: Clone> CstFct<X> {
  pub fn eval_fct(&self) -> X {
    self.0.clone()
  }

  pub fn auto_fct(self) -> Self {
    self
  }
}

impl<X, Y> Fct<X, Y> for ZeroFct where Y: Zero {
  fn eval_fct(&self, _: X) -> Y {
    self.eval_fct()
  }
}

impl ZeroFct {
  pub fn eval_fct<X: Zero>(&self) -> X {
    X::zero()
  }
}


impl<X, Y> Fct<X, Y> for OneFct where Y: One {
  fn eval_fct(&self, _: X) -> Y {
    self.eval_fct()
  }
}

impl OneFct {
  pub fn eval_fct<X: One>(&self) -> X {
    X::one()
  }
}


