use quote::{ToTokens, quote};

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

#[derive(Clone, Copy)]
pub struct AccessExpr<'a> {
  pub access: InputDataAccess,
  pub base: &'a syn::Expr
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


impl<'a> AccessExpr<'a> {
  pub const fn new(access: InputDataAccess, base: &'a syn::Expr) -> Self {
    Self { access, base }
  }
}

impl ToTokens for AccessExpr<'_> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let base = self.base;
    let token = match self.access {
      InputDataAccess::Owned => quote!(#base),
      InputDataAccess::Ref => quote!(&#base),
      InputDataAccess::MutRef => quote!(&mut #base),
      InputDataAccess::Borrowed => quote!(&#base.borrow()),
    };

    tokens.extend(token)
  }
}


pub(crate) mod macros {
  #[allow(unused_macros)]
  macro_rules! new_access_expr {
    ($ident:ident, $e:expr) => {
      crate::types::AccessExpr::new(crate::types::InputDataAccess::$ident, &parse_quote!($e))
    };
  }

  #[allow(unused_imports)]
  pub(crate) use new_access_expr;
}
