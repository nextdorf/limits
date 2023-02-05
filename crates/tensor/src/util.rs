use std::iter::Skip;

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


//use step-by instead
// pub struct NthIter<Iter: Iterator>{
//   i: Iter,
//   n: usize,
// }

// impl<Iter: Iterator> Iterator for NthIter<Iter>  {
//   type Item = Iter::Item;

//   fn next(&mut self) -> Option<Self::Item> {
//     self.i.nth(self.n)
//   }
// }

// impl<Iter: Iterator> NthIter<Iter> {
//   pub fn from(i: Iter, n: usize) -> Self {
//     Self { i, n }
//   }

//   pub fn with_offset_from(i: Iter, n: usize, offset: usize) -> NthIter<Skip<Iter>> {
//     NthIter { i: i.skip(offset), n }
//   }
// }


