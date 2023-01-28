use super::{Tensor, ContractionRes};

pub trait Contraction<Lhs, Rhs> where Lhs: Tensor, Rhs: Tensor {
  type Output: Tensor;

  fn contract(lhs: &Lhs, rhs: &Rhs, i: usize, j: usize) -> ContractionRes<Self::Output>;
}





