use std::{hash::Hash, collections::HashSet};

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



pub fn cycles<T: Eq + Copy + Hash>(edges: &[(T, T)]) -> Vec<HashSet<T>> {
  let mut res = Vec::<HashSet<T>>::new();
  'egde_loop: for (e1, e2) in edges {
    for s in res.iter_mut() {
      if s.contains(e1) {
        s.insert(*e2);
        continue 'egde_loop;
      } else if s.contains(e2) {
        s.insert(*e1);
        continue 'egde_loop;
      }
    }
    res.push(HashSet::from([*e1, *e2]));
  }
  res
}

#[test]
fn test_cycles() {
  assert_eq!(
    cycles::<i32>(&[]),
    vec![],
  );

  assert_eq!(
    cycles(&[(1,2), (3,2), (4,5)]),
    Vec::from([vec![1, 2, 3], vec![4, 5]].map(HashSet::from_iter)),
  );

  assert_eq!(
    cycles(&[(1,2), (3,2), (4,2), (5, 5), (6, 7), (7, 6), (7, 8), (8, 9)]),
    Vec::from([vec![1, 2, 3, 4], vec![5], vec![6, 7, 8, 9]].map(HashSet::from_iter)),
  );
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

