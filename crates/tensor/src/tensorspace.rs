use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DualVariant {
  Contra,
  Co,
}


pub trait TensorAlgebra {
  type Index: Clone;
  type Shape: Clone;
  type Tensor: Clone;
  type IndexErr: Debug;
  type TensorErr: Debug;

  fn swap_indices(idx: Self::Index, i: usize, j: usize) -> Result<Self::Index, Self::IndexErr>;
  fn move_index(idx: Self::Index, src: usize, dst: usize) -> Result<Self::Index, Self::IndexErr>;

  fn shape_variant(idx: Self::Shape, i: usize) -> Option<DualVariant>;
  fn swap_shape_indices(idx: Self::Shape, i: usize, j: usize) -> Result<Self::Shape, Self::IndexErr>;
  fn move_shape_index(idx: Self::Shape, src: usize, dst: usize) -> Result<Self::Shape, Self::IndexErr>;
  
  fn mul(a: Self::Tensor, b: Self::Tensor) -> Self::Tensor;
  fn rank(t: &Self::Tensor) -> (usize, usize);
  
  fn swap_tensor_shape(t: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::IndexErr>;
  fn move_tensor_shape(t: Self::Tensor, src: usize, dst: usize) -> Result<Self::Tensor, Self::IndexErr>;
  fn get(t: Self::Tensor, i: Self::Index) -> Result<Self::Tensor, Self::TensorErr>;
  fn get_shape(t: &Self::Tensor) -> Self::Shape;
  fn get_index_variant(t: &Self::Tensor, i: usize) -> Option<DualVariant>;
  fn contractions(t: Self::Tensor, idxs: &[(usize, usize)]) -> Result<Self::Tensor, Self::TensorErr>;

  fn order(t: &Self::Tensor) -> usize {
    let (m, n) = Self::rank(t);
    m + n
  }

  fn contraction(t: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::TensorErr> {
    Self::contractions(t, &[(i, j)])
  }
  fn contractions_with(a: Self::Tensor, b: Self::Tensor, idxs: &[(usize, usize)]) -> Result<Self::Tensor, Self::TensorErr> {
    let a_len = Self::order(&a);
    let idxs: Vec<(usize, usize)> = idxs.iter().map(|(i, j)| (*i, a_len + j)).collect();
    let c = Self::mul(a, b);
    Self::contractions(c, idxs.as_ref())
  }
  fn contraction_with(a: Self::Tensor, b: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::TensorErr> {
    let a_len = Self::order(&a);
    let c = Self::mul(a, b);
    Self::contraction(c, i, j + a_len)
  }
}


pub trait TensorMetrik {
  type Space: TensorAlgebra;

  /// (0, 2) Tensor
  fn as_tensor(&self) -> <Self::Space as TensorAlgebra>::Tensor;

  /// (2, 0) Tensor
  fn as_inv_tensor(&self) -> <Self::Space as TensorAlgebra>::Tensor;

  fn raise(&self, t: <Self::Space as TensorAlgebra>::Tensor, i: usize)
  -> Result<<Self::Space as TensorAlgebra>::Tensor, <Self::Space as TensorAlgebra>::TensorErr> {
    let idx_variant = <Self::Space as TensorAlgebra>::get_index_variant(&t, i).expect("Index could not be returned");
    match idx_variant {
      DualVariant::Contra => Ok(t),
      DualVariant::Co => {
        let g = self.as_inv_tensor();
        let g_t = <Self::Space as TensorAlgebra>::contraction_with(g, t, 0, i)?;
        let raised = <Self::Space as TensorAlgebra>::move_tensor_shape(g_t, 0, i).unwrap();
        Ok(raised)
      },
    }
  }

  fn lower(&self, t: <Self::Space as TensorAlgebra>::Tensor, i: usize)
  -> Result<<Self::Space as TensorAlgebra>::Tensor, <Self::Space as TensorAlgebra>::TensorErr> {
    let idx_variant = <Self::Space as TensorAlgebra>::get_index_variant(&t, i).expect("Index could not be returned");
    match idx_variant {
      DualVariant::Contra => {
        let g = self.as_tensor();
        let g_t = <Self::Space as TensorAlgebra>::contraction_with(g, t, 0, i)?;
        let lowered = <Self::Space as TensorAlgebra>::move_tensor_shape(g_t, 0, i).unwrap();
        Ok(lowered)
      },
      DualVariant::Co => Ok(t),
    }
  }

  fn flip(&self, t: <Self::Space as TensorAlgebra>::Tensor, i: usize)
  -> Result<<Self::Space as TensorAlgebra>::Tensor, <Self::Space as TensorAlgebra>::TensorErr> {
    let idx_variant = <Self::Space as TensorAlgebra>::get_index_variant(&t, i).expect("Index could not be returned");
    let g = match idx_variant {
      DualVariant::Contra => self.as_tensor(),
      DualVariant::Co => self.as_inv_tensor(),
    };
    let g_t = <Self::Space as TensorAlgebra>::contraction_with(g, t, 0, i)?;
    let res = <Self::Space as TensorAlgebra>::move_tensor_shape(g_t, 0, i).unwrap();
    Ok(res)
  }

  fn inv_tensor_transf(&self, t: <Self::Space as TensorAlgebra>::Tensor, i: usize, j: usize)
  -> Result<<Self::Space as TensorAlgebra>::Tensor, <Self::Space as TensorAlgebra>::TensorErr> {
    let t = <Self::Space as TensorAlgebra>::swap_tensor_shape(t, i, j).unwrap();
    self.flip(self.flip(t, i)?, j)
  }

  fn inv_tensor2(&self, t: <Self::Space as TensorAlgebra>::Tensor)
  -> Result<<Self::Space as TensorAlgebra>::Tensor, <Self::Space as TensorAlgebra>::TensorErr> {
    self.inv_tensor_transf(t, 0, 1)
  }
}


impl Default for DualVariant {
  fn default() -> Self {
    Self::Contra
  }
}

