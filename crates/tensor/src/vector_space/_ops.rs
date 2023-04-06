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

// trait GenGroup {
//   fn add(self, other: Self) -> Self;
//   fn ref_add(&self, other: Self) -> Self;
//   fn add_ref(self, other: &Self) -> Self;
//   fn ref_add_ref(&self, other: &Self) -> Self;

//   fn sub(self, other: Self) -> Self;
//   fn ref_sub(&self, other: Self) -> Self;
//   fn sub_ref(self, other: &Self) -> Self;
//   fn ref_sub_ref(&self, other: &Self) -> Self;

//   fn neg(self) -> Self;
//   fn ref_neg(&self) -> Self;

//   fn zero() -> Self;
// }

// struct Group<T: GenGroup>(pub T);

// impl<T: GenGroup> Add<Group<T>> for Group<T> {
//   type Output = Group<T>;

//   fn add(self, rhs: Self) -> Group<T> {
//     Group(self.0.add(rhs.0))
//   }
// }

// impl<T: GenGroup> Add<&Group<T>> for Group<T> {
//   type Output = Self;

//   fn add(self, rhs: &Self) -> Self {
//     Group(self.0.add_ref(&rhs.0))
//   }
// }


// #[derive(Clone, Copy)]
// struct GroupRepr(usize);

// trait GroupExpr {
//   fn as_group_repr(&self) -> GroupRepr;
// }

// struct GroupSum<A: GroupExpr, B: GroupExpr>(pub A, pub B);

// impl Add<GroupRepr> for GroupRepr {
//   type Output = GroupSum<G, H>;

//   fn add(self, rhs: H) -> Self::Output {
//       todo!()
//   }
// }

