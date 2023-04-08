use std::ops::{Neg, Add, Sub, Mul, Div, Deref, DerefMut};
use num_traits::{Zero, One};
use crate::{GenGroup, NumField, GenVectorSpace};


pub struct FiniteVS<const N: usize, T: GenVectorSpace>(pub [T; N]);


impl<const N: usize, T: NumField + GenVectorSpace> FiniteVS<N, T> {
  pub fn unit_vecs() -> [FiniteVS<N, T>; N] {
    let x0s = [<T as Zero>::zero(); N];
    let x1 = <T as One>::one();
    let mut k = 0;
    [(); N].map(|_| {
      let mut res = x0s;
      res[k] = x1;
      k += 1;
      Self(res)
    })
  }
}


impl<const N: usize, T: GenVectorSpace> Zero for FiniteVS<N, T> {
  fn zero() -> Self {
    fn get0<T: Zero>(_: ()) -> T {
      T::zero()
    }
    Self([();N].map(get0))
  }

  fn is_zero(&self) -> bool {
    for x in &self.0 {
      if x.is_zero() {
        return false;
      }
    }
    true
  }

  fn set_zero(&mut self) {
    for x in self.0.iter_mut() {
      x.set_zero();
    }
  }
}

impl<const N: usize, T: GenVectorSpace> Add<FiniteVS<N, T>> for FiniteVS<N, T> {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self {
    // for (x, y) in self.0.iter_mut().zip(rhs.0) {
    //   x.add_assign(y);
    // }
    // self
    self.add_assign(rhs);
    self
  }
}

impl<const N: usize, T: GenVectorSpace> Sub<FiniteVS<N, T>> for FiniteVS<N, T> {
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self {
    // for (x, y) in self.0.iter_mut().zip(rhs.0) {
    //   x.sub_assign(y);
    // }
    // self
    self.sub_assign(rhs);
    self
  }
}

impl<const N: usize, T: GenVectorSpace> Neg for FiniteVS<N, T> {
  type Output = Self;

  fn neg(mut self) -> Self {
    for x in self.0.iter_mut() {
      *x = x.ref_neg();
    }
    self
  }
}

impl<const N: usize, T: GenVectorSpace> GenGroup for FiniteVS<N, T> {
  fn ref_add(&self, mut rhs: Self) -> Self {
    rhs.add_assign_ref(self);
    rhs
  }

  fn add_ref(mut self, rhs: &Self) -> Self {
    self.add_assign_ref(rhs);
    self
  }

  fn ref_add_ref(&self, rhs: &Self) -> Self {
    let mut vals = self.0.iter().zip(&rhs.0);
    Self([(); N].map(|_| {
      let (x, y) = vals.next().unwrap();
      x.ref_add_ref(y)
    }))
  }

  fn ref_sub(&self, mut rhs: Self) -> Self {
    rhs.sub_assign_ref(self);
    rhs
  }

  fn sub_ref(mut self, rhs: &Self) -> Self {
    self.sub_assign_ref(rhs);
    self
  }

  fn ref_sub_ref(&self, rhs: &Self) -> Self {
    let mut vals = self.0.iter().zip(&rhs.0);
    Self([(); N].map(|_| {
      let (x, y) = vals.next().unwrap();
      x.ref_sub_ref(y)
    }))
  }

  fn ref_neg(&self) -> Self {
    let mut vals = self.0.iter();
    Self([(); N].map(|_| vals.next().unwrap().ref_neg()))
  }

  fn add_assign(&mut self, rhs: Self) {
    for (x, y) in self.0.iter_mut().zip(rhs.0) {
      x.add_assign(y);
    }
  }

  fn add_assign_ref(&mut self, rhs: &Self) {
    for (x, y) in self.0.iter_mut().zip(&rhs.0) {
      x.add_assign_ref(y);
    }
  }

  fn sub_assign(&mut self, rhs: Self) {
    for (x, y) in self.0.iter_mut().zip(rhs.0) {
      x.sub_assign(y);
    }
  }

  fn sub_assign_ref(&mut self, rhs: &Self) {
    for (x, y) in self.0.iter_mut().zip(&rhs.0) {
      x.sub_assign_ref(y);
    }
  }
}


impl<const N: usize, T: GenVectorSpace> Mul<T::Field> for FiniteVS<N, T> {
  type Output = Self;

  fn mul(mut self, rhs: T::Field) -> Self {
    // Self(self.0.map(|x| {
    //   x * rhs
    // }))
    self.mul_scalar_assign(rhs);
    self
  }
}

impl<const N: usize, T: GenVectorSpace> Div<T::Field> for FiniteVS<N, T> {
  type Output = Self;

  fn div(mut self, rhs: T::Field) -> Self {
    // Self(self.0.map(|x| {
    //   x / rhs
    // }))
    self.div_scalar_assign(rhs);
    self
  }
}

impl<const N: usize, T: GenVectorSpace> GenVectorSpace for FiniteVS<N, T> {
  type Field = T::Field;

  fn mul_scalar_assign(&mut self, rhs: Self::Field) {
    // *self = std::mem::replace(self, Self::zero()).mul(rhs)
    for x in self.0.iter_mut() {
      x.mul_scalar_assign(rhs);
    }
  }

  fn div_scalar_assign(&mut self, rhs: Self::Field) {
    // *self = std::mem::replace(self, Self::zero()).div(rhs)
    for x in self.0.iter_mut() {
      x.div_scalar_assign(rhs);
    }
  }
  
}


impl<const N: usize, T: GenVectorSpace> Deref for FiniteVS<N, T> {
  type Target = [T; N];
  
  fn deref(&self) -> &[T; N] {
    &self.0
  }
}

impl<const N: usize, T: GenVectorSpace> DerefMut for FiniteVS<N, T> {
  fn deref_mut(&mut self) -> &mut [T; N] {
    &mut self.0
  }
}


// pub type Vec0D<T> = VectorSpace<FiniteVS<0, T>>;
// pub type Vec1D<T> = VectorSpace<FiniteVS<1, T>>;
// pub type Vec2D<T> = VectorSpace<FiniteVS<2, T>>;
// pub type Vec3D<T> = VectorSpace<FiniteVS<3, T>>;
// pub type Vec4D<T> = VectorSpace<FiniteVS<4, T>>;
// pub type Vec5D<T> = VectorSpace<FiniteVS<5, T>>;
// pub type Vec6D<T> = VectorSpace<FiniteVS<6, T>>;
// pub type Vec7D<T> = VectorSpace<FiniteVS<7, T>>;
// pub type Vec8D<T> = VectorSpace<FiniteVS<8, T>>;


impl<const N: usize, T: GenVectorSpace> From<[T; N]> for FiniteVS<N, T> {
  fn from(value: [T; N]) -> Self {
    Self(value)
  }
}

