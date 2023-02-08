mod disjunct_sets;
mod disjunct_tuple;
pub use disjunct_sets::*;
pub use disjunct_tuple::*;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dim<const N: usize>(pub usize);

impl<const N: usize> Dim<N> {
  pub fn new(n: usize) -> Self {
    assert!(n < N);
    Self(n)
  }

  pub fn unwrap(self) -> usize {
    self.0
  }
}


pub fn move_vec_elems<T>(elems: &mut Vec<T>, src: usize, dst: usize) {
  match src.cmp(&dst) {
    std::cmp::Ordering::Less => {
      for i in src..dst {
        elems.swap(i, i+1)
      }
    },
    std::cmp::Ordering::Greater => {
      for i in ((dst+1)..=src).rev() {
        elems.swap(i, i-1)
      }
    },
    std::cmp::Ordering::Equal => (),
  }
}

