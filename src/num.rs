// pub mod bigint;

#[const_trait]
pub trait Zero {
  fn zero() -> Self;
}

#[const_trait]
pub trait One {
  fn one() -> Self;
}


impl const Zero for usize { fn zero() -> Self { 0 } }
impl const Zero for u8 { fn zero() -> Self { 0 } }
impl const Zero for u16 { fn zero() -> Self { 0 } }
impl const Zero for u32 { fn zero() -> Self { 0 } }
impl const Zero for u64 { fn zero() -> Self { 0 } }
impl const Zero for u128 { fn zero() -> Self { 0 } }
impl const Zero for isize { fn zero() -> Self { 0 } }
impl const Zero for i8 { fn zero() -> Self { 0 } }
impl const Zero for i16 { fn zero() -> Self { 0 } }
impl const Zero for i32 { fn zero() -> Self { 0 } }
impl const Zero for i64 { fn zero() -> Self { 0 } }
impl const Zero for i128 { fn zero() -> Self { 0 } }
impl const Zero for f32 { fn zero() -> Self { 0. } }
impl const Zero for f64 { fn zero() -> Self { 0. } }


impl const One for usize { fn one() -> Self { 1 } }
impl const One for u8 { fn one() -> Self { 1 } }
impl const One for u16 { fn one() -> Self { 1 } }
impl const One for u32 { fn one() -> Self { 1 } }
impl const One for u64 { fn one() -> Self { 1 } }
impl const One for u128 { fn one() -> Self { 1 } }
impl const One for isize { fn one() -> Self { 1 } }
impl const One for i8 { fn one() -> Self { 1 } }
impl const One for i16 { fn one() -> Self { 1 } }
impl const One for i32 { fn one() -> Self { 1 } }
impl const One for i64 { fn one() -> Self { 1 } }
impl const One for i128 { fn one() -> Self { 1 } }
impl const One for f32 { fn one() -> Self { 1. } }
impl const One for f64 { fn one() -> Self { 1. } }


impl<T: ~const Zero + Copy, const N: usize> const Zero for [T; N] {
  fn zero() -> Self {
    [T::zero(); N]
  }
}

impl<T: ~const One + Copy, const N: usize> const One for [T; N] {
  fn one() -> Self {
    [T::one(); N]
  }
}



