// pub mod var;
// pub mod id;
pub mod cst;

// pub use var::*;

use std::marker::PhantomData;

use crate::Auto;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DotNablaOp<X>(pub X);


pub trait Fct {
  type X;
  type Y;

  fn eval_fct(&self, x: Self::X) -> Self::Y;
}

pub trait DirDiffable: Fct {
  type DF: Fct<X = Self::X>;

  fn diff(&self, dir: Self::X) -> Self::DF;
}


impl<X, F> Fct for Auto<DotNablaOp<X>, F> where F: DirDiffable<X = X>, X: Clone {
  type X = F;
  type Y = F::DF;

  fn eval_fct(&self, f: F) -> F::DF {
    self.0.eval_fct(f)
  }
}

impl<X, F> DirDiffable for Auto<DotNablaOp<X>, F> where F: DirDiffable<X = X>, F::DF: Clone, X: Clone {
  type DF = Auto<cst::CstFct<F::DF>, F>;

  fn diff(&self, dir: F) -> Self::DF {
    cst::CstFct(self.0.eval_fct(dir)).auto_fct()
  }
}

impl<X: Clone> DotNablaOp<X> {
  pub fn eval_fct<F>(&self, f: F) -> F::DF where F: DirDiffable<X = X> {
    f.diff(self.0)
  }

  pub fn auto_fct<F>(self) -> Auto<Self, F> where F: DirDiffable<X = X> {
    Auto::wrap(self)
  }
}

