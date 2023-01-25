pub trait Index {}

pub trait MultiVar {
  type I: Index;
  type X;

  fn elem_at_index(&self, i: Self::I) -> &Self::X;
}

pub trait DualMultiVar<Dual>: MultiVar {
  type DualI;

  fn dot(&self, dual: &Dual) -> Self::X;
  fn as_dual(self) -> Dual;
}

pub trait SelfDualMultiVar: MultiVar {
  fn square_norm(&self, dual: &Self) -> Self::X;
}


impl<V> DualMultiVar<Self> for V where V: SelfDualMultiVar {
  type DualI = Self::I;

  fn dot(&self, dual: &Self) -> Self::X {
    self.square_norm(dual)
  }

  fn as_dual(self) -> Self {
    self
  }

}


