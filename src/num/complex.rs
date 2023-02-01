use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg}, fmt::Display, num::FpCategory};

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct c128(pub f64, pub f64);


pub const COMPLEX_0: c128 = c128(0., 0.);
pub const COMPLEX_1: c128 = c128(1., 0.);
pub const COMPLEX_I: c128 = c128(0., 1.);


impl c128 {
  pub const fn i() -> Self {
    COMPLEX_I
  }

  pub const fn re(self) -> f64 {
    self.0
  }

  pub const fn im(self) -> f64 {
    self.1
  }

  pub fn angle(self) -> f64 {
    self.1.atan2(self.0)
  }

  pub fn conj(self) -> Self {
    c128(self.0, -self.1)
  }

  
  pub fn abs_squared(self) -> f64 {
    self.0*self.0 + self.1*self.1
  }

  pub fn abs(self) -> f64 {
    self.abs_squared().sqrt()
  }

  pub fn exp(self) -> c128 {
    let e_x = self.0.exp();
    let (siny, cosy) = self.1.sin_cos();
    c128(e_x*cosy, e_x*siny)
  }

  pub fn powi(self, n: i32) -> c128 {
    match n {
      -1 => self.recip(),
      0 => COMPLEX_1,
      1 => self,
      _ => self.abs().powi(n) * ((n as f64) * self.angle() * COMPLEX_I).exp(),
    }
  }

  pub fn powf(self, n: f64) -> c128 {
    self.abs().powf(n) * (n * self.angle() * COMPLEX_I).exp()
  }

  pub fn powc(self, n: c128) -> c128 {
    let ln_r_n = self.abs_squared().ln() / 2. * n;
    let iphi_n = n * self.angle() * COMPLEX_I;
    (ln_r_n + iphi_n).exp()
  }

  fn sqrt_if_y_geq_0(self) -> c128 {
    // Solve x+iy = (a+ib)^2 with a >= 0. Define for y >= 0 and enforce conj(sqrt(conj(z))) = sqrt(z)
    debug_assert!(self.1 >= 0.);
    let x2 = self.0 / 2.;
    let y4 = self.1 / 4.;
    let a = (x2 + (x2*x2 + y4).sqrt()).sqrt();
    c128(a, self.1 / (2. * a))
  }

  pub fn sqrt(self) -> c128 {
    // Solve x+iy = (a+ib)^2 with a >= 0. Define for y >= 0 and enforce conj(sqrt(conj(z))) = sqrt(z)
    if self.1 >= 0. {
      self.sqrt_if_y_geq_0()
    } else {
      self.conj().sqrt_if_y_geq_0().conj()
    }
  }

  
  pub fn recip(self) -> c128 {
    let abs_squared = self.abs_squared();
    Self(self.0 / abs_squared, -self.1 / abs_squared)
  }


  pub fn ln(self) -> c128 {
    c128(self.abs_squared().ln() / 2., self.angle())
  }

  pub fn sin(self) -> c128 {
    self.sin_cos().0
  }

  pub fn cos(self) -> c128 {
    self.sin_cos().1
  }

  pub fn tan(self) -> c128 {
    // self.sin().div(self.cos())
    -COMPLEX_I * (COMPLEX_I*self).tanh()
  }

  pub fn asin(self) -> c128 {
    -COMPLEX_I * ((1. - self*self).sqrt() + COMPLEX_I*self).ln()
  }

  pub fn acos(self) -> c128 {
    -COMPLEX_I * (self + COMPLEX_I*(1. - self*self).sqrt()).ln()
  }

  pub fn atan(self) -> c128 {
    // (COMPLEX_1 + self*self).sqrt().reciprocal().acos()
    -COMPLEX_I * (COMPLEX_I * self).atanh()
  }

  pub fn sin_cos(self) -> (c128, c128) {
    let (sinx, cosx) = self.0.sin_cos();
    let (sinhy, coshy) = (self.0.sinh(), self.0.cosh());
    let sinz = c128(sinx * coshy, cosx * sinhy);
    let cosz = c128(cosx * coshy, -sinx * sinhy);
    (sinz, cosz)
  }

  pub fn sinh(self) -> c128 {
    -COMPLEX_I * (-COMPLEX_I*self).sin()
  }

  pub fn cosh(self) -> c128 {
    (-COMPLEX_I*self).sin()
  }

  fn tanh_if_x_leq_0(self) -> c128 {
    // f64::exp(self)
    let e_2x = (2.*self.0).exp();
    let (sin2y, cos2y) = (2.*self.1).sin_cos();
    let e_2z = c128(e_2x*cos2y, e_2x*sin2y);
    (e_2z - 1.) / (e_2z + 1.)
  }

  pub fn tanh(self) -> c128 {
    if self.0 <= 0. {
      self.tanh_if_x_leq_0()
    } else {
      -(-self).tanh_if_x_leq_0()
    }
  }

  pub fn asinh(self) -> c128 {
    COMPLEX_I * (-COMPLEX_I * self).asin()
  }

