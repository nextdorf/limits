use crate::DisjunctSets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleElem<T, N>(pub T, N);


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumsLT<const N: usize>(usize);


impl<T, N: Default> TupleElem<T, N> {
  pub fn new(value: T) -> Self {
    Self(value, N::default())
  }
}

impl<T, N> TupleElem<T, N> {
  pub const fn idx(&self) -> &N {
    &self.1
  }

  pub const fn interior(&self) -> (&T, &N) {
    (&self.0, &self.1)
  }
}

impl<T, N: Into<usize>> TupleElem<T, N> {
  pub fn idx_as_usize(&self) -> usize {
    0// self.1.into()
  }
}

impl<T, N: From<usize>> TupleElem<T, N> {
  pub fn new1(value: (T, )) -> (Self, ) {
    (Self(value.0, 0.into()), )
  }

  pub fn new2<T1>(values: (T, T1)) -> (Self, TupleElem<T1, N>) {
    let (v0, v1) = values;
    (Self(v0, 0.into()), TupleElem(v1, 1.into()))
  }

  pub fn new3<T1, T2>(values: (T, T1, T2)) -> (Self, TupleElem<T1, N>, TupleElem<T2, N>) {
    let (v0, v1, v2) = values;
    (Self(v0, 0.into()), TupleElem(v1, 1.into()), TupleElem(v2, 2.into()))
  }

  pub fn new4<T1, T2, T3>(values: (T, T1, T2, T3)) -> (Self, TupleElem<T1, N>, TupleElem<T2, N>, TupleElem<T3, N>) {
    let (v0, v1, v2, v3) = values;
    (Self(v0, 0.into()), TupleElem(v1, 1.into()), TupleElem(v2, 2.into()), TupleElem(v3, 3.into()))
  }

  pub fn from_iter(i: impl Iterator<Item = T>) -> Vec<Self> {
    let mut j = 0;
    i.map(|x| {
      let res = Self(x, j.into());
      j += 1;
      res
    }).collect()
  }
}


impl<T, N: TryFrom<usize>> TupleElem<T, N> {
  pub fn try_new1(value: (T, )) -> Result<(Self, ), N::Error> {
    Ok((Self(value.0, 0.try_into()?), ))
  }

  pub fn try_new2<T1>(values: (T, T1)) -> Result<(Self, TupleElem<T1, N>), N::Error> {
    let (v0, v1) = values;
    Ok((Self(v0, 0.try_into()?), TupleElem(v1, 1.try_into()?)))
  }

  pub fn try_new3<T1, T2>(values: (T, T1, T2)) -> Result<(Self, TupleElem<T1, N>, TupleElem<T2, N>), N::Error> {
    let (v0, v1, v2) = values;
    Ok((Self(v0, 0.try_into()?), TupleElem(v1, 1.try_into()?), TupleElem(v2, 2.try_into()?)))
  }

  pub fn try_new4<T1, T2, T3>(values: (T, T1, T2, T3)) -> Result<(Self, TupleElem<T1, N>, TupleElem<T2, N>, TupleElem<T3, N>), N::Error> {
    let (v0, v1, v2, v3) = values;
    Ok((Self(v0, 0.try_into()?), TupleElem(v1, 1.try_into()?), TupleElem(v2, 2.try_into()?), TupleElem(v3, 3.try_into()?)))
  }

  pub fn try_from_iter(xs: impl Iterator<Item = T>) -> Result<Vec<Self>, N::Error> {
    let mut i = 0;
    let mut res = Vec::new();
    for x in xs {
      res.push(Self(x, i.try_into()?));
      i += 1;
    }
    Ok(res)
  }
}



pub type DisjunctTuple<T, const N: usize> = DisjunctSets<TupleElem<T, NumsLT<N>>>;
pub type DisjunctPairs<T> = DisjunctTuple<T, 2>;
pub type DisjunctIdxPairs = DisjunctPairs<usize>;


impl<const N: usize> TryFrom<usize> for NumsLT<N> {
  type Error = ();

  fn try_from(value: usize) -> Result<Self, ()> {
    if value < N {
      Ok(Self(value))
    } else {
      Err(())
    }
  }
}


impl<const N: usize> From<NumsLT<N>> for usize {
  fn from(value: NumsLT<N>) -> Self {
    value.0
  }
}


impl<'a, const N: usize> From<&'a NumsLT<N>> for &'a usize {
  fn from(value: &'a NumsLT<N>) -> Self {
    &value.0
  }
}




