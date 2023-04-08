mod field_impl;
use num_traits::{One, NumOps, real::Real};
use crate::GenGroup;


pub trait NumField: Copy + One + NumOps + GenGroup + From<Self::Real> {
  type Real: Real;

  /// Returns the smallest positive, normalized value that this type can represent.
  fn min_positive_value() -> Self::Real;

  /// Returns epsilon, a small positive value.
  fn epsilon() -> Self::Real;

  /// Returns the largest finite value that this type can represent.
  fn max_positive_value() -> Self::Real;

  /// Computes the absolute value of `self`. Returns `Float::nan()` if the
  fn abs(self) -> Self::Real;

  /// Returns itself if zero, otherwise it returns self/|self|.
  fn phase(self) -> Self;

  /// Take the reciprocal (inverse) of a number, `1/x`.
  fn recip(self) -> Self;

  /// Raise a number to an integer power.
  fn powi(self, n: i32) -> Self;

  /// Raise a number to a real number power.
  fn powf(self, n: Self::Real) -> Self;

  /// Take the square root of a number.
  fn sqrt(self) -> Self;

  /// Returns `e^(self)`, (the exponential function).
  fn exp(self) -> Self;

  /// Returns `2^(self)`.
  fn exp2(self) -> Self;

  /// Returns the natural logarithm of the number.
  fn ln(self) -> Self;

  /// Returns the logarithm of the number with respect to an arbitrary base.
  fn log(self, base: Self::Real) -> Self {
    self.ln() / Self::from(base).ln()
  }

  /// Returns the base 2 logarithm of the number.
  fn log2(self) -> Self;

  /// Returns the base 10 logarithm of the number.
  fn log10(self) -> Self;

  /// Take the cubic root of a number.
  fn cbrt(self) -> Self;

  /// Computes the sine of a number (in radians).
  fn sin(self) -> Self;

  /// Computes the cosine of a number (in radians).
  fn cos(self) -> Self;

  /// Computes the tangent of a number (in radians).
  fn tan(self) -> Self;

  /// Computes the arcsine of a number. Return value is in radians in
  /// the range [-pi/2, pi/2] or NaN if the number is outside the range
  /// [-1, 1].
  fn asin(self) -> Self;

  /// Computes the arccosine of a number. Return value is in radians in
  /// the range [0, pi] or NaN if the number is outside the range
  /// [-1, 1].
  fn acos(self) -> Self;

  /// Computes the arctangent of a number. Return value is in radians in the
  /// range [-pi/2, pi/2];
  fn atan(self) -> Self;

  /// Simultaneously computes the sine and cosine of the number, `x`. Returns
  fn sin_cos(self) -> (Self, Self);

  /// Returns `e^(self) - 1` in a way that is accurate even if the
  /// number is close to zero.
  fn exp_m1(self) -> Self;

  /// Returns `ln(1+n)` (natural logarithm) more accurately than if
  /// the operations were performed separately.
  fn ln_1p(self) -> Self;

  /// Hyperbolic sine function.
  fn sinh(self) -> Self;

  /// Hyperbolic cosine function.
  fn cosh(self) -> Self;

  /// Hyperbolic tangent function.
  fn tanh(self) -> Self;

  /// Inverse hyperbolic sine function.
  fn asinh(self) -> Self;

  /// Inverse hyperbolic cosine function.
  fn acosh(self) -> Self;

  /// Inverse hyperbolic tangent function.
  fn atanh(self) -> Self;

  /// Computes the abs(self)^2 / self
  fn conj(self) -> Self;
}


