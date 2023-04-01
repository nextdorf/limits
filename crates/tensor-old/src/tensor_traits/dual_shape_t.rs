use std::collections::HashSet;

use crate::{move_vec_elems, DisjunctIdxs};

use super::{DualVariant, BoundedShape, DualShape, TensorShape, SelfContractable, ContractRes};


#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DualShapeMetaInfo {
  pub contravariants: usize,
  pub covariants: usize,
  pub dual_shape: Vec<DualVariant>,
}


#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DualShapeT<T> {
  inner: T,
  info: DualShapeMetaInfo,
}


pub type DualShapeMeta<'a, T> = DualShapeT<&'a T>;


impl DualShapeMetaInfo {
  pub fn new(dual_shape: Vec<DualVariant>) -> Self {
    let (mut contra, mut co) = (0, 0);
    for v in dual_shape.iter() {
      match v {
        DualVariant::Contra => contra += 1,
        DualVariant::Co => co += 1,
      }
    }
    Self { contravariants: contra, covariants: co, dual_shape }
  }

  pub fn remove_axis(&mut self, index: usize) {
    let removed_elem = self.dual_shape.remove(index);
    let rank_to_change = match removed_elem {
      DualVariant::Contra => &mut self.contravariants,
      DualVariant::Co => &mut self.covariants,
    };
    *rank_to_change -= 1;
  }

  pub fn without_axes(&self, axes: impl Iterator<Item = usize>) -> Self {
    let mut axes = axes.collect::<Vec<_>>();
    axes.sort();
    axes.dedup();
    let mut dual_shape = vec![];
    let mut vs = self.dual_shape.iter();
    let mut curr_axis = 0;
    for skip_axis in axes {
      for _ in curr_axis..skip_axis {
        dual_shape.push(*vs.next().unwrap())
      }
      curr_axis = skip_axis+1;
    }
    for v in vs {
      dual_shape.push(*v);
    }
    Self::new(dual_shape)
  }
}

impl<T> DualShapeT<T> {
  pub unsafe fn new_unchecked(inner: T, info: DualShapeMetaInfo) -> Self {
    Self { inner, info }
  }

  pub fn info(&self) -> &DualShapeMetaInfo {
    &self.info
  }

  pub fn inner_ref(&self) -> &T {
    &self.inner
  }

  pub fn unwrap(self) -> (T, DualShapeMetaInfo) {
    let Self { inner, info } = self;
    (inner, info)
  }
}

impl<T> DualShapeT<&T> {
  pub fn inner(&self) -> &T {
    self.inner
  }
}


impl<T: BoundedShape> DualShapeT<T> {
  pub fn new(inner: T, info: DualShapeMetaInfo) -> Self {
    assert_eq!(inner.order(), info.order());
    unsafe { Self::new_unchecked(inner, info) }
  }

  pub fn new_from_ref<'a>(inner: &'a T, info: DualShapeMetaInfo) -> DualShapeT<&'a T> {
    assert_eq!(inner.order(), info.order());
    unsafe { DualShapeT::new_unchecked(inner, info) }
  }

  pub fn new_vec(inner: T) -> Self {
    Self::new(inner, DualShapeMetaInfo::new(vec![DualVariant::Contra]))
  }

  pub fn new_dualvec(inner: T) -> Self {
    Self::new(inner, DualShapeMetaInfo::new(vec![DualVariant::Co]))
  }

  pub fn map_vecspace(inner: T) -> Self {
    Self::new(inner, DualShapeMetaInfo::new(vec![DualVariant::Contra, DualVariant::Co]))
  }

  pub fn map_dualspace(inner: T) -> Self {
    Self::new(inner, DualShapeMetaInfo::new(vec![DualVariant::Co, DualVariant::Contra]))
  }
}


impl BoundedShape for DualShapeMetaInfo {
  fn order(&self) -> usize {
    self.contravariants + self.covariants
  }
}

impl DualShape for DualShapeMetaInfo {
  fn variant_at(&self, i: usize) -> Option<DualVariant> {
    self.dual_shape.get(i).cloned()
  }

  fn rank(&self) -> (usize, usize) {
    (self.contravariants, self.covariants)
  }
}

impl TensorShape for DualShapeMetaInfo {
  fn swap_axes(&mut self, i: usize, j: usize) {
    self.dual_shape.swap(i, j)
  }

  fn move_axis(&mut self, src: usize, dst: usize) {
    move_vec_elems(&mut self.dual_shape, src, dst)
  }
}


impl<T> BoundedShape for DualShapeT<T> {
  fn order(&self) -> usize {
    self.info.order()
  }
}

impl<T> DualShape for DualShapeT<T> {
  fn variant_at(&self, i: usize) -> Option<DualVariant> {
    self.info.variant_at(i)
  }

  fn rank(&self) -> (usize, usize) {
    self.info.rank()
  }
}


impl<T: TensorShape> TensorShape for DualShapeT<T> {
  fn swap_axes(&mut self, i: usize, j: usize) {
    self.info.swap_axes(i, j);
    self.inner.swap_axes(i, j);
  }

  fn move_axis(&mut self, src: usize, dst: usize) {
    self.info.move_axis(src, dst);
    self.inner.move_axis(src, dst);
  }
}

impl<T: SelfContractable> SelfContractable for DualShapeT<T> {
  fn contract_gen(self, ax_sets: &DisjunctIdxs) -> ContractRes<Self, Self> {
    match self.inner.contract_gen(ax_sets) {
      Ok(inner) => {
        let axes = ax_sets.interior().iter().flatten().map(|i| *i);
        let info = self.info.without_axes(axes);
        Ok(Self { inner, info })
      },
      Err((err, inner)) => Err((err, Self { inner, info: self.info }))
    }
  }

  fn contract(self, axes: &[[usize; 2]]) -> ContractRes<Self, Self> {
    match self.inner.contract(axes) {
      Ok(inner) => {
        let axes = axes.iter().flatten().map(|i| *i);
        let info = self.info.without_axes(axes);
        Ok(Self { inner, info })
      },
      Err((err, inner)) => Err((err, Self { inner, info: self.info }))
    }
  }

  fn contract_axis(self, i: usize, j: usize) -> ContractRes<Self, Self> {
    match self.inner.contract_axis(i, j) {
      Ok(inner) => {
        let info = self.info.without_axes([i, j].into_iter());
        Ok(Self { inner, info })
      },
      Err((err, inner)) => Err((err, Self { inner, info: self.info }))
    }
  }
}

