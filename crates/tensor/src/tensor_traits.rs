// mod impl_traits;
mod dual_shape_t;
// pub use impl_traits::*;
pub use dual_shape_t::*;

use std::collections::HashSet;

use crate::{DisjunctIdxs, DisjunctIdxPairs, TupleElem, NumsLT};




#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContractErr {
  AxisOutOfBounds(usize),
  IncompatibleAxes(usize, usize),
  Irrepresentable
}
pub type ContractBaseRes<T> = Result<T, ContractErr>;
pub type ContractRes<T, U> = Result<T, (ContractErr, U)>;

pub trait Contractable<Rhs = Self>: Sized {
  type Output;

  fn contract_gen_with(self, rhs: Rhs, ax_sets: &DisjunctIdxPairs) -> ContractRes<Self::Output, (Self, Rhs)>;

  fn contract_with(self, rhs: Rhs, axes: &[[usize; 2]]) -> ContractRes<Self::Output, (Self, Rhs)> {
    self.contract_gen_with(rhs, &ax_disjunct_sets(axes))
  }

  fn contract_axis_with(self, rhs: Rhs, i: usize, j: usize) -> ContractRes<Self::Output, (Self, Rhs)> {
    self.contract_with(rhs, &[[i, j]])
  }

  fn tensor_mul(self, rhs: Rhs) -> ContractRes<Self::Output, (Self, Rhs)> {
    self.contract_with(rhs, &[])
  }
}


pub trait SelfContractable: Sized {
  fn contract_gen(self, ax_sets: &DisjunctIdxs) -> ContractRes<Self, Self>;

  fn contract(self, axes: &[[usize; 2]]) -> ContractRes<Self, Self> {
    self.contract_gen(&ax_sets(axes))
  }

  fn contract_axis(self, i: usize, j: usize) -> ContractRes<Self, Self> {
    self.contract(&[[i, j]])
  }
}


pub fn ax_sets(axes: &[[usize; 2]]) -> DisjunctIdxs {
  let axes = axes.iter().map(|pair| HashSet::from(*pair));
  Vec::from_iter(axes).into()
}

pub fn ax_disjunct_sets(axes: &[[usize; 2]]) -> DisjunctIdxPairs {
  let axes = axes.iter().map(|pair| {
    let pair = TupleElem::<usize, NumsLT<2>>
      ::try_from_iter((*pair).into_iter())
      .unwrap();
    HashSet::from_iter(pair)
  });
  Vec::from_iter(axes).into()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DualVariant {
  Contra,
  Co
}


pub trait BoundedShape {
  fn order(&self) -> usize;
}

pub trait DualShape: BoundedShape {
  fn variant_at(&self, i: usize) -> Option<DualVariant>;

  fn rank(&self) -> (usize, usize);
}


pub trait TensorShape: Clone + BoundedShape {
  fn swap_axes(&mut self, i: usize, j: usize);

  fn move_axis(&mut self, src: usize, dst: usize) {
    match src.cmp(&dst) {
      std::cmp::Ordering::Equal => return,
      std::cmp::Ordering::Less => {
        for i in src..dst {
          self.swap_axes(i, i+1)
        }
      },
      std::cmp::Ordering::Greater => {
        for i in ((dst-1)..=src).rev() {
          self.swap_axes(i, i+1)
        }
      },
    }
  }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorIdx<T> {
  pub axis: usize,
  pub idx: T,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum TensorRepr<T, Idx> where T: Tensor<Idx> {
//   Tensor(T),
//   View(T::View),
//   // Func(fn() -> T),
// }

// pub trait Tensor<Idx>: TensorShape + DualShape + SelfContractable + Contractable<TensorRepr<Self, Idx>> {
//   type Shape: DualShape;
//   type TensorView: Tensor<Idx, Shape = Self::Shape> + Contractable<Self>; //+ Contractable<Self, Output = Self::Output>;

//   fn get_shape(&self) -> Self::Shape;
//   fn get(self, idxs: &[TensorIdx<Idx>]) -> Self;
//   fn get_view(&self, idxs: &[TensorIdx<Idx>]) -> Self::TensorView;

//   fn as_view(&self) -> Self::TensorView {
//     self.get_view(&[])
//   }
// }


pub trait IndexedShape<Idx> {
  fn get(self, idxs: &[TensorIdx<Idx>]) -> Self;
}


// pub trait Tensor<Idx>: TensorShape + DualShape + IndexedShape<Idx> + SelfContractable + Contractable<Self> + Contractable<Self::TensorView> {//+ Contractable<TensorRepr<Self, Idx>> {
//   type Shape: DualShape;
//   type TensorView: Tensor<Idx, Shape = Self::Shape> + Contractable<Self, Output = <Self as Contractable<Self::TensorView>>::Output>; //+ Contractable<Self, Output = Self::Output>;

//   // fn get_shape(&self) -> Self::Shape;
//   fn get_view(&self, idxs: &[TensorIdx<Idx>]) -> Self::TensorView;

//   fn as_view(&self) -> Self::TensorView {
//     self.get_view(&[])
//   }
// }


pub trait AsView<Idx> {
  type View<'a> where Self: 'a;

  fn get_view(&self, idxs: &[TensorIdx<Idx>]) -> Self::View<'_>;

  fn as_view(&self) -> Self::View<'_> {
    self.get_view(&[])
  }
}


pub trait AsMutView<Idx>: AsView<Idx> {
  type MutView<'a> where Self: 'a;

  fn get_mut_view(&mut self, idxs: &[TensorIdx<Idx>]) -> Self::MutView<'_>;

  fn as_mut_view(&mut self) -> Self::MutView<'_> {
    self.get_mut_view(&[])
  }
}


pub trait TensorBase<Idx, COut>: DualShape + IndexedShape<Idx> + TensorShape
  + Contractable<Self, Output = COut> + SelfContractable
{}

pub trait TensorView<'a, Idx, COut>: 'a + AsMutView<Idx>
  + Contractable<Self::MutView<'a>, Output = COut> + Contractable<Self::View<'a>, Output = COut>
where Self::View<'a>: Contractable<Self, Output = COut>, Self::MutView<'a>: Contractable<Self, Output = COut>
{}

pub trait Tensor<'a, Idx, COut>: TensorBase<Idx, COut> + TensorView<'a, Idx, COut>
where Self::View<'a>: Contractable<Self, Output = COut>, Self::MutView<'a>: Contractable<Self, Output = COut>
{}


impl<Idx, COut, T> TensorBase<Idx, COut> for T where
T: DualShape + IndexedShape<Idx> + TensorShape + Contractable<Self, Output = COut> + SelfContractable
{}

impl<'a, Idx, COut, T> TensorView<'a, Idx, COut> for T where
T: 'a + AsMutView<Idx> + Contractable<Self::MutView<'a>, Output = COut> + Contractable<Self::View<'a>, Output = COut>,
Self::View<'a>: Contractable<Self, Output = COut>,
Self::MutView<'a>: Contractable<Self, Output = COut>
{}

impl<'a, Idx, COut, T> Tensor<'a, Idx, COut> for T where
T: TensorBase<Idx, COut> + TensorView<'a, Idx, COut>,
Self::View<'a>: Contractable<Self, Output = COut>,
Self::MutView<'a>: Contractable<Self, Output = COut>
{}



