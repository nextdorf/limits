use std::ops::{AddAssign, MulAssign};

use crate::num::{Zero, One};

use super::{MultiVar, Index};

pub struct Dim<const N: usize>(pub usize);

impl<const N: usize> super::Index for Dim<N> { }


impl<T, const N: usize> const MultiVar for [T; N] where T: ~const AddAssign + ~const MulAssign + Copy + Zero {
  type I = Dim<N>;
  type X = T;
  type Mapped<Y> = [Y; N] where Y: AddAssign + MulAssign + Copy + Zero;
  
  fn get_idx(&self, args: Dim<N>) -> Self::X {
    self[args.0]
  }

  fn trace_idxs(self) -> Self::X {
    let mut res = Self::X::zero();
    let mut i = 0;
    let len = self.len();
    while i < len {
      res += self[i];
      i += 1;
    }
    res
  }

  fn mult_idxs(mut self, rhs: Self) -> Self {
    let mut i = 0;
    let len = self.len();
    while i < len {
      self[i] *= rhs[i];
      i += 1;
    }
    self
  }

  fn map<Y>(self, map_fn: fn(Self::X) -> Y) -> Self::Mapped<Y> where Y: AddAssign + MulAssign + Copy + Zero {
    if N == 0 {
      unsafe {*([].as_ptr())}
    } else {
      let mut ret = [map_fn(self[0]); N];
      let mut i = 1;
      let len = self.len();
      while i < len {
        ret[i] = map_fn(self[i]);
        i+=1;
      }
      ret
    }
  }
}



#[derive(Debug, Clone, Copy)]
pub struct Var<X>(X);

impl Index for () {}

impl<X> const MultiVar for Var<X> where X: Copy + ~const MulAssign + AddAssign + Zero {
  type I = ();
  type X = X;
  type Mapped<Y> = Var<Y> where Y: AddAssign + MulAssign + Copy + Zero;

  fn get_idx(&self, _: Self::I) -> Self::X {
    self.0
  }

  fn trace_idxs(self) -> Self::X {
    self.0
  }

  fn mult_idxs(mut self, rhs: Self) -> Self {
    self.0 *= rhs.0;
    self
  }


  fn map<Y>(self, map_fn: fn(Self::X) -> Y) -> Self::Mapped<Y> where Y: AddAssign + MulAssign + Copy + Zero {
    Var(map_fn(self.0))
  }
}

impl<X: Copy> Var<X> {
  pub const fn get_idx(&self) -> X {
    self.0
  }
}
