use super::WrapperDeref;

#[derive(WrapperDeref, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cyclic<const N: usize>(usize);


impl<const N: usize> Cyclic<N> {
  pub const fn new(value: usize) -> Self {
    assert!(N > 0);
    Self::new_no_assert(value)
  }
  const fn new_no_assert(value: usize) -> Self {
    Self(value % N)
  }
}


impl<const N: usize> super::GenGroup for Cyclic<N> {
  fn mult(self, rhs: Self) -> Self {
    Self::new_no_assert(self.0 + rhs.0)
  }
  fn ref_mult(&self, rhs: Self) -> Self {
    Self::mult(*self, rhs)
  }
  fn mult_ref(self, rhs: &Self) -> Self {
    Self::mult(self, *rhs)
  }
  fn ref_mult_ref(&self, rhs: &Self) -> Self {
    Self::mult(*self, *rhs)
  }
  fn mult_assign(&mut self, rhs: Self) {
    *self = Self::mult(*self, rhs)
  }
  fn mult_assign_ref(&mut self, rhs: &Self) {
    *self = Self::mult(*self, *rhs)
  }

  fn mult_inv(self, rhs: Self) -> Self {
    Self::new_no_assert(N + self.0 - rhs.0)
  }
  fn ref_mult_inv(&self, rhs: Self) -> Self {
    Self::mult_inv(*self, rhs)
  }
  fn mult_inv_ref(self, rhs: &Self) -> Self {
    Self::mult_inv(self, *rhs)
  }
  fn ref_mult_inv_ref(&self, rhs: &Self) -> Self {
    Self::mult_inv(*self, *rhs)
  }
  fn mult_assign_inv(&mut self, rhs: Self) {
    *self = Self::mult_inv(*self, rhs)
  }
  fn mult_assign_inv_ref(&mut self, rhs: &Self) {
    *self = Self::mult_inv(*self, *rhs)
  }


  fn inv(self) -> Self {
    match self.0 {
      0 => Self(0),
      n => Self(N-n),
    }
  }
  fn ref_inv(&self) -> Self {
    Self::inv(*self)
  }

  fn unit() -> Self {
    Self(0)
  }
  fn is_unit(&self) -> bool {
    self.0 == 0
  }
}

impl<const N: usize> super::GenAbelGroup for Cyclic<N> {}


impl<const N: usize> From<usize> for Cyclic<N> {
  fn from(value: usize) -> Self {
    Self::new(value)
  }
}

impl<const N: usize> From<&Cyclic<N>> for usize {
  fn from(value: &Cyclic<N>) -> Self {
    **value
  }
}

impl<const N: usize> From<Cyclic<N>> for usize {
  fn from(value: Cyclic<N>) -> Self {
    *value
  }
}


impl<const N: usize> ToString for Cyclic<N> {
  fn to_string(&self) -> String {
    format!("({} mod {N})", self.0)
  }
}


#[cfg(test)]
mod tests {
  use num_traits::Zero;
  use crate::group::AbelGroup;
  use super::Cyclic;

  #[test]
  fn cyclic_law() {
    fn inner<const N: usize>(a: usize, b: usize, c: usize) {
      let a: AbelGroup<Cyclic<N>> = AbelGroup::from(a);
      let b: AbelGroup<Cyclic<N>> = AbelGroup::from(b);
      let c: AbelGroup<Cyclic<N>> = AbelGroup::from(c);

      assert_eq!((a+b)+c, a+(b+c));
      assert_eq!(a + b, b + a);
      assert_eq!(a, -(-a));
      assert_eq!(-a, AbelGroup::zero() - a);
      assert_eq!((-a) + a, AbelGroup::zero());
      assert_eq!(a - a, AbelGroup::zero());
    }

    let xs = [0, 1, 2, 3, 30, 50, 100, 200, 1000].into_iter();
    let ys = [3, 7].into_iter().cycle();
    let zs = [81].into_iter().cycle();
    for ((a, b), c) in xs.zip(ys).zip(zs) {
      inner::<1>(a, b, c);
      inner::<2>(a, b, c);
      inner::<3>(a, b, c);
      inner::<5>(a, b, c);
      inner::<100>(a, b, c);
      inner::<501>(a, b, c);
    }

  }

  #[test]
  #[should_panic(expected = "assertion failed: N > 0")]
  fn no_empty_cyclic_group() {
    Cyclic::<0>::new(1);
  }
}

