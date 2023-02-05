/// Marker trait for indecces
pub trait Index: Copy { }

/// Specifies a `CoIndex` for an index. If elements from some vector space V are represented as
/// a function from the `Index` set to the number field, then elements of V^† are representated
/// as a function from the `CoIndex` set to the number field. Mapping the vector to the field
/// using the dual-vector is then done by contracting the vectors.
pub trait DualIndex: Index {
  /// Covariant index / Index for some dual space
  type CoIndex: Index;

  fn to_covariant_index(self) -> Self::CoIndex;
  fn from_covariant_index(co_idx: Self::CoIndex) -> Self;
}

/// Marker trait for providing a blanket type for dual indecces incase `CoIndex == Self`
pub trait SelfDualIndex: Index { }


/// Representation for a tensor index. Every tensor is assumed to own an element implementing
/// this trait and it specifies how many indecees the tensor has and which ones are raised and which
/// one are lowered
pub trait TensorIndexRepr: Clone {
  /// Lower the ith index. If `Index != CoIndex` the index will be a covariant index afterwards
  fn lower(self, i: usize) -> Self;
  /// Raise the ith index. If `Index != CoIndex` the index will be a contravariant index afterwards
  fn raise(self, i: usize) -> Self;
  /// `(M, N)` for an (M, N)-Tensor
  fn rank(&self) -> (usize, usize);
  /// `M` for an (M, N)-Tensor
  fn contravariant_rank(&self) -> usize { self.rank().0 }
  /// `N` for an (M, N)-Tensor
  fn covariant_rank(&self) -> usize { self.rank().1 }
  /// `M + N` for an (M, N)-Tensor
  fn order(&self) -> usize {
    let (a, b) = self.rank();
    a + b
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents the sum type `Index (+) CoIndex`
pub enum ValuedTensorIndexVariant<I: DualIndex> {
  /// Contravariant Index
  Contra(I),
  /// Covariant Index
  Co(I::CoIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Distinguishes between a contravariant and a covariant index
pub enum TensorIndexVariant {
  /// Contravariant Index
  Contra,
  /// Covariant Index
  Co,
}


#[derive(Default, Clone)]
pub struct TensorIndex<I: DualIndex>(pub Vec<ValuedTensorIndexVariant<I>>);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TensorIndexKind(pub Vec<TensorIndexVariant>);

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct TensorIndex2<R1: TensorIndexRepr, R2: TensorIndexRepr>(pub R1, pub R2);


impl Index for () {}
impl DualIndex for () {
  type CoIndex = ();
  fn to_covariant_index(self) { }
  fn from_covariant_index(co_idx: ()) { }
}


impl<I: DualIndex> TensorIndexRepr for ValuedTensorIndexVariant<I> {
  fn lower(self, i: usize) -> Self { assert_eq!(i, 0); self.lower() }

  fn raise(self, i: usize) -> Self { assert_eq!(i, 0); self.raise() }

  fn rank(&self) -> (usize, usize) {
    match self {
      Self::Contra(_) => (1, 0),
      Self::Co(_) => (0, 1),
    }
  }

  fn order(&self) -> usize { 1 }
}
impl<I: DualIndex> ValuedTensorIndexVariant<I> {
  pub fn lower(self) -> Self {
    if let Self::Contra(i) = self {
      Self::Co(i.to_covariant_index())
    } else {
      self
    }
  }
  
  pub fn raise(self) -> Self {
    if let Self::Co(i) = self {
      Self::Contra(I::from_covariant_index(i))
    } else {
      self
    }
  }
  
  pub fn flip(self) -> Self {
    match self {
      Self::Contra(c) => Self::Co(c.to_covariant_index()),
      Self::Co(c) => Self::Contra(I::from_covariant_index(c)),
    }
  }
}


impl<I> TensorIndexRepr for TensorIndex<I> where I: DualIndex + Copy {
  fn lower(mut self, i: usize) -> Self {
    self.0[i] = self.0[i].lower();
    self
  }

  fn raise(mut self, i: usize) -> Self {
    self.0[i] = self.0[i].raise();
    self
  }

  fn rank(&self) -> (usize, usize) {
    let (mut contra, mut co) = (0, 0);
    for variant in self.0.iter() {
      match variant {
        ValuedTensorIndexVariant::Contra(_) => contra += 1,
        ValuedTensorIndexVariant::Co(_) => co += 1,
      }
    }
    (contra, co)
  }

  fn order(&self) -> usize {
    self.0.len()
  }
}


impl<R1, R2> TensorIndexRepr for (R1, R2) where R1: TensorIndexRepr, R2: TensorIndexRepr {
  fn lower(self, i: usize) -> Self {
    let len0 = self.0.order();
    if i < len0 {
      (self.0.lower(i), self.1)
    } else {
      (self.0, self.1.lower(i - len0))
    }
  }

  fn raise(self, i: usize) -> Self {
    let len0 = self.0.order();
    if i < len0 {
      (self.0.raise(i), self.1)
    } else {
      (self.0, self.1.raise(i - len0))
    }
  }

  fn contravariant_rank(&self) -> usize { self.0.contravariant_rank() + self.1.contravariant_rank() }

  fn covariant_rank(&self) -> usize { self.0.covariant_rank() + self.1.covariant_rank() }

  fn order(&self) -> usize { self.0.order() + self.1.order() }

  fn rank(&self) -> (usize, usize) {
    let (a0, b0) = self.0.rank();
    let (a1, b1) = self.1.rank();
    (a0 + a1, b0 + b1)
  }
}


impl<R0, R1, R2> TensorIndexRepr for (R0, R1, R2) where R0: TensorIndexRepr, R1: TensorIndexRepr, R2: TensorIndexRepr {
  fn lower(self, i: usize) -> Self {
    let mut len0 = 0;
    let mut len1 = 0;
    len1 += self.0.order();
    if i < len1 {
      (self.0.lower(i - len0), self.1, self.2)
    } else {
      len0 = len1;
      len1 += self.1.order();
      if i < len1 {
        (self.0, self.1.lower(i - len0), self.2)
      } else {
        (self.0, self.1, self.2.lower(i - len1))
      }
    }
  }

  fn raise(self, i: usize) -> Self {
    let mut len0 = 0;
    let mut len1 = 0;
    len1 += self.0.order();
    if i < len1 {
      (self.0.raise(i - len0), self.1, self.2)
    } else {
      len0 = len1;
      len1 += self.1.order();
      if i < len1 {
        (self.0, self.1.raise(i - len0), self.2)
      } else {
        (self.0, self.1, self.2.raise(i - len1))
      }
    }
  }

  fn contravariant_rank(&self) -> usize {
    self.0.contravariant_rank() + self.1.contravariant_rank() + self.2.contravariant_rank()
  }

  fn covariant_rank(&self) -> usize {
    self.0.covariant_rank() + self.1.covariant_rank() + self.2.covariant_rank()
  }

  fn order(&self) -> usize {
    self.0.order() + self.1.order() + self.2.order()
  }

  fn rank(&self) -> (usize, usize) {
    let (a0, b0) = self.0.rank();
    let (a1, b1) = self.1.rank();
    let (a2, b2) = self.2.rank();
    (a0 + a1 + a2, b0 + b1 + b2)
    // vec![]
  }
}


impl<I> DualIndex for I where I: SelfDualIndex {
  type CoIndex = Self;

  fn to_covariant_index(self) -> Self { self }
  fn from_covariant_index(co_idx: Self) -> Self { co_idx }
}



impl<I: DualIndex + Default> Default for ValuedTensorIndexVariant<I> {
  fn default() -> Self {
    Self::Contra(I::default())
  }
}


impl<I: DualIndex> From<ValuedTensorIndexVariant<I>> for TensorIndexVariant {
  fn from(value: ValuedTensorIndexVariant<I>) -> Self {
    match value {
      ValuedTensorIndexVariant::Contra(_) => Self::Contra,
      ValuedTensorIndexVariant::Co(_) => Self::Co,
    }
  }
}


