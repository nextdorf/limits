pub use num::*;

pub trait MulCommute { }

#[allow(non_camel_case_types)]
pub type c128 = complex::Complex64;

#[allow(non_camel_case_types)]
pub type c64 = complex::Complex32;


// impl MulCommute for usize {}
// impl MulCommute for u8 {}
// impl MulCommute for u16 {}
// impl MulCommute for u32 {}
// impl MulCommute for u64 {}
// impl MulCommute for u128 {}
// impl MulCommute for isize {}
// impl MulCommute for i8 {}
// impl MulCommute for i16 {}
// impl MulCommute for i32 {}
// impl MulCommute for i64 {}
// impl MulCommute for i128 {}
// impl MulCommute for f32 {}
// impl MulCommute for f64 {}
// impl MulCommute for c64 {}
// impl MulCommute for c128 {}


impl<X: Num> MulCommute for X {}

