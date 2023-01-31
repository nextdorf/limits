pub trait Index {}

pub trait MultiVar {
  type I: Index;
  type X;

  fn elem_at_index(&self, i: Self::I) -> Self::X;
}

pub trait DualMultiVar<Dual>: MultiVar {
  type DualI: Index;

  fn dot(&self, dual: Dual) -> Self::X;
  fn try_as_dual(&self) -> Option<Dual>;
}

pub trait SelfDualMultiVar: MultiVar {
  fn square_norm(&self, dual: &Self) -> Self::X;
}


impl<V> DualMultiVar<Self> for V where V: SelfDualMultiVar, Self: Clone {
  type DualI = Self::I;

  fn dot(&self, dual: &Self) -> Self::X {
    self.square_norm(dual)
  }

  fn try_as_dual(&self) -> Option<Self> {
    Some(self.clone())
  }

}


