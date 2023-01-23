pub enum Set<N: NonEmptySet> {
  Empty,
  NEmpty(N),
}

pub trait NonEmptySet : Eq + Sized {
  fn disjoint_elem() -> Set<Self>;
}



