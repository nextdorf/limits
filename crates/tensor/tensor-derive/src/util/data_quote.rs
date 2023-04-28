use quote::{quote, ToTokens};
use syn::{Path, Data, Ident, DataStruct, parse_quote, Fields, FieldsNamed, punctuated::Punctuated, GenericParam, TypeParam, TraitBound, TypeParamBound, parse::Parse, FieldsUnnamed, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataQuote {
  Unit,

  Own,
  Ref,
  Mut,

  OwnOwn,
  RefOwn,
  MutOwn,

  OwnBor,
  RefBor,
  MutBor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpKind {
  Unit,
  Unary,
  Binary,
}

pub struct DataQuotePaths {
  pub t_idents: Vec<Ident>,
  pub unit_idents: Vec<Ident>,
  pub t_fn_path: Path,
  pub default_fn_path: Path,
}

type TokenStream = quote::__private::TokenStream;


impl DataQuote {
  pub fn quote(&self, lhs_path: &Path, rhs_path: &Path, paths: &DataQuotePaths, ident: &Ident, data: &Data) -> TokenStream {
    let rhs_path = match self {
      Self::OwnBor | Self::RefBor | Self::MutBor => parse_quote!(&#rhs_path.borrow()),
      Self::Unit | Self::Own | Self::Ref | Self::Mut | Self::OwnOwn | Self::RefOwn | Self::MutOwn => parse_quote!(#rhs_path),
    };
    match data {
      Data::Struct(DataStruct {fields, ..}) => {
        match (fields, self.in_place()) {
          (Fields::Named(FieldsNamed {named, ..}), false) => {
            let mut fields = Punctuated::<syn::FieldValue, syn::Token![,]>::new();
            for n in named {
              let val = n.ident.as_ref().unwrap();
              let (a, b) = self.resolve_field_named(paths, &n.ty, val, lhs_path, &rhs_path);
              fields.push(parse_quote!(#a: #b));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote!(#ident { #fields })
          },
          (Fields::Unnamed(FieldsUnnamed { unnamed, .. }), false) => {
            let mut fields = Punctuated::<syn::Expr, syn::Token![,]>::new();
            for (i, n) in unnamed.iter().enumerate() {
              let idx = syn::Index::from(i);
              fields.push(self.resolve_field_unnamed(paths, &n.ty, &idx, lhs_path, &rhs_path));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote!(#ident ( #fields ))
          },
          (Fields::Named(FieldsNamed {named, ..}), true) => {
            let mut assignments = Punctuated::<syn::Expr, syn::Token![; ]>::new();
            for n in named {
              let val = n.ident.as_ref().unwrap();
              let (_, b) = self.resolve_field_named(paths, &n.ty, val, lhs_path, &rhs_path);
              assignments.push(b);
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            assignments.to_token_stream()
          },
          (Fields::Unnamed(FieldsUnnamed { unnamed, .. }), true) => {
            let mut assignments = Punctuated::<syn::Expr, syn::Token![; ]>::new();
            for (i, n) in unnamed.iter().enumerate() {
              let idx = syn::Index::from(i);
              assignments.push(self.resolve_field_unnamed(paths, &n.ty, &idx, lhs_path, &rhs_path));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            assignments.to_token_stream()
          },
          (Fields::Unit, false) => ident.to_token_stream(),
          (Fields::Unit, true) => TokenStream::new(),
        }
      },
      _ => panic!("Use structs")
    }
  }

  pub fn chain_bool(&self, lhs_path: &Path, rhs_path: &Path, paths: &DataQuotePaths, data: &Data) -> TokenStream {
    let rhs_path: syn::Expr = match self {
      Self::OwnBor | Self::RefBor | Self::MutBor => parse_quote!(&#rhs_path.borrow()),
      Self::Unit | Self::Own | Self::Ref | Self::Mut | Self::OwnOwn | Self::RefOwn | Self::MutOwn => parse_quote!(#rhs_path),
    };
    match data {
      Data::Struct(DataStruct {fields, ..}) => {
        match fields {
          Fields::Named(FieldsNamed {named, ..}) => {
            let mut fields = Punctuated::<syn::Expr, syn::Token![ && ]>::new();
            for n in named {
              let val = n.ident.as_ref().unwrap();
              fields.push(self.chain_bool_rec(paths, &n.ty, val, lhs_path, &rhs_path));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote!(#fields)
          },
          Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut fields = Punctuated::<syn::Expr, syn::Token![ && ]>::new();
            for (i, n) in unnamed.iter().enumerate() {
              let idx = syn::Index::from(i);
              fields.push(self.chain_bool_rec(paths, &n.ty, &idx, lhs_path, &rhs_path));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote!(#fields)
          },
          Fields::Unit => quote!(true),
        }
      },
      _ => panic!("Use structs")
    }
  }

  fn resolve_field_named<'a>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &Ident, lhs: &Path, rhs: &syn::Expr) -> (Path, syn::Expr) {
    self.resolve_field_rec(paths, ty, val, lhs, rhs)
  }

  fn resolve_field_unnamed<'a>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &syn::Index, lhs: &Path, rhs: &syn::Expr) -> syn::Expr {
    self.resolve_field_rec::<syn::Index, _>(paths, ty, val, lhs, rhs).1
  }

  fn resolve_field_rec<'a, P: Parse, Q: Parse>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &(impl Parse + ToTokens), lhs: &(impl Parse + ToTokens), rhs: &(impl Parse + ToTokens)) -> (P, Q) {
    let lhs_val = match self {
      Self::Unit => ty.to_token_stream(),
      Self::Own | Self::OwnOwn | Self::OwnBor => quote!(#lhs.#val),
      Self::Ref | Self::RefOwn | Self::RefBor => quote!((&#lhs.#val)),
      Self::Mut | Self::MutOwn | Self::MutBor => quote!((&mut #lhs.#val)),
    };
    
    let parsed_val = match ty {
      syn::Type::Array(xs) => {
        let len = &xs.len;
        let x_type = xs.elem.to_token_stream();
        // let x_type: syn::Type = parse_quote!(#x_type);
        let x_type: syn::TypePath = parse_quote!(#x_type);
        let fn_path = paths.select_path_for(&x_type.path).to_token_stream();
        // Self::resolve_field_rec(paths, ty, val, lhs, rhs)
        let lhs_iter = quote!(#lhs_val.into_iter());

        match self {
          Self::Unit => parse_quote!(
            [(); #len].map(|()| #x_type::#fn_path())
          ),
          Self::Own => parse_quote!(
            #lhs.#val.map(|x| x.#fn_path())
          ),
          Self::Ref => parse_quote!({
            let mut lhs_iter = #lhs_iter;
            [(); #len].map(|()| lhs_iter.next().unwrap().#fn_path())
          }),
          Self::Mut => parse_quote!({
            for x in #lhs_iter {
              x.#fn_path()
            }
          }),
          Self::OwnOwn | Self::RefOwn | Self::OwnBor | Self::RefBor => parse_quote!({
            let mut res_iter = #lhs_iter.zip(#rhs.#val).map(|(x, y)| x.#fn_path(y));
            [(); #len].map(|()| res_iter.next().unwrap())
          }),
          Self::MutOwn | Self::MutBor => parse_quote!({
            for (x, y) in #lhs_iter.zip(#rhs.#val) {
              x.#fn_path(y)
            }
          }),
        }
      },
      syn::Type::Path(p) => {
        let p_path = &p.path;
        // let p_ident_path_store: syn::Path;
        // let p_ident = match p_path.get_ident() {
        //   Some(ident) => Some(ident),
        //   None => {
        //     let mut out_path = p_path.segments.clone();
        //     if let Some(x) = out_path.last_mut() {
        //       x.arguments = syn::PathArguments::None;
        //     }
        //     p_ident_path_store = parse_quote!(#out_path);
        //     p_ident_path_store.get_ident()
        //   }
        // };
        // let is_unit_type = p_ident
        //   .and_then(|ident| {
        //     Some(paths.unit_idents.contains(ident))
        // }).unwrap_or(false);
        let (p_ident_path, is_unit_type) = Self::p_ident_path_helper(paths, p_path);
        let p_ident = p_ident_path.as_ref().and_then(syn::Path::get_ident).or(p_path.get_ident());

        if !is_unit_type {
          let fn_path = paths.select_path_for(p_path).to_token_stream();
          match self.kind() {
            OpKind::Unit => parse_quote!(<#p_path>::#fn_path()),
            OpKind::Unary => parse_quote!(#lhs_val.#fn_path()),
            OpKind::Binary => parse_quote!(#lhs_val.#fn_path(#rhs.#val)),
          }
        } else {
          if self.in_place() {
            // panic!("{:?}", p_ident);
            parse_quote!(())
          } else {
            parse_quote!(#p_ident)
          }
        }
      },
      syn::Type::Tuple(t) => {
        let res = t.elems
          .iter()
          .enumerate()
          .map(|(i, t)| self.resolve_field_rec::<TokenStream, syn::Expr>(
              paths,
              t,
              &syn::Index::from(i),
              &quote!(#lhs.#val),
              &quote!(#rhs.#val)
            ).1
          ).collect::<Punctuated::<_, syn::Token![,]>>();
        // let res = Punctuated::<_, Token![,]>::from_iter(res);
        // panic!("{}", res.to_token_stream());
        parse_quote!((#res))
      },
      _ => panic!("No implementation for case `{}`", ty.to_token_stream().to_string()),
    };
    (parse_quote!(#val), parsed_val)
  }

  fn chain_bool_rec<'a, P: Parse>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &(impl Parse + ToTokens), lhs: &(impl Parse + ToTokens), rhs: &(impl Parse + ToTokens)) -> P {
    let lhs_val = match self {
      Self::Unit => ty.to_token_stream(),
      Self::Own | Self::OwnOwn | Self::OwnBor => quote!(#lhs.#val),
      Self::Ref | Self::RefOwn | Self::RefBor => quote!((&#lhs.#val)),
      Self::Mut | Self::MutOwn | Self::MutBor => quote!((&mut #lhs.#val)),
    };
    
    match ty {
      syn::Type::Array(xs) => {
        let x_type = xs.elem.to_token_stream();
        let x_type: syn::TypePath = parse_quote!(#x_type);
        let fn_path = paths.select_path_for(&x_type.path).to_token_stream();
        let lhs_iter = quote!(#lhs_val.into_iter());

        match self {
          Self::Unit => parse_quote!(#x_type::#fn_path()),
          Self::Own | Self::Ref | Self::Mut => parse_quote!(
            #lhs_iter.find_map(|x|
              if x.#fn_path() { None } else { Some(false) }
            ).unwrap_or(true)
          ),
          Self::OwnOwn | Self::RefOwn | Self::MutOwn | Self::OwnBor | Self::RefBor | Self::MutBor => parse_quote!(
            #lhs_iter.zip(#rhs.#val).find_map(|(x, y)|
              if x.#fn_path(y) { None } else { Some(false) }
            ).unwrap_or(true)
          ),
        }
      },
      syn::Type::Path(p) => {
        let p_path = &p.path;
        let is_unit_type = Self::p_ident_path_helper(paths, p_path).1;
        // let (p_ident_path, is_unit_type) = Self::p_ident_path_helper(paths, p_path);
        // let p_ident = p_ident_path.as_ref().and_then(syn::Path::get_ident).or(p_path.get_ident());

        if !is_unit_type {
          let fn_path = paths.select_path_for(p_path).to_token_stream();
          match self.kind() {
            OpKind::Unit => parse_quote!(<#p_path>::#fn_path()),
            OpKind::Unary => parse_quote!(#lhs_val.#fn_path()),
            OpKind::Binary => parse_quote!(#lhs_val.#fn_path(#rhs.#val)),
          }
        } else {
          parse_quote!(true)
        }
      },
      syn::Type::Tuple(t) => {
        let res = t.elems
          .iter()
          .enumerate()
          .map(|(i, t)| self.chain_bool_rec::<syn::Expr>(
              paths,
              t,
              &syn::Index::from(i),
              &quote!(#lhs.#val),
              &quote!(#rhs.#val)
            )
          ).collect::<Punctuated::<_, syn::Token![ && ]>>();
        parse_quote!(#res)
      },
      _ => panic!("No implementation for case `{}`", ty.to_token_stream().to_string()),
    }
  }

  fn p_ident_path_helper(paths: &DataQuotePaths, p_path: &syn::Path) -> (Option<syn::Path>, bool) {
    fn is_unit_type(paths: &DataQuotePaths, p_ident: Option<&syn::Ident>) -> bool {
      p_ident.and_then(|ident| {
        Some(paths.unit_idents.contains(ident))
      }).unwrap_or(false)
    }

    match p_path.get_ident() {
      Some(ident) => (None, is_unit_type(paths, Some(ident))),
      None => {
        let mut out_path = p_path.segments.clone();
        if let Some(x) = out_path.last_mut() {
          x.arguments = syn::PathArguments::None;
        }
        let p_ident_path: syn::Path = parse_quote!(#out_path);
        let is_unit_type_res = is_unit_type(paths, p_ident_path.get_ident());
        (Some(p_ident_path), is_unit_type_res)
      }
    }
  }

  pub fn in_place(&self) -> bool {
    match self {
      Self::Mut | Self::MutOwn | Self::MutBor => true,
      Self::Unit | Self::Own | Self::Ref | Self::OwnOwn | Self::RefOwn | Self::OwnBor | Self::RefBor => false,
    }
  }

  pub fn kind(&self) -> OpKind {
    match self {
      Self::Unit => OpKind::Unit,
      Self::Own | Self::Ref | Self::Mut => OpKind::Unary,
      Self::OwnOwn | Self::RefOwn | Self::MutOwn | Self::OwnBor | Self::RefBor | Self::MutBor => OpKind::Binary,
    }
  }
}


impl DataQuotePaths {
  pub fn select_path_for(&self, p: &Path) -> Option<&Path> {
    if let Some(p) = p.get_ident() {
      if self.t_idents.iter().find(|q| q == &p).is_some() {
        return Some(&self.t_fn_path);
      }
      if self.unit_idents.iter().find(|q| q == &p).is_some() {
        return None;
      }
    }
    Some(&self.default_fn_path)
  }

  pub fn t_idents_from<'a>(path: &Path, params: impl Iterator<Item = &'a GenericParam>) -> Vec<Ident> {
    let cmp_path = path;
    let mut t_idents = Vec::new();
    for f in params {
      if let GenericParam::Type(TypeParam { ident, bounds, .. }) = f {
        for bound in bounds {
          if let TypeParamBound::Trait(TraitBound {path, ..}) = bound {
            let is_cmp_path = super::eq_iter_over(
              path.segments.iter(),
              cmp_path.segments.iter(),
              &|x| &x.ident
            );
            if is_cmp_path {
              t_idents.push(ident.clone());
              break;
            }
          }
        }
      }
    }
    t_idents
  }
}
