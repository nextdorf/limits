use std::{marker::PhantomData, ops::Mul};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exp<X>(pub X);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Monom<S, X> where S: From<isize> {
  X(S, X, isize),
  Zero,
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonomN<const N: isize, S, X>(pub S, pub X) where S: From<isize>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroExpr;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Log<X>(pub X);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum<X>(pub Vec<X>);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prod<X>(pub Vec<X>);



impl<X> Exp<X> {
  pub fn diff(self) -> Self { self }
}

impl<S, X> Monom<S, X> where S: From<isize> + Mul<S, Output = S> {
  pub fn diff(self) -> Self { 
    match self {
      Self::Zero => Self::Zero,
      Self::X(_, _, 0) => Self::Zero,
      Self::X(a, x, n) => Self::X(a * n.into(), x, n-1),
    }
  }
}

impl ZeroExpr where {
  pub fn diff(self) -> Self { self }
}

impl<X> Log<X> where {
  pub fn diff<S: From<isize>>(self) -> MonomN<-1, S, X> { MonomN(1.into(), self.0) }
}



impl<const N: isize, S, X> From<MonomN<N, S, X>> for Monom<S, X> where S: From<isize> {
  fn from(value: MonomN<N, S, X>) -> Self {
    let MonomN(a, x) = value;
    Self::X(a, x, N)
  }
}


