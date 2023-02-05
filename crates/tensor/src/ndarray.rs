use std::marker::PhantomData;

pub use ndarray as np;

use np::{IntoDimension, OwnedRepr, RawData};
pub use np::prelude::*;

use crate::tensorspace::{TensorAlgebra, DualVariant};


pub struct NDArrayAlgebraOwned<S: np::RawData>(PhantomData<S>);
// pub struct NDArrayAlgebra<S: np::RawData>(PhantomData<S>);


// impl<S> TensorAlgebra for NDArrayAlgebra<S> where S: np::RawDataClone + np::Data, S::Elem: Clone + np::LinalgScalar {
// impl<S> TensorAlgebra for NDArrayAlgebra<S> where S: np::RawData + np::RawDataClone + np::Data, S::Elem: np::LinalgScalar {
impl<S> TensorAlgebra for NDArrayAlgebraOwned<S> where S: RawData + Clone + np::LinalgScalar {
  type Index = Vec<(np::Axis, usize)>;
  type Shape = Vec<np::AxisDescription>;
  // type Tensor = ArrayBase<S, IxDyn>;
  type Tensor = ArrayBase<OwnedRepr<S>, IxDyn>;
  type IndexErr = ();
  type TensorErr = ();

  fn swap_indices(idx: Self::Index, i: usize, j: usize) -> Result<Self::Index, Self::IndexErr> {
    todo!()
  }

  fn move_index(idx: Self::Index, src: usize, dst: usize) -> Result<Self::Index, Self::IndexErr> {
    todo!()
  }

  fn shape_variant(idx: Self::Shape, i: usize) -> Option<DualVariant> {
    todo!()
  }

  fn swap_shape_indices(idx: Self::Shape, i: usize, j: usize) -> Result<Self::Shape, Self::IndexErr> {
    todo!()
  }

  fn move_shape_index(idx: Self::Shape, src: usize, dst: usize) -> Result<Self::Shape, Self::IndexErr> {
      todo!()
  }

  fn mul(a: Self::Tensor, b: Self::Tensor) -> Self::Tensor {
    let a_shape = a.shape();
    let a_len = a.len();
    let b_shape = b.shape();
    let b_len = b.len();
    let mut shape = Vec::with_capacity(a_shape.len() + b_shape.len());
    shape.append(&mut Vec::from(a_shape));
    shape.append(&mut Vec::from(b_shape));

    let a_flat: ArrayBase<_, Dim<[usize; 2]>> = a.into_shape(Ix2(a_len, 1)).unwrap();
    let b_flat: ArrayBase<_, Dim<[usize; 2]>> = b.into_shape(Ix2(1, b_len)).unwrap();
    let c_mat = a_flat.dot(&b_flat);
    c_mat.into_shape(shape).unwrap()
  }

  fn rank(t: &Self::Tensor) -> (usize, usize) {
    (Self::order(t), 1)
  }

  fn order(t: &Self::Tensor) -> usize {
    t.axes().count()
  }

  fn swap_tensor_shape(mut t: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::IndexErr> {
    t.swap_axes(i, j);
    Ok(t)
  }

  fn move_tensor_shape(mut t: Self::Tensor, src: usize, dst: usize) -> Result<Self::Tensor, Self::IndexErr> {
    match src.cmp(&dst) {
      std::cmp::Ordering::Less => {
        for i in src..dst {
          t.swap_axes(i, i+1)
        }
      },
      std::cmp::Ordering::Greater => {
        for i in ((dst+1)..=src).rev() {
          t.swap_axes(i, i-1)
        }
      },
      std::cmp::Ordering::Equal => (),
    }
    Ok(t)
  }

  fn get(t: Self::Tensor, i: Vec<(np::Axis, usize)>) -> Result<Self::Tensor, Self::TensorErr> {
    // let mut res = t.index_axis_move(axis, index);
    let mut res = t;
    for (axis, idx) in i {
      res = res.index_axis_move(axis, idx)
    }
    Ok(res)
  }

  fn get_shape(t: &Self::Tensor) -> Self::Shape {
    t.axes().collect()
  }

  fn get_index_variant(t: &Self::Tensor, i: usize) -> Option<DualVariant> {
    if i < Self::order(t) {
      Some(DualVariant::Contra)
    } else {
      None
    }
  }

  fn contractions(t: Self::Tensor, idxs: &[(usize, usize)]) -> Result<Self::Tensor, Self::TensorErr> {
    ArrayBase::
  }
}


