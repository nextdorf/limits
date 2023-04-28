use super::WrapperDeref;

#[derive(WrapperDeref, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Permutation<const N: usize>([usize; N]);


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermErr {
  TooShort(usize),
  TooLong,
  Repeating(usize),
  OutOfRange(usize),
}

impl<const N: usize> Permutation<N> {
  pub fn new(values: impl Iterator<Item = usize>) -> Self {
    match Self::new_wrapped(values) {
      Ok(res) => res,
      Err(PermErr::TooLong) => panic!("Given iterator is too long. It should have {N} elements"),
      Err(PermErr::TooShort(n)) => panic!("Given iterator has only {n} elements. It should have {N} elements"),
      Err(PermErr::Repeating(x)) => panic!("Value {x} is repeating"),
      Err(PermErr::OutOfRange(x)) => panic!("Value {x} is out of range. All values must be less than {N}"),
    }
  }
  pub fn new_wrapped(values: impl Iterator<Item = usize>) -> Result<Self, PermErr> {
    let mut inner_values = [0; N];
    let mut opt_values = [None; N];
    let mut len = 0;
    for val in values {
      if len > N {
        return Err(PermErr::TooLong);
      }
      if val >= N {
        return Err(PermErr::OutOfRange(val));
      }
      if let Some(x) = opt_values[val] {
        return Err(PermErr::Repeating(x));
      }
      opt_values[val] = Some(val);
      inner_values[len] = val;
      len += 1;
    }
    if len < N {
      return Err(PermErr::TooShort(len));
    }
    Ok(Self::new_no_checks(inner_values))
  }
  const fn new_no_checks(values: [usize; N]) -> Self {
    Self(values)
  }
  pub fn new_from_array(values: [usize; N]) -> Self {
    Self::new(values.into_iter())
  }
}


impl<const N: usize> super::GenGroup for Permutation<N> {
  fn ref_mult(&self, rhs: Self) -> Self {
    Self::new_no_checks(self.map(|x| rhs[x]))
  }

  fn ref_mult_ref(&self, rhs: &Self) -> Self {
    self.ref_mult(*rhs)
  }

  fn mult_assign(&mut self, rhs: Self) {
    let vals = &mut self.0;
    for x in vals {
      *x = rhs[*x];
    }
  }

  fn mult_assign_ref(&mut self, rhs: &Self) {
    self.mult_assign(*rhs)
  }

  fn inv(self) -> Self {
    let mut res = [0; N];
    for (i, x) in self.0.into_iter().enumerate() {
      res[x] = i;
    }
    Self::new_no_checks(res)
  }

  fn ref_inv(&self) -> Self {
    (*self).inv()
  }

  fn unit() -> Self {
    let mut res = Self::new_no_checks([0; N]);
    res.set_unit();
    res
  }

  fn set_unit(&mut self) {
    for (i, x) in self.0.iter_mut().enumerate() {
      *x = i
    }
  }

  fn is_unit(&self) -> bool {
    for (i, x) in self.iter().enumerate() {
      if *x != i {
        return false;
      }
    }
    return true;
  }
}


impl<const N: usize> TryFrom<[usize; N]> for Permutation<N> {
  type Error = PermErr;

  fn try_from(value: [usize; N]) -> Result<Self, PermErr> {
    Permutation::new_wrapped(value.into_iter())
  }
}

impl<const N: usize> From<&Permutation<N>> for [usize; N] {
  fn from(value: &Permutation<N>) -> Self {
    **value
  }
}

impl<const N: usize> From<super::Cyclic<N>> for Permutation<N> {
  fn from(value: super::Cyclic<N>) -> Self {
    let value = *value;
    let mut res = [0; N];
    for (i, x) in res.iter_mut().enumerate() {
      *x = (i + value) % N;
    }
    Self::new_no_checks(res)
  }
}


impl<const N: usize> ToString for Permutation<N> {
  fn to_string(&self) -> String {
    match N {
      0 => String::new() + "Perm[]",
      _ => {
        let mut vals = self.iter();
        let mut res = format!("Perm[{N} | {}", vals.next().unwrap());
        for v in vals {
          res += &format!(", {v}");
        }
        res + "]"
      }
    }
  }
}


impl<const N: usize> Default for Permutation<N> {
  fn default() -> Self {
    super::GenGroup::unit()
  }
}

#[cfg(test)]
mod tests {
  use num_traits::{Inv, One};
  use crate::group::Group;
  use super::Permutation;

  #[test]
  fn law() {
    fn inner<const N: usize>(a: [usize; N], b: [usize; N], c: [usize; N]) {
      let a = Group(Permutation::new_from_array(a));
      let b = Group(Permutation::new_from_array(b));
      let c = Group(Permutation::new_from_array(c));

      assert_eq!((a*b)*c, a*(b*c));
      assert_eq!(a, a.inv().inv());
      assert_eq!(a.inv(), Group::one() / a);
      assert_eq!(a.inv() * a, Group::one());
      assert_eq!(a / a, Group::one());
    }

    inner([], [], []);
    inner([0], [0], [0]);
    inner([0,1], [1,0], [0,1]);
    inner([3,2,1,0], [1,0,2,3], [2,1,3,0]);
  }
}
