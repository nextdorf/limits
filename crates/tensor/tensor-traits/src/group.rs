pub trait GenGroup<Kind = ()>: Sized {
  fn mult(mut self, rhs: Self) -> Self {
    self.mult_assign(rhs);
    self
  }
  fn ref_mult(&self, rhs: Self) -> Self;
  fn mult_ref(mut self, rhs: &Self) -> Self {
    self.mult_assign_ref(rhs);
    self
  }
  fn ref_mult_ref(&self, rhs: &Self) -> Self;
  fn mult_assign(&mut self, rhs: Self);
  fn mult_assign_ref(&mut self, rhs: &Self);

  fn mult_inv(mut self, rhs: Self) -> Self {
    self.mult_assign_inv(rhs);
    self
  }
  fn ref_mult_inv(&self, rhs: Self) -> Self {
    self.ref_mult(rhs.inv())
  }
  fn mult_inv_ref(mut self, rhs: &Self) -> Self {
    self.mult_assign_inv_ref(rhs);
    self
  }
  fn ref_mult_inv_ref(&self, rhs: &Self) -> Self {
    self.ref_mult_ref(&rhs.ref_inv())
  }
  fn mult_assign_inv(&mut self, rhs: Self) {
    self.mult_assign(rhs.inv())
  }
  fn mult_assign_inv_ref(&mut self, rhs: &Self) {
    self.mult_assign_ref(&rhs.ref_inv())
  }

  fn inv(self) -> Self;
  fn ref_inv(&self) -> Self;

  fn unit() -> Self;
  fn set_unit(&mut self) {
    *self = Self::unit();
  }
  fn is_unit(&self) -> bool;
}


pub trait GenAbelGroup<Kind = ()>: GenGroup<Kind> {}



