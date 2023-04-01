use std::f64::consts::PI;

pub use ndarray as np;

use ndarray_einsum_beta::{einsum, ArrayLike};
pub use np::prelude::*;


use crate::{tensor_traits::{SelfContractable, ContractRes, ContractErr, Contractable, BoundedShape, TensorShape, Tensor, DualShapeT, TensorIdx, IndexedShape, AsView, AsMutView, DualShapeMetaInfo, DualVariant}, DisjunctIdxs, DisjunctIdxPairs};


impl<A> SelfContractable for ArrayD<A> where A: np::LinalgScalar {
  fn contract_gen(self, ax_sets: &DisjunctIdxs) -> ContractRes<Self, Self> {
    match contract_gen_ref(&self, ax_sets) {
      Ok(res) => Ok(res),
      Err(err) => Err((err, self)),
    }
  }
} 


impl<'a, A> Contractable<&'a ArrayD<A>> for &ArrayD<A> where A: np::LinalgScalar {
  type Output = ArrayD<A>;

  fn contract_gen_with(self, rhs: &'a ArrayD<A>, ax_sets: &DisjunctIdxPairs) -> ContractRes<ArrayD<A>, (Self, &'a ArrayD<A>)> {
    match contract2_gen_ref(self, rhs, ax_sets) {
      Ok(res) => Ok(res),
      Err(err) => Err((err, (self, rhs))),
    }
  }
} 
impl<'a, A> Contractable<&'a ArrayD<A>> for ArrayD<A> where A: np::LinalgScalar {
  type Output = Self;

  fn contract_gen_with(self, rhs: &'a ArrayD<A>, ax_sets: &DisjunctIdxPairs) -> ContractRes<Self, (Self, &'a ArrayD<A>)> {
    match contract2_gen_ref(&self, rhs, ax_sets) {
      Ok(res) => Ok(res),
      Err(err) => Err((err, (self, rhs))),
    }
  }
} 
impl<A> Contractable<ArrayD<A>> for &ArrayD<A> where A: np::LinalgScalar {
  type Output = ArrayD<A>;

  fn contract_gen_with(self, rhs: ArrayD<A>, ax_sets: &DisjunctIdxPairs) -> ContractRes<ArrayD<A>, (Self, ArrayD<A>)> {
    match contract2_gen_ref(self, &rhs, ax_sets) {
      Ok(res) => Ok(res),
      Err(err) => Err((err, (self, rhs))),
    }
  }
}
impl<A> Contractable<ArrayD<A>> for ArrayD<A> where A: np::LinalgScalar {
  type Output = Self;

  fn contract_gen_with(self, rhs: ArrayD<A>, ax_sets: &DisjunctIdxPairs) -> ContractRes<Self, (Self, Self)> {
    match contract2_gen_ref(&self, &rhs, ax_sets) {
      Ok(res) => Ok(res),
      Err(err) => Err((err, (self, rhs))),
    }
  }
}


pub fn contract_gen_ref<A>(arr: &ArrayD<A>, ax_sets: &DisjunctIdxs) -> Result<ArrayD<A>, ContractErr> where A: np::LinalgScalar {
  // const letters: &str = "abcdefghijklmnopqrstuvwxyzαβγδεζηθικλμνξοπρστυφχψω";
  const LETTERS: [char; 26 + 24] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z', 'α', 'β', 'γ', 'δ',
    'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ',
    'ο', 'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω'
  ];
  // let max_axis_val = ax_sets.interior()
  //   .iter()
  //   .map(|s| *s.iter().max().unwrap_or(&0))
  //   .max()
  //   .unwrap_or(0);
  // assert!(max_axis_val < LETTERS.len());

  let mut input_str = Vec::from_iter(std::iter::repeat(' ').take(arr.axes().count()));
  let mut i = 0;
  for s in ax_sets.interior() {
    for axi in s {
      input_str[*axi] = LETTERS[i];
    }
    i += 1
  }
  let mut output_str = Vec::new();
  for c in input_str.iter_mut() {
    if *c == ' ' {
      *c = LETTERS[i];
      output_str.push(LETTERS[i]);
      i+=1;
    }
  }
  let input_str = String::from_iter(input_str) + " -> " + String::from_iter(output_str).as_str();
  match einsum(input_str.as_str(), &[arr]) {
    Ok(res) => Ok(res),
    Err(estr) => {eprintln!("{estr}"); Err(ContractErr::Irrepresentable)},
  }
}


