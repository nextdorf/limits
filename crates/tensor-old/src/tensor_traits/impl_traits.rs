use crate::{DisjunctIdxs, DisjunctIdxPairs};

use super::{Tensor, TensorRepr, SelfContractable, ContractRes, Contractable, BoundedShape, DualShape, DualVariant, TensorShape, TensorIdx};

impl<T, Idx> SelfContractable for TensorRepr<T, Idx> where T: Tensor<Idx> {
  fn contract_gen(self, ax_sets: &DisjunctIdxs) -> ContractRes<Self, Self> {
    match self {
      Self::Tensor(t) => {
        match t.contract_gen(ax_sets) {
          Ok(ct) => Ok(Self::Tensor(ct)),
          Err((err, this)) => Err((err, Self::Tensor(this))),
        }
      },
      Self::View(v) => {
        match v.contract_gen(ax_sets) {
          Ok(cv) => Ok(Self::View(cv)),
          Err((err, this)) => Err((err, Self::View(this))),
        }
      },
    }
  }

  fn contract(self, axes: &[[usize; 2]]) -> ContractRes<Self, Self> {
    match self {
      Self::Tensor(t) => {
        match t.contract(axes) {
          Ok(ct) => Ok(Self::Tensor(ct)),
          Err((err, this)) => Err((err, Self::Tensor(this))),
        }
      },
      Self::View(v) => {
        match v.contract(axes) {
          Ok(cv) => Ok(Self::View(cv)),
          Err((err, this)) => Err((err, Self::View(this))),
        }
      },
    }
  }

  fn contract_axis(self, i: usize, j: usize) -> ContractRes<Self, Self> {
    match self {
      Self::Tensor(t) => {
        match t.contract_axis(i, j) {
          Ok(ct) => Ok(Self::Tensor(ct)),
          Err((err, this)) => Err((err, Self::Tensor(this))),
        }
      },
      Self::View(v) => {
        match v.contract_axis(i, j) {
          Ok(cv) => Ok(Self::View(cv)),
          Err((err, this)) => Err((err, Self::View(this))),
        }
      },
    }
  }
}


impl<T, Idx, Rhs> Contractable<Rhs> for TensorRepr<T, Idx> where
  T: Tensor<Idx> + Contractable<Rhs>, T::TensorView: Contractable<Rhs, Output = <T as Contractable<Rhs>>::Output>
{
  type Output = <T as Contractable<Rhs>>::Output;

  fn contract_gen_with(self, rhs: Rhs, ax_disjunct_sets: &DisjunctIdxPairs) -> ContractRes<Self::Output, (Self, Rhs)> {
    match self {
      Self::Tensor(t) => match t.contract_gen_with(rhs, ax_disjunct_sets) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::Tensor(this), rhs))),
      },
      Self::View(v) => match v.contract_gen_with(rhs, ax_disjunct_sets) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::View(this), rhs))),
      },
    }
  }

  fn contract_with(self, rhs: Rhs, axes: &[[usize; 2]]) -> ContractRes<Self::Output, (Self, Rhs)> {
    match self {
      Self::Tensor(t) => match t.contract_with(rhs, axes) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::Tensor(this), rhs))),
      },
      Self::View(v) => match v.contract_with(rhs, axes) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::View(this), rhs))),
      },
    }
  }

  fn contract_axis_with(self, rhs: Rhs, i: usize, j: usize) -> ContractRes<Self::Output, (Self, Rhs)> {
    match self {
      Self::Tensor(t) => match t.contract_axis_with(rhs, i, j) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::Tensor(this), rhs))),
      },
      Self::View(v) => match v.contract_axis_with(rhs, i, j) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::View(this), rhs))),
      },
    }
  }

  fn tensor_mul(self, rhs: Rhs) -> ContractRes<Self::Output, (Self, Rhs)> {
    match self {
      Self::Tensor(t) => match t.tensor_mul(rhs) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::Tensor(this), rhs))),
      },
      Self::View(v) => match v.tensor_mul(rhs) {
        Ok(ct) => Ok(ct),
        Err((err, (this, rhs))) => Err((err, (Self::View(this), rhs))),
      },
    }
  }
}


// impl<T, Idx> Contractable<TensorRepr<TensorRepr<T, Idx>, Idx>> for TensorRepr<T, Idx> where
//   T: Tensor<Idx> , T::TensorView: Contractable<TensorRepr<T, Idx>, Output = T::Output>
// {
//   type Output = T::O;
// }




impl<T, Idx> BoundedShape for TensorRepr<T, Idx> where T: Tensor<Idx> {
  fn order(&self) -> usize {
    match self {
      TensorRepr::Tensor(t) => t.order(),
      TensorRepr::View(v) => v.order(),
    }
  }
}


impl<T, Idx> DualShape for TensorRepr<T, Idx> where T: Tensor<Idx> {
  fn variant_at(&self, i: usize) -> Option<DualVariant> {
    match self {
      TensorRepr::Tensor(t) => t.variant_at(i),
      TensorRepr::View(v) => v.variant_at(i),
    }
  }

