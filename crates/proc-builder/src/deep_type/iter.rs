use std::pin::Pin;

use futures::{FutureExt, future::LocalBoxFuture};
use quote::ToTokens;
use syn::{parse_quote, Ident, Type, TypePath, DataStruct};
use proc_macro2::TokenStream;
use super::{DeepTypeValue, DeepTypeKind};
use crate::{IterAsync, yield_async};
pub type Iter<'a> = IterAsync<'a, DeepTypeValue>;


impl<'a> Iter<'a> {
  pub fn new_from_data(ident: &'a Ident, data: &'a DataStruct) -> Self {
    Self::new_without_waker(|out| Self::iter_data(ident, data, out))
  }

  async fn iter_data(ident: &Ident, data: &DataStruct, mut out: Pin<&mut Option<DeepTypeValue>>) {
    fn inner<'a>(fs: Vec<TokenStream>, ty: &'a Type, kind: DeepTypeKind, mut out: Pin<&'a mut Option<DeepTypeValue>>) -> LocalBoxFuture<'a, ()> {
      async move {
        match ty {
          Type::Array(_) | Type::Path(_) | Type::Ptr(_) | Type::Reference(_) | Type::Slice(_) => {
            yield_async!(out <- DeepTypeValue { path: fs, ty: ty.clone(), kind })
          },
          Type::Tuple(ts) => {
            for (i, elem) in ts.elems.iter().enumerate() {
              let mut fs = fs.clone();
              fs.push(syn::Index::from(i).into_token_stream());
              inner(fs, elem, kind, out.as_mut()).await;
            }
          },
          // Type::BareFn(_) => todo!(),
          // Type::Group(_) => todo!(),
          // Type::ImplTrait(_) => todo!(),
          // Type::Infer(_) => todo!(),
          // Type::Macro(_) => todo!(),
          // Type::Never(_) => todo!(),
          // Type::Paren(_) => todo!(),
          // Type::TraitObject(_) => todo!(),
          // Type::Verbatim(_) => todo!(),
          _ => todo!(),
          
        }
      }.boxed_local()
    }

    match &data.fields {
      syn::Fields::Named(syn::FieldsNamed {named: fields, ..}) => {
        for f in fields {
          let fs = f.ident.as_ref()
            .map(|x| Vec::from([x.to_token_stream()]))
            .unwrap_or_default();
          inner(fs, &f.ty, DeepTypeKind::NamedStructElem, out.as_mut()).await;
        }
      },
      syn::Fields::Unnamed(syn::FieldsUnnamed {unnamed: fields, ..}) => {
        for (i, f) in fields.iter().enumerate() {
          let idx = syn::Index::from(i).into_token_stream();
          inner(vec![idx], &f.ty, DeepTypeKind::UnnamedStructElem, out.as_mut()).await;
        }
      },
      syn::Fields::Unit => {
        let ty = Type::Path(TypePath { qself: None, path: parse_quote!(#ident) });
        yield_async!(out <- DeepTypeValue { path: Vec::new(), ty, kind: DeepTypeKind::UnitStructElem })
      },
    }
    yield_async!(out);
  }
}


#[test]
fn test_iter_data() {
  use crate::{types::{InputDataAccess, AccessExpr}, tests::assert_eq_wo_whitespace};
  use syn::{Data, Token, DeriveInput, punctuated::Punctuated};
  use quote::{quote};

  let data: DeriveInput = parse_quote!(
    struct A {
      pub vals: (i32, char),
      num: &'static usize,
      bytes: [u8; 9]
    }
  );

  let base = parse_quote!(x);

  let exprs = if let Data::Struct(s) = &data.data {
    Iter::new_from_data(&data.ident, s)
      // .map(|x| x.as_expr(InputDataAccess::Owned, &base))
      .map(|x| x.as_expr(AccessExpr::new(InputDataAccess::Owned, &base)))
      .collect::<Punctuated<_, Token![,]>>()
  } else { panic!() };
  assert_eq_wo_whitespace(quote!(#exprs), "x.vals.0, x.vals.1, x.num, x.bytes")
}

