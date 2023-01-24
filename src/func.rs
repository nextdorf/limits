pub mod cst;
pub mod id;

use std::ops::Mul;

use crate::num::{One, Zero};


pub trait Index { }

#[const_trait]
pub trait MultiVar: Clone {
  type I: Index;
  type X;
  type Mapped<Y>: MultiVar<I = Self::I, X = Y> where Y: Copy;

  fn get_idx(&self, arg: Self::I) -> Self::X;

  // fn map<Y: Copy>(self, map_fn: fn(Self::X) -> Y) -> Self::Mapped<Y>;
  fn map<Y: Copy>(self, map_fn: &impl Fn(Self::X) -> Y) -> Self::Mapped<Y>;
}

#[const_trait]
pub trait TracableMultiVar: MultiVar {
  fn trace_idxs(self) -> Self::X;
}

#[const_trait]
pub trait DualMultiVar: MultiVar {
  type Dual: DualMultiVar<Dual = Self, X = Self::X>;

  fn dot_idxs(self, rhs: Self::Dual) -> Self::X;
}

#[const_trait]
pub trait MultiVarFromIndex: MultiVar {
  fn new_from_index(init_fn: &impl Fn(Self::I) -> Self::X) -> Self;
}

#[const_trait]
pub trait NormedSpace: DualMultiVar where Self::Dual: ~const NormedSpace {
  fn unit_vector(i: Self::I) -> Self;
  fn unit_dual_vector(i: <Self::Dual as MultiVar>::I) -> Self::Dual {
    <Self::Dual as NormedSpace>::unit_vector(i)
  }
}


/// Function X -> Y
#[const_trait]
pub trait Fct {
  type X;
  type Y;
  fn eval_fct(&self, x: Self::X) -> Self::Y;
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


pub struct Dim<const N: usize>(pub usize);

impl<const N: usize> Index for Dim<N> { }


#[derive(Debug, Clone, Copy)]
pub struct Var<X>(pub X);

impl Index for () {}

impl<X> const MultiVar for Var<X> where X: Copy {
  type I = ();
  type X = X;
  type Mapped<Y> = Var<Y> where Y: Copy;

  fn get_idx(&self, _: Self::I) -> Self::X {
    self.0
  }

  fn map<Y: Copy>(self, map_fn: &impl ~const Fn(Self::X) -> Y) -> Self::Mapped<Y> {
    Var(map_fn(self.0))
  }
}
impl<X> const TracableMultiVar for Var<X> where X: Copy {
  fn trace_idxs(self) -> Self::X {
    self.0
  }
}
impl<X> const DualMultiVar for Var<X> where X: ~const Mul<Output = X> + Copy {
  type Dual = Self;

  fn dot_idxs(self, rhs: Self::Dual) -> Self::X {
    self.0 * rhs.0
  }
}
impl<X> const NormedSpace for Var<X> where X: ~const Mul<Output = X> + ~const One + Copy {
  fn unit_vector(_: Self::I) -> Self {
    Self(X::one())
  }
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


