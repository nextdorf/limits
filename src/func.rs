// pub mod var;
// pub mod id;
pub mod cst;
// pub mod trig;

// pub use var::*;

use std::marker::PhantomData;

use crate::Auto;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DotNablaOp<X>(pub X);


pub trait Fct<X, Y> {
  fn eval_fct(&self, x: X) -> Y;
}

pub trait DirDiffable<X, Y, DY = Y>: Fct<X, Y> {
  type DF: Fct<X, DY>;

  fn diff(&self, dir: X) -> Self::DF;
}


impl<X, Y, F> Fct<F, F::DF> for Auto<DotNablaOp<X>, Y> where F: DirDiffable<X, Y>, X: Clone {
  fn eval_fct(&self, f: F) -> F::DF {
    self.0.eval_fct(f)
  }
}

impl<X, Y, F> DirDiffable<F, F::DF> for Auto<DotNablaOp<X>, Y> where F: DirDiffable<X, Y>, F::DF: Clone, X: Clone {
  type DF = cst::CstFct<F::DF>;

  fn diff(&self, dir: F) -> Self::DF {
    cst::CstFct(self.eval_fct(dir))
  }
}

impl<X: Clone> DotNablaOp<X> {
  pub fn eval_fct<Y, F>(&self, f: F) -> F::DF where F: DirDiffable<X, Y> {
    f.diff(self.0.clone())
  }

  pub fn auto_fct<Y, F>(self) -> Auto<Self, Y> where F: DirDiffable<X, Y> {
    Auto::wrap(self)
  }
}

