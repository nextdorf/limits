#[const_trait]
pub trait Zero {
  fn zero() -> Self;
}

#[const_trait]
pub trait One {
  fn one() -> Self;
}

#[const_trait]
pub trait MulCommute { }


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

impl const MulCommute for usize {}
impl const MulCommute for u8 {}
impl const MulCommute for u16 {}
impl const MulCommute for u32 {}
impl const MulCommute for u64 {}
impl const MulCommute for u128 {}
impl const MulCommute for isize {}
impl const MulCommute for i8 {}
impl const MulCommute for i16 {}
impl const MulCommute for i32 {}
impl const MulCommute for i64 {}
impl const MulCommute for i128 {}
impl const MulCommute for f32 {}
impl const MulCommute for f64 {}