  fn rank(&self) -> (usize, usize) {
    match self {
      TensorRepr::Tensor(t) => t.rank(),
      TensorRepr::View(v) => v.rank(),
    }
  }
}


impl<T, Idx> TensorShape for TensorRepr<T, Idx> where T: Tensor<Idx>, Idx: Clone {
  fn swap_axes(&mut self, i: usize, j: usize) {
    match self {
      TensorRepr::Tensor(t) => t.swap_axes(i, j),
      TensorRepr::View(v) => v.swap_axes(i, j),
    }
  }

  fn move_axis(&mut self, src: usize, dst: usize) {
    match self {
      TensorRepr::Tensor(t) => t.move_axis(src, dst),
      TensorRepr::View(v) => v.move_axis(src, dst),
    }
  }
}


// impl<T, Idx, View> Tensor<Idx> for TensorRepr<T, Idx> where
//   T: Tensor<Idx, TensorView = View>,
//   T::TensorView: Tensor<Idx, TensorView = View>,
//   View: Tensor<Idx, Shape = T::Shape> + Contractable<Self, Output = Self::Output>,
//   Self: Contractable<TensorRepr<Self, Idx>>, 
//   Idx: Clone,
// {
//   type Shape = T::Shape;
//   type TensorView = View;
/////////////////////////////////
// impl<T, Idx> Tensor<Idx> for TensorRepr<T, Idx> where
//   T: Tensor<Idx>,
//   Self: Contractable<TensorRepr<Self, Idx>>,
//   // T::TensorView: Contractable<Self>,
//   Idx: Clone,
//   TensorRepr<T::TensorView, Idx>: Contractable<Self, Output = Self::Output>
// {
//   type Shape = T::Shape;
//   type TensorView = TensorRepr<T::TensorView, Idx>;

//   fn get_shape(&self) -> Self::Shape {
//     match self {
//       TensorRepr::Tensor(t) => t.get_shape(),
//       TensorRepr::View(v) => v.get_shape(),
//     }
//   }

//   fn get(self, idxs: &[TensorIdx<Idx>]) -> Self {
//     match self {
//       TensorRepr::Tensor(t) => TensorRepr::Tensor(t.get(idxs)),
//       TensorRepr::View(v) => TensorRepr::View(v.get(idxs)),
//     }
//   }

//   fn get_view(&self, idxs: &[TensorIdx<Idx>]) -> TensorRepr<T::TensorView, Idx> {
//     match self {
//       TensorRepr::Tensor(t) => TensorRepr::Tensor(t.get_view(idxs)),
//       TensorRepr::View(v) => TensorRepr::View(v.get_view(idxs)),
//     }
//   }
// }
impl<T, Idx, View> Tensor<Idx> for TensorRepr<T, Idx> where
  T: Tensor<Idx, TensorView = View>,
  T::TensorView: Tensor<Idx, TensorView = View>,
  Self: Contractable<TensorRepr<Self, Idx>>,
  View: Tensor<Idx, Shape = T::Shape> + Contractable<Self>,
  Idx: Clone,
{
  type Shape = T::Shape;
  type TensorView = View;

  fn get_shape(&self) -> Self::Shape {
    match self {
      TensorRepr::Tensor(t) => t.get_shape(),
      TensorRepr::View(v) => v.get_shape(),
    }
  }

  fn get(self, idxs: &[TensorIdx<Idx>]) -> Self {
    match self {
      TensorRepr::Tensor(t) => TensorRepr::Tensor(t.get(idxs)),
      TensorRepr::View(v) => TensorRepr::View(v.get(idxs)),
    }
  }

  fn get_view(&self, idxs: &[TensorIdx<Idx>]) -> View {
    match self {
      TensorRepr::Tensor(t) => t.get_view(idxs),
      TensorRepr::View(v) => v.get_view(idxs),
    }
  }

  fn as_view(&self) -> View {
    match self {
      TensorRepr::Tensor(t) => t.as_view(),
      TensorRepr::View(v) => v.as_view(),
    }
  }
}



impl<T, Idx> From<T> for TensorRepr<T, Idx> where T: Tensor<Idx> {
  fn from(value: T) -> Self {
    Self::Tensor(value)
  }
}


impl<T, Idx, View> From<TensorRepr<TensorRepr<T, Idx>, Idx>> for TensorRepr<T, Idx> where
  T: Tensor<Idx, TensorView = View>, Self: Tensor<Idx, TensorView = View>,
{
  fn from(value: TensorRepr<TensorRepr<T, Idx>, Idx>) -> Self {
    match value {
      TensorRepr::Tensor(t) => match t {
        TensorRepr::Tensor(t) => Self::Tensor(t),
        TensorRepr::View(v) => Self::View(v),
      },
      TensorRepr::View(v) => Self::View(v),
    }
  }
}



