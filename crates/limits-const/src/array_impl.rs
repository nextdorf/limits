use std::ops::{AddAssign, Mul, MulAssign};

use crate::{ops::{BinOp, Sum, Prod}, func::{Dim, Fct, MultiVar, TracableMultiVar, DualMultiVar, Diffable, MultiVarFromIndex, square_norm_default_impl}, num::{Zero, One, MulCommute}};

impl<const N: usize, X: Copy + ~const AddAssign + ~const Zero> const BinOp for Sum<[X; N]> {
  type I = Dim<N>;
  type X = X;

  fn neutral_elem() -> X {
    X::zero()
  }

  fn commutative() -> bool {
    true
  }

  fn eval_binop(self) -> X {
    let mut ret = Self::neutral_elem();
    let mut i = 0;
    let len = self.0.len();
    while i < len {
      ret += self.0[i];
      i += 1;
    }
    ret
  }
}

impl<const N: usize, X: Copy + ~const MulAssign + ~const One + MulCommute> const BinOp for Prod<[X; N]> {
  type I = Dim<N>;
  type X = X;

  fn neutral_elem() -> X {
    X::one()
  }

  fn commutative() -> bool {
    true
  }

  fn eval_binop(self) -> X {
    let mut ret = Self::neutral_elem();
    let mut i = 0;
    let len = self.0.len();
    while i < len {
      ret *= self.0[i];
      i += 1;
    }
    ret
  }
}

impl<const N: usize, X, Y, F> const Fct for Sum<[F; N]> where
  X: Copy, Y: Copy, F: ~const Fct<X = X, Y = Y>
{
  type X = F::X;
  type Y = Sum<[F::Y; N]>;

  fn eval_fct(&self, x: Self::X) -> Self::Y {
    let fs = &self.0;
    Sum(<[F::Y; N]>::new_from_index(&|Dim(i)| fs[i].eval_fct(x)))
  }
}

impl<const N: usize, X, Y, F> const Fct for Prod<[F; N]> where
  X: Copy, Y: Copy, F: ~const Fct<X = X, Y = Y>
{
  type X = F::X;
  type Y = Prod<[F::Y; N]>;

  fn eval_fct(&self, x: Self::X) -> Self::Y {
    let fs = &self.0;
    Prod(<[F::Y; N]>::new_from_index(&|Dim(i)| fs[i].eval_fct(x)))
  }
}


impl<const N: usize, F> const Diffable for Sum<[F; N]> where
  F: ~const Diffable, F::Args: Copy, F::Y: Copy, F::DFct: Copy, <F::DFct as Fct>::Y: Copy,
  <F::DF as MultiVar>::Mapped<Sum<[F::DFct; N]>>: ~const MultiVarFromIndex, <F::DF as MultiVar>::I: Copy,
  <F::Args as MultiVar>::I: Copy
{
  type Args = F::Args;
  type XVar = F::XVar;
  type DFct = Sum<[F::DFct; N]>;
  type DF = <F::DF as MultiVar>::Mapped<Self::DFct>;

  fn diff_fct(&self, var_j: <Self::Args as MultiVar>::I) -> Self::DF {
    let sum_fs = &self.0;
    let res = MultiVarFromIndex::new_from_index(&|k| {
      let diff_sum_fs = <[F::DFct; N]>::new_from_index(&|Dim(i)| {
        sum_fs[i].diff_fct(var_j).get_idx(k)
      });
      Sum(diff_sum_fs)
    });
    res
  }
}


impl<X, const N: usize> const MultiVar for [X; N] where X: Copy {
  type I = Dim<N>;
  type X = X;
  type Mapped<Y> = [Y; N] where Y: Copy;

  fn get_idx(&self, arg: Self::I) -> Self::X {
    self[arg.0]
  }

  fn map<Y: Copy>(self, map_fn: &impl ~const Fn(Self::X) -> Y) -> Self::Mapped<Y> {
    <[Y; N]>::new_from_index(&|Dim(i)| map_fn(self[i]))
  }
}

impl<X, const N: usize> const TracableMultiVar for [X; N] where X: Copy + ~const AddAssign + ~const Zero {
  fn trace_idxs(self) -> Self::X {
    let mut res = X::zero();
    let mut i = 1;
    let len = self.len();
    while i < len {
      res += self[i];
      i+=1;
    }
    res
  }
}

impl<X, const N: usize> const DualMultiVar for [X; N] where X: Copy + ~const Mul<Output = X> + ~const AddAssign + ~const Zero {
  type Dual = Self;

  fn dot_idxs(self, rhs: Self::Dual) -> Self::X {
    let mut res = X::zero();
    let mut i = 1;
    let len = self.len();
    while i < len {
      res += self[i] * rhs[i];
      i+=1;
    }
    res
  }

  fn as_dual(self) -> Self {
    self
  }

  fn dual_index(i: Dim<N>) -> Dim<N> {
    i
  }

  fn square_norm(self) -> Self::X {
    square_norm_default_impl(self)
  }
}

impl<X, const N: usize> const MultiVarFromIndex for [X; N] where X: Copy {
  fn new_from_index(init_fn: &impl ~const Fn(Self::I) -> Self::X) -> Self {
    if N == 0 {
      unsafe {*([].as_ptr())}
    } else {
      let mut res = [init_fn(Dim(0)); N];
      let mut i = 1;
      while i < N {
        res[i] = init_fn(Dim(i));
        i+=1;
      }
      res
    }
  }
}


impl<T: ~const Zero + Copy, const N: usize> const Zero for [T; N] {
  fn zero() -> Self {
    [T::zero(); N]
  }
}

impl<T: ~const One + Copy, const N: usize> const One for [T; N] {
  fn one() -> Self {
    [T::one(); N]
  }
}

