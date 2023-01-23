mod sum;

use super::Index;
pub use sum::*;

#[const_trait]
pub trait BinOp {
  type I: Index;
  type X;

  fn neutral_elem() -> Self::X;

  fn commutative() -> bool;

  fn eval_binop(self) -> Self::X;
}