  pub fn acosh(self) -> c128 {
    COMPLEX_I * self.acos()
  }

  pub fn atanh(self) -> c128 {
    ((1. + self) / (1. - self)).ln() / 2.
  }
}


impl Add<c128> for c128 {
  type Output = c128;

  fn add(self, rhs: c128) -> c128 {
    c128(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl AddAssign<c128> for c128 {
  fn add_assign(&mut self, rhs: c128) {
    self.0 += rhs.0;
    self.1 += rhs.1;
  }
}


impl Add<f64> for c128 {
  type Output = c128;

  fn add(self, rhs: f64) -> c128 {
    c128(self.0 + rhs, self.1)
  }
}
impl Add<c128> for f64 {
  type Output = c128;

  fn add(self, rhs: c128) -> c128 {
    rhs + self
  }
}

impl AddAssign<f64> for c128 {
  fn add_assign(&mut self, rhs: f64) {
    self.0 += rhs;
  }
}


impl Sub<c128> for c128 {
  type Output = c128;

  fn sub(self, rhs: c128) -> c128 {
    c128(self.0 - rhs.0, self.1 - rhs.1)
  }
}

impl SubAssign<c128> for c128 {
  fn sub_assign(&mut self, rhs: c128) {
    self.0 -= rhs.0;
    self.1 -= rhs.1;
  }
}


impl Sub<f64> for c128 {
  type Output = c128;

  fn sub(self, rhs: f64) -> c128 {
    c128(self.0 - rhs, self.1)
  }
}
impl Sub<c128> for f64 {
  type Output = c128;

  fn sub(self, rhs: c128) -> c128 {
    (-rhs) + self
  }
}

impl SubAssign<f64> for c128 {
  fn sub_assign(&mut self, rhs: f64) {
    self.0 -= rhs;
  }
}


impl Mul<c128> for c128 {
  type Output = c128;

  fn mul(self, rhs: c128) -> c128 {
    c128(self.0*rhs.0 - self.1*rhs.1, self.0*rhs.1 + self.1*rhs.0)
  }
}

impl MulAssign<c128> for c128 {
  fn mul_assign(&mut self, rhs: c128) {
    *self = *self * rhs
  }
}


impl Mul<f64> for c128 {
  type Output = c128;

  fn mul(self, rhs: f64) -> c128 {
    c128(self.0*rhs, self.1*rhs)
  }
}
impl Mul<c128> for f64 {
  type Output = c128;

  fn mul(self, rhs: c128) -> c128 {
    rhs * self
  }
}

impl MulAssign<f64> for c128 {
  fn mul_assign(&mut self, rhs: f64) {
    self.0 *= rhs;
    self.1 *= rhs;
  }
}


impl Div<c128> for c128 {
  type Output = c128;

  fn div(self, rhs: c128) -> c128 {
    self * rhs.recip()
  }
}

impl DivAssign<c128> for c128 {
  fn div_assign(&mut self, rhs: c128) {
    *self = *self / rhs
  }
}


impl Div<f64> for c128 {
  type Output = c128;

  fn div(self, rhs: f64) -> c128 {
    self * rhs.recip()
  }
}
impl Div<c128> for f64 {
  type Output = c128;

  fn div(self, rhs: c128) -> c128 {
    rhs.recip() * self
  }
}

impl DivAssign<f64> for c128 {
  fn div_assign(&mut self, rhs: f64) {
    self.0 /= rhs;
    self.1 /= rhs;
  }
}


impl Neg for c128 {
  type Output = c128;

  fn neg(self) -> c128 { 
    Self(-self.0, -self.1)
  }
}


impl From<f64> for c128 {
  fn from(value: f64) -> Self {
    Self(value, 0.)
  }
}

pub struct NonRealNumber(c128);
impl TryFrom<c128> for f64 {
  type Error = NonRealNumber;

  fn try_from(value: c128) -> Result<Self, NonRealNumber> {
    match value.1.classify() {
      std::num::FpCategory::Zero | std::num::FpCategory::Subnormal => 
        Ok(value.0),
      _ => Err(NonRealNumber(value))
    }
  }
}


impl Display for c128 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match (self.0.classify(), self.1.classify()) {
      (_, FpCategory::Zero | FpCategory::Subnormal) => write!(f, "{}", self.0),
      (FpCategory::Zero, _) => match (self.1.abs() - 1.).classify() {
        FpCategory::Zero | FpCategory::Subnormal => {
          f.write_str(if self.1 > 0. {"i"} else {"-i"})
        },
        _ => write!(f, "{}i", self.1),
      },
      _ => match (self.1.abs() - 1.).classify() {
        FpCategory::Zero | FpCategory::Subnormal => {
          write!(f, "{}{}", self.0, if self.1 > 0. {"+i"} else {"-i"})
        },
        _ => write!(f, "{}{}{}i", self.0, if self.1 < 0. {'-'} else {'+'}, self.1.abs())
      }
    }
  }
}

