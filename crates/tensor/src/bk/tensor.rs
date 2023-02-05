use crate::TensorIndexRepr;

pub enum ContractionErr {
  /// In the general case one index has to be contra-variant (raised) and the other has to be co-
  /// variant (lowered). Try Flipping one index.
  ContractingTwoContravariants,
  /// In the general case one index has to be contra-variant (raised) and the other has to be co-
  /// variant (lowered). Try Flipping one index.
  ContractingTwoCovariants,
  /// Incompatible index types
  IncompatibleIndexTypes,
  /// Contraction is denied for context-dependent reasons (Different physical units for example)
  DeniedContraction,
  /// At least one index is out of bounds
  OutOfBounds,
}

pub type ContractionRes<T> = Result<T, ContractionErr>;


// pub trait Contraction: Tensor {
//   type Output: Contraction;

//   fn contract(self, i: usize, j: usize) -> ContractionRes<Self::Output>;
// }

pub trait TensorMul<Rhs = Self>: Tensor where Rhs: Tensor {
  type Output: Tensor;

  fn tensor_mul(self, rhs: Rhs) -> Self::Output;

  fn contract_with(self, rhs: Rhs, i: usize, j: usize) -> ContractionRes<Self::Output> {
    self.tensor_mul(rhs).contract(i, j)
  }
}


pub trait Tensor: Sized + Clone {
  type I: TensorIndexRepr;
  type SelfContractionOutput: Into<Self> + Tensor;

  fn contract(self, i: usize, j: usize) -> ContractionRes<Self::SelfContractionOutput>;
  // fn elem_at_index(&self, i: Self::I) -> Self::X;
  fn get_index_repr(&self) -> Self::I;
  fn set_index_repr(&mut self, i: Self::I);
  fn contract_self(&self, i: usize, j: usize) -> ContractionRes<Self::SelfContractionOutput>;

  // fn contract_with<C, Rhs>(&self, rhs: &Rhs, i: usize, j: usize) -> ContractionRes<C::Output> where
  // C: Contraction<Self, Rhs>, Rhs: Tensor {
  //   C::contract(self, rhs, i, j)
  // }
  fn contract_self_mut(&mut self, i: usize, j: usize) -> ContractionRes<()> {
    tensor_contract_self_mut_default_self(self, i, j)
  }
}


pub fn tensor_contract_self_mut_default_self<T>(t: &mut T, i: usize, j: usize) -> ContractionRes<()>
where T: Tensor {
  match t.contract_self(i, j) {
    Ok(t_new) => {*t = t_new.into(); Ok(())},
    Err(err) => Err(err)
  }
}