pub fn contract2_gen_ref<A>(arr: &ArrayD<A>, brr: &ArrayD<A>, ax_sets: &DisjunctIdxPairs) -> Result<ArrayD<A>, ContractErr> where A: np::LinalgScalar {
  // const letters: &str = "abcdefghijklmnopqrstuvwxyzαβγδεζηθικλμνξοπρστυφχψω";
  const LETTERS: [char; 26 + 24] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z', 'α', 'β', 'γ', 'δ',
    'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ',
    'ο', 'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω'
  ];
  // let max_axis_val = ax_sets.interior()
  //   .iter()
  //   .map(|s| *s.iter().max().unwrap_or(&0))
  //   .max()
  //   .unwrap_or(0);
  // assert!(max_axis_val < LETTERS.len());

  let mut input_str_a = Vec::from_iter(std::iter::repeat(' ').take(arr.axes().count()));
  let mut input_str_b = Vec::from_iter(std::iter::repeat(' ').take(brr.axes().count()));
  let mut i = 0;
  for s in ax_sets.interior() {
    for axi in s {
      // let input_str = match *Into::<&usize>::into(axi.idx()) {
      let input_str = match axi.idx_as_usize() {
        0 => &mut input_str_a,
        1 => &mut input_str_b,
        _ => unreachable!()
      };
      input_str[axi.0] = LETTERS[i];
    }
    i += 1
  }
  let mut output_str = Vec::new();
  let input_str_iter_mut = input_str_a.iter_mut().chain(input_str_b.iter_mut());
  for c in input_str_iter_mut {
    if *c == ' ' {
      *c = LETTERS[i];
      output_str.push(LETTERS[i]);
      i+=1;
    }
  }
  // let input_str = String::from_iter(input_str_a)
  //   + ", " + String::from_iter(input_str_b).as_str()
  //   + " -> " + String::from_iter(output_str).as_str();
  let input_str = format!("{}, {} -> {}", String::from_iter(input_str_a), String::from_iter(input_str_b), String::from_iter(output_str));
  match einsum(input_str.as_str(), &[arr, brr]) {
    Ok(res) => Ok(res),
    Err(estr) => {eprintln!("{estr}"); Err(ContractErr::Irrepresentable)},
  }
}


// impl<D> BoundedShape for D where D: Dimension {
//   fn order(&self) -> usize {
//     self.ndim()
//   }
// }


impl<S, D> BoundedShape for ArrayBase<S, D> where S: np::RawData, D: Dimension {
  fn order(&self) -> usize {
    self.ndim()
  }
}


impl<S, D> TensorShape for ArrayBase<S, D> where S: np::RawDataClone, D: Dimension {
  fn swap_axes(&mut self, i: usize, j: usize) {
    self.swap_axes(i, j)
  }
}


impl<S> IndexedShape<usize> for ArrayBase<S, IxDyn> where S: np::Data {
  fn get(mut self, idxs: &[TensorIdx<usize>]) -> Self {
    for TensorIdx { axis, idx } in idxs.iter() {
      self.index_axis_inplace(Axis(*axis), *idx);
    }
    self
  }
}


impl<S, D> AsView<usize> for ArrayBase<S, D> where S: np::Data, D: np::RemoveAxis {
  type View<'a> = ArrayView<'a, S::Elem, IxDyn> where S::Elem: 'a, Self: 'a;
  // type View<'a> = ArrayBase<np::ViewRepr<&'a S::Elem>, IxDyn> where S::Elem: 'a, Self: 'a;

  fn get_view(&self, idxs: &[TensorIdx<usize>]) -> ArrayView<'_, S::Elem, IxDyn> {
    let mut res = self.view().into_dyn();
    for TensorIdx { axis, idx } in idxs {
      res = res.index_axis_move(Axis(*axis), *idx);
    }
    res
  }
}

impl<S, D> AsMutView<usize> for ArrayBase<S, D> where S: np::DataMut, D: np::RemoveAxis {
  type MutView<'a> = ArrayViewMut<'a, S::Elem, IxDyn> where S::Elem: 'a, Self: 'a;
  // type MutView<'a> = ArrayBase<np::ViewRepr<&'a mut S::Elem>, IxDyn> where S::Elem: 'a, Self: 'a;

  fn get_mut_view(&mut self, idxs: &[TensorIdx<usize>]) -> ArrayViewMut<'_, S::Elem, IxDyn> {
    let mut res = self.view_mut().into_dyn();
    for TensorIdx { axis, idx } in idxs {
      res = res.index_axis_move(Axis(*axis), *idx);
    }
    res
  }
}


pub type DualTensor<A> = DualShapeT<ArrayD<A>>;


#[test]
fn dualtensor_check() {
  let eps = Array::from_shape_fn((3, 3, 3), |inds| {
    match inds {
      (0, 1, 2) | (1, 2, 0) | (2, 0, 1) => 1.,
      (0, 2, 1) | (1, 0, 2) | (2, 1, 0) => -1.,
      _ => 0.
    }
  });
  let rot_g_tensor: DualTensor<f64> = DualShapeT::new(eps.into_dyn(),
    DualShapeMetaInfo::new(vec![
    DualVariant::Co,
    DualVariant::Contra,
    DualVariant::Co,
  ]));

  let n = array![1.234, -0.2, 0.876];
  let n_norm = (n.dot(&n) as f64).sqrt();
  let phi = 30./180.*PI;
  let n = DualShapeT::new_vec(n / n_norm * phi);

  let rot_g = rot_g_tensor.contract(n);
}


