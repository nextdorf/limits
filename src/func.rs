pub mod f64;
pub mod ops;
mod func_impl;
mod blankets;

use std::ops::{AddAssign, MulAssign};

pub use blankets::*;
pub use func_impl::*;

use crate::num::Zero;


pub trait Index { }

// T: ~const AddAssign + ~const MulAssign + Copy + ~const Zero
#[const_trait]
pub trait MultiVar: Clone {
  type I: Index;
  type X: AddAssign<Self::X> + MulAssign<Self::X> + Copy + Zero;
  type Mapped<Y>: MultiVar<I = Self::I, X = Y> where Y: AddAssign + MulAssign + Copy + Zero;

  fn get_idx(&self, args: Self::I) -> Self::X;

  fn trace_idxs(self) -> Self::X;

  fn mult_idxs(self, rhs: Self) -> Self;

  // fn mult_many(self, rhs: impl ~const IntoIterator<Item = Self>) -> Self {
  //   let mut res = self;
  //   for r in rhs {
  //     res = res.mult(r);
  //   }
  //   res
  // }

  fn dot_idxs(self, rhs: Self) -> Self::X {
    self.mult_idxs(rhs).trace_idxs()
  }

  fn map<Y>(self, map_fn: fn(Self::X) -> Y) -> Self::Mapped<Y> where Y: AddAssign + MulAssign + Copy + Zero;
}

/// Differentiable function \
/// F = F: (x1_I, x2_I, ..)_J ↦ Y \
/// diff F = (diff F)_I: (x1_I, x2_I, ..)_J ↦ Y'
#[const_trait]
pub trait Diffable: Fct<X = Self::Args> {
  type Args: MultiVar<X = Self::XVar>;
  type XVar: MultiVar;
  type DFct: Fct<X = Self::Args>;
  type DF: MultiVar<I = <Self::XVar as MultiVar>::I, X = Self::DFct>;

  fn diff_fct(&self, var_i: <Self::Args as MultiVar>::I) -> Self::DF;
}


/// Function X -> Y
#[const_trait]
pub trait Fct {
  type X;
  type Y;
  fn eval_fct(&self, x: Self::X) -> Self::Y;
}


/// Function composition FctComp(G, Y, F): X --(F)-> Y --(G)-> Z
pub struct FctComp<G: Fct<X = Y>, Y, F: Fct<Y = Y>>(pub G, pub F);

impl<G: ~const Fct<X = Y>, Y, F: ~const Fct<Y = Y>> const Fct for FctComp<G, Y, F> {
  type X = F::X;
  type Y = G::Y;

  fn eval_fct(&self, x: Self::X) -> Self::Y {
    self.0.eval_fct(self.1.eval_fct(x))
  }
}


