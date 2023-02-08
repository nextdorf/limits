use std::marker::PhantomData;

pub use ndarray as np;

use ndarray_einsum_beta::{tensordot, einsum};
use np::RawData;
pub use np::prelude::*;

use crate::{
  tensorspace::{
    TensorAlgebra,
    DualVariant
  },
  move_vec_elems,
  cycles
};


pub struct NDArrayAlgebra<A>(PhantomData<A>);
// pub struct NDArrayAlgebra<S: np::RawData>(PhantomData<S>);


// impl<S> TensorAlgebra for NDArrayAlgebra<S> where S: np::RawDataClone + np::Data, S::Elem: Clone + np::LinalgScalar {
// impl<S> TensorAlgebra for NDArrayAlgebra<S> where S: np::RawData + np::RawDataClone + np::Data, S::Elem: np::LinalgScalar {
impl<A> TensorAlgebra for NDArrayAlgebra<A> where A: Clone + np::LinalgScalar {
  type Index = Vec<(np::Axis, usize)>;
  type Shape = Vec<np::AxisDescription>;
  // type Tensor = ArrayBase<S, IxDyn>;
  type Tensor = ArrayD<A>;
  type IndexErr = ();
  type TensorErr = ();

  fn swap_indices(mut idx: Self::Index, i: usize, j: usize) -> Result<Self::Index, Self::IndexErr> {
    idx.swap(i, j);
    Ok(idx)
  }

  fn move_index(mut idx: Self::Index, src: usize, dst: usize) -> Result<Self::Index, Self::IndexErr> {
    move_vec_elems(&mut idx, src, dst);
    Ok(idx)
  }

  fn shape_variant(idx: Self::Shape, i: usize) -> Option<DualVariant> {
    if i < idx.len() {
      Some(DualVariant::Contra)
    } else {
      None
    }
  }

  fn swap_shape_indices(mut idx: Self::Shape, i: usize, j: usize) -> Result<Self::Shape, Self::IndexErr> {
    idx.swap(i, j);
    Ok(idx)
  }

  fn move_shape_index(mut idx: Self::Shape, src: usize, dst: usize) -> Result<Self::Shape, Self::IndexErr> {
    move_vec_elems(&mut idx, src, dst);
    Ok(idx)
  }

  fn mul(a: ArrayD<A>, b: ArrayD<A>) -> ArrayD<A> {
    NDArrayAlgebra::contractions_over(&a, &b, &[])
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

  fn contractions_with(a: Self::Tensor, b: Self::Tensor, idxs: &[(usize, usize)]) -> Result<Self::Tensor, Self::TensorErr> {
    Ok(NDArrayAlgebra::contractions_with(&a, &b, idxs))
  }
  
  fn contractions(t: Self::Tensor, idxs: &[(usize, usize)]) -> Result<Self::Tensor, Self::TensorErr> {
    Ok(NDArrayAlgebra::contractions(&t, idxs))
  }

  fn contraction_with(a: Self::Tensor, b: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::TensorErr> {
    Ok(NDArrayAlgebra::contraction_with(&a, &b, i, j))
  }

  fn contraction(t: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::TensorErr> {
    Ok(NDArrayAlgebra::contraction(&t, i, j))
  }

  
}



impl<A: np::LinalgScalar> NDArrayAlgebra<A> {
  pub fn contractions_over<RhsS, LhsS, RhsD, LhsD>(lhs: &ArrayBase<LhsS, LhsD>, rhs: &ArrayBase<RhsS, RhsD>, axes: &[(Axis, Axis)]) -> ArrayD<A>
    where RhsS: np::Data<Elem = A>, LhsS: np::Data<Elem = A>, LhsD: Dimension, RhsD: Dimension
  {
    let (mut lhs_axes, mut rhs_axes) = (Vec::with_capacity(axes.len()), Vec::with_capacity(axes.len()));
    for (a, b) in axes {
      lhs_axes.push(*a);
      rhs_axes.push(*b);
    }
    tensordot(lhs, rhs, &lhs_axes[..], &rhs_axes[..])
  }
  
  pub fn contractions_with<RhsS, LhsS, RhsD, LhsD>(lhs: &ArrayBase<LhsS, LhsD>, rhs: &ArrayBase<RhsS, RhsD>, axes: &[(usize, usize)]) -> ArrayD<A>
    where RhsS: np::Data<Elem = A>, LhsS: np::Data<Elem = A>, LhsD: Dimension, RhsD: Dimension
  {
    let (mut lhs_axes, mut rhs_axes) = (Vec::with_capacity(axes.len()), Vec::with_capacity(axes.len()));
    for (a, b) in axes {
      lhs_axes.push(Axis(*a));
      rhs_axes.push(Axis(*b));
    }
    tensordot(lhs, rhs, &lhs_axes[..], &rhs_axes[..])
  }
  
  pub fn contraction_with<RhsS, LhsS, RhsD, LhsD>(lhs: &ArrayBase<LhsS, LhsD>, rhs: &ArrayBase<RhsS, RhsD>, i: usize, j: usize) -> ArrayD<A>
    where RhsS: np::Data<Elem = A>, LhsS: np::Data<Elem = A>, LhsD: Dimension, RhsD: Dimension
  {
    tensordot(lhs, rhs, &[Axis(i)], &[Axis(j)])
  }
  
  pub fn contractions<S, D>(t: &ArrayBase<S, D>, axes: &[(usize, usize)]) -> ArrayD<A> where S: np::Data<Elem = A>, D: Dimension {
    // const letters: &str = "abcdefghijklmnopqrstuvwxyzαβγδεζηθικλμνξοπρστυφχψω";
    const LETTERS: [char; 26 + 24] = [
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
      'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
      'u', 'v', 'w', 'x', 'y', 'z', 'α', 'β', 'γ', 'δ',
      'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ',
      'ο', 'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω'
    ];
    let max_axis_val = axes.iter().max().unwrap();
    let max_axis_val = max_axis_val.0.max(max_axis_val.1);
    assert!(max_axis_val < LETTERS.len());

    let axes_cycles = cycles(axes);
    let mut input_str = Vec::from_iter(std::iter::repeat(' ').take(t.len()));
    let mut i = 0;
    for axis_set in axes_cycles {
      for axi in axis_set {
        input_str[axi] = LETTERS[i];
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
    einsum(input_str.as_str(), &[t]).unwrap()
  }
  
  pub fn contraction<S, D>(t: &ArrayBase<S, D>, i: usize, j: usize) -> ArrayD<A> where S: np::Data<Elem = A>, D: Dimension {
    Self::contractions(t, &[(i, j)])
  }
}
