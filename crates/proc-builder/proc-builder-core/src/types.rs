
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataTypeKind {
  TSelf,
  T
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct DataTypeKindT<T>(pub DataTypeKindBase, PhantomData<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputDataAccess {
  Owned,
  Ref,
  MutRef,
  Borrowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputDataShape {
  Owned(DataTypeKind),
  Ref(DataTypeKind),
  MutRef(DataTypeKind),
  Unit,
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct DataShape(pub Vec<InputDataShape>, pub OutputDataShape);



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImplTraitFunction;




// impl<T> DataTypeKindT<T> {
//   pub const fn new(value: DataTypeKindBase) -> Self {
//     Self(value, PhantomData)
//   }
// }

// impl<T> ::core::ops::Deref for DataTypeKindT<T> {
//   type Target = DataTypeKindBase;
//   fn deref(&self) -> &Self::Target {
//     &self.0
//   }
// }
// impl<T> ::core::ops::DerefMut for DataTypeKindT<T> {
//   fn deref_mut(&mut self) -> &mut Self::Target {
//     &mut self.0
//   }
// }
