use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Default, Clone)]
pub struct DisjunctSets<T>(Vec<HashSet<T>>);


impl<T: Hash + Eq> DisjunctSets<T> {
  pub fn new(value: Vec<HashSet<T>>) -> Self {
    let mut res = Self(Vec::new());
    for set in value {
      res.append(set)
    }
    res
  }

  pub fn append(&mut self, set: HashSet<T>) {
    for s in self.0.iter_mut() {
      let mut intersection = s.intersection(&set);
      if intersection.next().is_some() {
        s.extend(set.into_iter());
        return;
      }
    }

    self.append(set)
  }
  
  pub fn append_one(&mut self, value: T) {
    for s in self.0.iter_mut() {
      if s.contains(&value) {
        s.insert(value);
        return;
      }
    }

    self.append(HashSet::from([value]))
  }
}

impl<T> DisjunctSets<T> {
  pub unsafe fn new_unchecked(value: Vec<HashSet<T>>) -> Self {
    Self(value)
  }

  pub fn interior(&self) -> &Vec<HashSet<T>> {
    &self.0
  }

  pub fn unwrap(self) -> Vec<HashSet<T>> {
    self.0
  }
}

pub type DisjunctIdxs = DisjunctSets<usize>;


impl<T> Hash for DisjunctSets<T> where HashSet<T>: Hash {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<T> PartialEq for DisjunctSets<T> where HashSet<T>: PartialEq {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T> Eq for DisjunctSets<T> where HashSet<T>: Eq { }

impl<T> PartialOrd for DisjunctSets<T> where HashSet<T>: PartialOrd {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<T> Ord for DisjunctSets<T> where HashSet<T>: Ord {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.cmp(&other.0)
  }
}

impl<T> From<Vec<HashSet<T>>> for DisjunctSets<T> where T: Eq + Hash {
  fn from(value: Vec<HashSet<T>>) -> Self {
    Self::new(value)
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

