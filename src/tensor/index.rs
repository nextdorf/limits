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


pub trait TensorIndexRepr: Clone {
  fn lower(self, i: usize) -> Self;
  fn raise(self, i: usize) -> Self;
  fn rank(&self) -> (usize, usize);
  fn contravariant_rank(&self) -> usize { self.rank().0 }
  fn covariant_rank(&self) -> usize { self.rank().1 }
  fn order(&self) -> usize {
    let (a, b) = self.rank();
    a + b
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TensorIndexVariant<I: DualIndex> {
  Contra(I),
  Co(I::CoIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TensorIndexVariantKind {
  Contra,
  Co,
}


#[derive(Default, Clone)]
pub struct TensorIndex<I: DualIndex>(pub Vec<TensorIndexVariant<I>>);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TensorIndexKind(pub Vec<TensorIndexVariantKind>);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TensorIndex2<R1: TensorIndexRepr, R2: TensorIndexRepr>(pub R1, pub R2);


impl<I: DualIndex> TensorIndexRepr for TensorIndexVariant<I> {
  fn lower(self, i: usize) -> Self { assert_eq!(i, 0); self.lower() }

  fn raise(self, i: usize) -> Self { assert_eq!(i, 0); self.raise() }

  fn rank(&self) -> (usize, usize) {
    match self {
      TensorIndexVariant::Contra(_) => (1, 0),
      TensorIndexVariant::Co(_) => (0, 1),
    }
  }

  fn order(&self) -> usize { 1 }
}


impl<I> TensorIndexRepr for TensorIndex<I> where I: DualIndex + Copy {
  fn lower(mut self, i: usize) -> Self {
    self.0[i] = self.0[i].lower();
    self
  }

  fn raise(self, i: usize) -> Self {
    self.0[i] = self.0[i].raise();
    self
  }

  fn rank(&self) -> (usize, usize) {
    let (mut contra, mut co) = (0, 0);
    for variant in self.0.iter() {
      match variant {
        TensorIndexVariant::Contra(_) => contra += 1,
        TensorIndexVariant::Co(_) => co += 1,
      }
    }
    (contra, co)
  }

  fn order(&self) -> usize {
    self.0.len()
  }
}


impl<R1, R2> TensorIndexRepr for TensorIndex2<R1, R2> where R1: TensorIndexRepr, R2: TensorIndexRepr {
  fn lower(self, i: usize) -> Self {
    let len0 = self.0.order();
    if i < len0 {
      Self(self.0.lower(i), self.1)
    } else {
      Self(self.0, self.1.lower(i - len0))
    }
  }

  fn raise(self, i: usize) -> Self {
    let len0 = self.0.order();
    if i < len0 {
      Self(self.0.raise(i), self.1)
    } else {
      Self(self.0, self.1.raise(i - len0))
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


impl<I: DualIndex> TensorIndexVariant<I> {
  pub fn lower(self) -> Self {
    if let TensorIndexVariant::Contra(i) = self {
      TensorIndexVariant::Co(i.to_covariant_index())
    } else {
      self
    }
  }
  
  pub fn raise(self) -> Self {
    if let TensorIndexVariant::Co(i) = self {
      TensorIndexVariant::Contra(I::from_covariant_index(i))
    } else {
      self
    }
  }
  
  pub fn flip(self) -> Self {
    match self {
      TensorIndexVariant::Contra(c) => TensorIndexVariant::Co(c.to_covariant_index()),
      TensorIndexVariant::Co(c) => TensorIndexVariant::Contra(I::from_covariant_index(c)),
    }
  }
}


impl<I> DualIndex for I where I: SelfDualIndex {
  type CoIndex = Self;

  fn to_covariant_index(self) -> Self { self }
  fn from_covariant_index(co_idx: Self) -> Self { co_idx }
}



impl<I: DualIndex + Default> Default for TensorIndexVariant<I> {
  fn default() -> Self {
    Self::Contra(I::default())
  }
}


impl<I: DualIndex> From<TensorIndexVariant<I>> for TensorIndexVariantKind {
  fn from(value: TensorIndexVariant<I>) -> Self {
    match value {
      TensorIndexVariant::Contra(_) => TensorIndexVariantKind::Contra,
      TensorIndexVariant::Co(_) => TensorIndexVariantKind::Co,
    }
  }
}

