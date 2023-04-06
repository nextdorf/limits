use num_complex::{Complex32, ComplexFloat, Complex64};
use num_traits::{real::Real, Zero};

use crate::forward;

use super::NumField;


#[macro_export]
macro_rules! real_num_field_impl {
  ($($t:ty)*) => ($(
    impl NumField for $t {
      type Real = Self;

      forward! {
        Real::min_positive_value() -> Self::Real;
        Real::epsilon() -> Self::Real;
      }
      
      forward! {
        Real::abs(self) -> Self::Real;
        Real::recip(self) -> Self;
        Real::powi(self, n: i32) -> Self;
        Real::powf(self, n: Self::Real) -> Self;
        Real::sqrt(self) -> Self;
        Real::exp(self) -> Self;
        Real::exp2(self) -> Self;
        Real::ln(self) -> Self;
        Real::log(self, base: Self::Real) -> Self;
        Real::log2(self) -> Self;
        Real::log10(self) -> Self;
        Real::cbrt(self) -> Self;
        Real::sin(self) -> Self;
        Real::cos(self) -> Self;
        Real::tan(self) -> Self;
        Real::asin(self) -> Self;
        Real::acos(self) -> Self;
        Real::atan(self) -> Self;
        Real::sin_cos(self) -> (Self, Self);
        Real::exp_m1(self) -> Self;
        Real::ln_1p(self) -> Self;
        Real::sinh(self) -> Self;
        Real::cosh(self) -> Self;
        Real::tanh(self) -> Self;
        Real::asinh(self) -> Self;
        Real::acosh(self) -> Self;
        Real::atanh(self) -> Self;
      }
      
      fn max_positive_value() -> Self::Real {
        Real::max_value()
      }
      
      fn phase(self) -> Self {
        Real::signum(self)
      }
      
      fn conj(self) -> Self {
        self
      }
    }
  )*)
}
real_num_field_impl! { f32 f64 }

#[macro_export]
macro_rules! complex_num_field_impl {
  ($($t:ty)*) => ($(
    impl NumField for $t {
      type Real = <Self as ComplexFloat>::Real;

      forward! {
        ComplexFloat::abs(self) -> Self::Real;
        ComplexFloat::recip(self) -> Self;
        ComplexFloat::powi(self, n: i32) -> Self;
        ComplexFloat::powf(self, n: Self::Real) -> Self;
        ComplexFloat::sqrt(self) -> Self;
        ComplexFloat::exp(self) -> Self;
        ComplexFloat::exp2(self) -> Self;
        ComplexFloat::ln(self) -> Self;
        ComplexFloat::log(self, base: Self::Real) -> Self;
        ComplexFloat::log2(self) -> Self;
        ComplexFloat::log10(self) -> Self;
        ComplexFloat::cbrt(self) -> Self;
        ComplexFloat::sin(self) -> Self;
        ComplexFloat::cos(self) -> Self;
        ComplexFloat::tan(self) -> Self;
        ComplexFloat::asin(self) -> Self;
        ComplexFloat::acos(self) -> Self;
        ComplexFloat::atan(self) -> Self;
        ComplexFloat::sinh(self) -> Self;
        ComplexFloat::cosh(self) -> Self;
        ComplexFloat::tanh(self) -> Self;
        ComplexFloat::asinh(self) -> Self;
        ComplexFloat::acosh(self) -> Self;
        ComplexFloat::atanh(self) -> Self;
        ComplexFloat::conj(self) -> Self;
      }


      fn min_positive_value() -> Self::Real {
        <Self::Real as NumField>::min_positive_value()
      }

      fn epsilon() -> Self::Real {
        <Self::Real as NumField>::epsilon()
      }

      fn max_positive_value() -> Self::Real {
        <Self::Real as NumField>::max_positive_value()
      }

      fn phase(self) -> Self {
        if self.is_zero() {
          Self::zero()
        } else {
          self / ComplexFloat::abs(self)
        }
      }

      fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
      }

      fn exp_m1(self) -> Self {
        // todo!();
        self.exp() - 1.
      }

      fn ln_1p(self) -> Self {
        // todo!();
        (1. + self).ln()
      }
    }
  )*)
}
complex_num_field_impl! { Complex32 Complex64 }

