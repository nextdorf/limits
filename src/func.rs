pub mod var;
pub mod id;
pub mod cst;

pub use var::*;

use crate::multivar::{MultiVar, DualMultiVar};

pub trait Fct {
  type X;
  type Y;

  fn eval_fct(&self, x: Self::X) -> Self::Y;
}

pub trait Diffable<DF>: Fct<X = Self::Args> where
DF: MultiVar<I = <Self::Xs as DualMultiVar<DF>>::DualI, X = Self::DFct> {
  type Args: MultiVar<X = Self::Xs>;
  type Xs: DualMultiVar<DF>;
  type DFct: Fct<X = Self::Args, Y = Self::Y>;
  // type DF: MultiVar<I = <Self::Xs as DualMultiVar<DualXs>>::DualI, X = Self::DFct>;

  fn diff(&self, var_i: <Self::Xs as MultiVar>::I) -> DF;

  fn dot_diff(&self, var_i: <Self::Xs as MultiVar>::I, dir: &Self::Xs) -> <Self::Xs as MultiVar>::X {
    dir.dot(&self.diff(var_i))
  }
}

// pub fn dot_diff<D, Xs>(f: &D, var_i: <D::Xs as MultiVar>::I, dir: &Xs) -> Xs::X where
// D: Diffable, D::DF: MultiVar, Xs: DualMultiVar<D::DF> {
//   dir.dot(&f.diff(var_i))
// }

