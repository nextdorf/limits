use crate::func::Index;

#[const_trait]
pub trait BinOp {
  type I: Index;
  type X;

  fn neutral_elem() -> Self::X;

  fn commutative() -> bool;

  fn eval_binop(self) -> Self::X;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Sum<Xs>(pub Xs);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Prod<Xs>(pub Xs);


