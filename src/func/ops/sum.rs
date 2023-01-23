use std::{string::ToString, fmt::Debug, ops::{AddAssign, MulAssign, Mul}};

use crate::{func::{MultiVar, Index, Fct, Diffable, Dim}, num::{Zero, One}};

use super::BinOp;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Sum<Xs>(pub Xs);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Prod<Xs>(pub Xs);


impl<X: ~const Zero, Xs: ~const MultiVar<X = X> + Copy> const BinOp for Sum<Xs> {
  type I = Xs::I;
  type X = X;

  fn neutral_elem() -> X {
    X::zero()
  }

  fn commutative() -> bool {
    true
  }

  fn eval_binop(self) -> X {
    self.0.trace_idxs()
  }

}

// impl<I: Index, X, Y, F: Fct<X, Y>, Fs: ~const MultiVar<I, F> + Copy> const Fct<X, Y> for Sum<Fs> {
//   fn eval(&self, x: X) -> Y {
//       todo!()
//   }
// }

impl<const N: usize, X, Y, F> const Fct for Sum<[F; N]> where
  X: Copy, Y: ~const Default + Copy, F: ~const Fct<X = X, Y = Y>
{
  type X = F::X;
  type Y = Sum<[F::Y; N]>;

  fn eval_fct(&self, x: Self::X) -> Self::Y {
    let mut res = [Y::default(); N];
    let mut i = 0;
    while i < N {
      res[i] = self.0[i].eval_fct(x);
      i += 1;
    }
    Sum(res)
  }
}

// impl<Xs: AddAssign> AddAssign for Sum<Xs> {
//   fn add_assign(&mut self, rhs: Self) {
//     self.0 += rhs.0
//   }
// }

impl<X: AddAssign, const N: usize> AddAssign for Sum<[X; N]> {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0
  }
}

impl<Xs: MulAssign> MulAssign for Sum<Xs> {
  fn mul_assign(&mut self, rhs: Self) {
      todo!()
  }
}

impl<Xs: Zero> Zero for Sum<Xs> {
    fn zero() -> Self {
        todo!()
    }
}

// / diff F = (diff F)_I: (x1_I, x2_I, ..)_J ↦ Y
// / diff Sum[F | 1..N] = (diff Sum[F | 1..N])_I: Sum[(x1_I, x2_I, ..)_J ↦ Y | 1..N]
// impl<const N: usize, F> const Diffable for Sum<[F; N]> where
//   Self: Fct, F: Diffable, F::Args: Copy, F::Y: Default + Copy, <F::DFct as Fct>::Y: Default + Copy
// {
//     type Args = F::Args;
//     type XVar = F::XVar;
//     type DFct = Sum<[F::DFct; N]>;
//     type DF = <Self::XVar as MultiVar>::Mapped<Self::DFct>;

//     fn diff_fct(&self, var_i: <Self::Args as MultiVar>::I) -> Self::DF {
//         todo!()
//     }
// }

// impl<const N: usize, const D: usize, X, Y, F, DFct>
//   const Diffable<Dim<N>, X, [X; N], Sum<[Y; N]>, Dim<D>, F::DFct, [DFct; D]>
//   for Sum<[F; N]>
//   where X: Copy, Y: Copy + ~const Default, F: ~const Fct<X, Y> + ~const Diffable<>
// {
  // fn eval(&self, x: X) -> Sum<[Y; N]> {
  //   let mut res = [Y::default(); N];
  //   let mut i = 0;
  //   while i < N {
  //     res[i] = self.0[i].eval(x);
  //     i += 1;
  //   }
  //   Sum(res)
  // }
// }




