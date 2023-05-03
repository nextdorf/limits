use futures::{FutureExt, future::LocalBoxFuture};
use quote::ToTokens;
use syn::{parse_quote, Ident, Type, TypePath, DataStruct};

use proc_macro2::TokenStream;

use super::{DeepTypeValue, DeepTypeKind};

use crate::{IterAsync, yield_async};

pub type Iter<'a> = IterAsync<'a, DeepTypeValue>;


// macro_rules! return_async {
//   ($out:ident <- $val:expr) => {{
//     *$out = Some($val);
//     futures::pending!()
//   }};
//   ($out:ident) => {
//     *$out = None
//   }
// }


impl<'a> Iter<'a> {
  pub fn new_from_data(ident: &'a Ident, data: &'a DataStruct) -> Self {
    // let future = unsafe {
    //   let next_val = (&mut *next_val) as *mut Option<DeepTypeValue>;
    //   let next_val = next_val.as_mut().unwrap();
    //   Self::iter_data(ident, data, next_val).fuse().boxed_local()
    // };
    Self::new_without_waker(|out| Self::iter_data(ident, data, out))
  }

  async fn iter_data(ident: &Ident, data: &DataStruct, out: &mut Option<DeepTypeValue>) {
    fn inner<'a>(fs: Vec<TokenStream>, ty: &'a Type, kind: DeepTypeKind, out: &'a mut Option<DeepTypeValue>) -> LocalBoxFuture<'a, ()> {
      async move {
        match ty {
          Type::Array(_) | Type::Path(_) | Type::Ptr(_) | Type::Reference(_) | Type::Slice(_) => {
            yield_async!(out <- DeepTypeValue { path: fs, ty: ty.clone(), kind })
          },
          Type::Tuple(ts) => {
            for (i, elem) in ts.elems.iter().enumerate() {
              let mut fs = fs.clone();
              fs.push(syn::Index::from(i).into_token_stream());
              inner(fs, elem, kind, out).await;
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
          inner(fs, &f.ty, DeepTypeKind::NamedStructElem, out).await;
        }
      },
      syn::Fields::Unnamed(syn::FieldsUnnamed {unnamed: fields, ..}) => {
        for (i, f) in fields.iter().enumerate() {
          let idx = syn::Index::from(i).into_token_stream();
          inner(vec![idx], &f.ty, DeepTypeKind::UnnamedStructElem, out).await;
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





// impl<'a> Iterator for Iter<'a> {
//   type Item = DeepTypeValue;

//   fn next(&mut self) -> Option<Self::Item> {
//     // self.waker.wake_by_ref();
//     *self.next_val = None;
//     loop {
//       let mut ctx = Context::from_waker(&self.waker);
//       match self.future.as_mut().poll(&mut ctx) {
//         std::task::Poll::Pending if self.next_val.is_none() => {},
//         _ => break,
//       }
//     }
//     (*self.next_val).clone()
//   }
// }



#[test]
fn test_iter_data() {
  use crate::types::InputDataAccess;
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
      .map(|x| x.as_expr(InputDataAccess::Owned, &base))
      .collect::<Punctuated<_, Token![,]>>()
  } else { panic!() };
  assert_eq!(quote!(#exprs).to_string(), "x . vals . 0 , x . vals . 1 , x . num , x . bytes")
}

