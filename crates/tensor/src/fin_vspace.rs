use crate::{tensorspace::{TensorAlgebra, DualVariant}, Dim};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FinVSpace;


#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shape(pub Vec<(usize, DualVariant)>);


#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tensor<T> {
  pub shape: Shape,
  pub elems: Vec<T>
}


impl TensorAlgebra for FinVSpace {
    type Index = Vec<usize>;
    type Shape = Shape;
    type Tensor = Tensor<f64>;
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

    fn shape_variant(idx: Shape, i: usize) -> Option<DualVariant> {
      if let Some((_, v)) = idx.0.get(i) {
        Some(*v)
      } else {
        None
      }
    }

    fn swap_shape_indices(mut idx: Self::Shape, i: usize, j: usize) -> Result<Self::Shape, Self::IndexErr> {
      idx.0.swap(i, j);
      Ok(idx)
    }

    fn move_shape_index(mut idx: Self::Shape, src: usize, dst: usize) -> Result<Self::Shape, Self::IndexErr> {
      move_vec_elems(&mut idx.0, src, dst);
      Ok(idx)
    }

    fn mul(a: Self::Tensor, b: Self::Tensor) -> Self::Tensor {
      let Tensor { shape: Shape(mut shape), elems: elems_a } = a;
      let Tensor { shape: Shape(mut shape_b), elems: elems_b } = b;
      shape.append(&mut shape_b);
      let mut elems = Vec::with_capacity(elems_a.len() * elems_b.len());
      for val_a in elems_a {
        for val_b in elems_b.iter() {
          elems.push(val_a * *val_b)
        }
      }
      Tensor { shape: Shape(shape), elems }
    }

    fn rank(t: &Self::Tensor) -> (usize, usize) {
      let (mut contra, mut co) = (0, 0);
      for (_, v) in t.shape.0.iter() {
        match v {
          DualVariant::Contra => contra += 1,
          DualVariant::Co => co += 1,
        }
      }
      (contra, co)
    }

    fn swap_tensor_shape(t: Self::Tensor, i: usize, j: usize) -> Result<Self::Tensor, Self::IndexErr> {
      let Tensor { shape, elems } = t;
      let new_shape = Self::swap_shape_indices(shape, i, j)?;
      todo!()
    }

    fn move_tensor_shape(t: Self::Tensor, src: usize, dst: usize) -> Result<Self::Tensor, Self::IndexErr> {
        todo!()
    }

    fn get(t: Self::Tensor, i: Self::Index) -> Result<Self::Tensor, Self::TensorErr> {
        todo!()
    }

    fn get_shape(t: &Self::Tensor) -> Self::Shape {
        todo!()
    }

    fn get_index_variant(t: &Self::Tensor, i: usize) -> Option<DualVariant> {
        todo!()
    }

    fn contractions(t: Self::Tensor, idxs: &[(usize, usize)]) -> Result<Self::Tensor, Self::TensorErr> {
        todo!()
    }
}


impl<T> Tensor<T> {
  pub fn iter_index(&self, i: usize) -> impl Iterator<Item = usize> {
    // self.elems.iter().skip(n);
    // self.elems.iter()
    let r = (0..10).step_by(step);
    todo!()
  }

  pub fn iter(&self, i: usize) -> impl Iterator<Item = &T> {
    self.iter_index(i).map(|j| &self.elems[j])
  }

  // pub fn iter_mut(&mut self, i: usize) -> impl Iterator<Item = &mut T> {
  //   self.iter_index(i).map(move |j| {&mut self.elems[j]})
  // }

}


pub fn move_vec_elems<T>(elems: &mut Vec<T>, src: usize, dst: usize) {
  let (i0, i1) = (src.min(dst), src.max(dst));
  assert!(i1 < elems.len());
  for i in i0..i1 {
    elems.swap(i, i+1);
  }
}


