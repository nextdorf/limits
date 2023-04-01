use std::ops::{Add, Mul};

use super::{Index, DualIndex, Tensor, TensorIndex, ContractionRes, SelfDualIndex};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dim<const N: usize>(pub usize);

impl<const N: usize> Index for Dim<N> {}
impl<const N: usize> SelfDualIndex for Dim<N> { }


impl<const N: usize, X> Tensor for Vec<[X; N]> where X: Add<X, Output = X>, X: Mul<X, Output = X> + Clone {
    type I = TensorIndex<Dim<N>>;
    type SelfContractionOutput = Self;

    fn get_index_repr(&self) -> TensorIndex<Dim<N>> {
      self.get(index)
    }

    fn set_index_repr(&mut self, i: TensorIndex<Dim<N>>) {
        todo!()
    }

    fn contract_self(&self, i: usize, j: usize) -> ContractionRes<Self> {
        todo!()
    }
} 

