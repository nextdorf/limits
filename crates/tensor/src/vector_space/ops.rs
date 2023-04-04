use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait VectorSpaceOps<Field>:
  Sized
+ Add<Self, Output = Self>
+ Sub<Self, Output = Self>
+ Mul<Field, Output = Self>
+ Div<Field, Output = Self>
+ Neg<Output = Self>
{
}
impl<T, Field> VectorSpaceOps<Field> for T where T:
  Sized
+ Add<Self, Output = Self>
+ Sub<Self, Output = Self>
+ Mul<Field, Output = Self>
+ Div<Field, Output = Self>
+ Neg<Output = Self>
{
}


pub trait VectorSpaceOpsAssign<Field>:
  Sized
+ AddAssign<Self>
+ SubAssign<Self>
+ MulAssign<Field>
+ DivAssign<Field>
{
}
impl<T, Field> VectorSpaceOpsAssign<Field> for T where T:
  Sized
+ AddAssign<Self>
+ SubAssign<Self>
+ MulAssign<Field>
+ DivAssign<Field>
{
}


pub trait VectorSpaceOpsRef<Field>: Sized where
for<'a> Self:
  Add<&'a Self, Output = Self>
+ Sub<&'a Self, Output = Self>
+ Mul<&'a Field, Output = Self>
+ Div<&'a Field, Output = Self>
{
}
impl<T, Field> VectorSpaceOpsRef<Field> for T where T: Sized,
for<'a> Self:
  Add<&'a Self, Output = Self>
+ Sub<&'a Self, Output = Self>
+ Mul<&'a Field, Output = Self>
+ Div<&'a Field, Output = Self>
{
}


pub trait VectorSpaceRefOps<Field>: Sized where
for<'a> &'a Self:
  Add<Self, Output = Self>
+ Sub<Self, Output = Self>
+ Mul<Field, Output = Self>
+ Div<Field, Output = Self>
+ Neg<Output = Self>
{
}
impl<T, Field> VectorSpaceRefOps<Field> for T where T: Sized,
for<'a> &'a Self:
  Add<Self, Output = Self>
+ Sub<Self, Output = Self>
+ Mul<Field, Output = Self>
+ Div<Field, Output = Self>
+ Neg<Output = Self>
{
}


pub trait VectorSpaceOpsAssignRef<Field>: Sized where
for<'a> Self:
  AddAssign<&'a Self>
+ SubAssign<&'a Self>
+ MulAssign<&'a Field>
+ DivAssign<&'a Field>
{
}
impl<T, Field> VectorSpaceOpsAssignRef<Field> for T where T: Sized,
for<'a> Self:
  AddAssign<&'a Self>
+ SubAssign<&'a Self>
+ MulAssign<&'a Field>
+ DivAssign<&'a Field>
{
}


pub trait VectorSpaceFullOps<Field>:
  VectorSpaceOps<Field>
+ VectorSpaceOpsRef<Field>
+ VectorSpaceOpsAssign<Field>
+ VectorSpaceOpsAssignRef<Field>
{
}
impl<T, Field> VectorSpaceFullOps<Field> for T where T:
  VectorSpaceOps<Field>
+ VectorSpaceOpsRef<Field>
+ VectorSpaceOpsAssign<Field>
+ VectorSpaceOpsAssignRef<Field>
{
}

