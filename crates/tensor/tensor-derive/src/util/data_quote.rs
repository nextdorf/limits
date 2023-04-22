use quote::{quote, ToTokens};
use syn::{Path, Data, Ident, DataStruct, parse_quote, Fields, FieldsNamed, punctuated::Punctuated, GenericParam, TypeParam, TraitBound, TypeParamBound, parse::Parse, FieldsUnnamed};

pub enum DataQuote {
  OwnOwn,
  RefOwn,
  MutOwn,
}

pub struct DataQuotePaths {
  pub t_idents: Vec<Ident>,
  pub t_fn_path: Path,
  pub default_fn_path: Path,
}

type TokenStream = quote::__private::TokenStream;


impl DataQuote {
  pub fn quote(&self, lhs_path: &Path, rhs_path: &Path, paths: &DataQuotePaths, ident: &Ident, data: &Data) -> TokenStream {
    match data {
      Data::Struct(DataStruct {fields, ..}) => {
        match (fields, self.in_place()) {
          (Fields::Named(FieldsNamed {named, ..}), false) => {
            let mut fields = Punctuated::<syn::FieldValue, syn::Token![,]>::new();
            for n in named {
              let val = n.ident.as_ref().unwrap();
              let (a, b) = self.resolve_field_named(paths, &n.ty, val, lhs_path, rhs_path);
              fields.push(parse_quote!(#a: #b));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote!(#ident { #fields })
          },
          (Fields::Unnamed(FieldsUnnamed { unnamed, .. }), false) => {
            let mut fields = Punctuated::<syn::Expr, syn::Token![,]>::new();
            for (i, n) in unnamed.iter().enumerate() {
              let idx = syn::Index::from(i);
              fields.push(self.resolve_field_unnamed(paths, &n.ty, &idx, lhs_path, rhs_path));
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote!(#ident ( #fields ))
          },
          (Fields::Named(FieldsNamed {named, ..}), true) => {
            let mut assignments = Punctuated::<syn::Expr, syn::Token![; ]>::new();
            for n in named {
              let val = n.ident.as_ref().unwrap();
              let (_, b) = self.resolve_field_named(paths, &n.ty, val, lhs_path, rhs_path);
              assignments.push(b);
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            assignments.to_token_stream()
          },
          (Fields::Unnamed(FieldsUnnamed { unnamed, .. }), true) => {
            let mut assignments = Punctuated::<syn::Expr, syn::Token![; ]>::new();
            for (i, n) in unnamed.iter().enumerate() {
              let idx = syn::Index::from(i);
              assignments.push(self.resolve_field_unnamed(paths, &n.ty, &idx, lhs_path, rhs_path));
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

  fn resolve_field_named<'a>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &Ident, lhs: &Path, rhs: &Path) -> (Path, syn::Expr) {
    self.resolve_field_rec(paths, ty, val, lhs, rhs)
  }

  fn resolve_field_unnamed<'a>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &syn::Index, lhs: &Path, rhs: &Path) -> syn::Expr {
    self.resolve_field_rec::<syn::Index, _>(paths, ty, val, lhs, rhs).1
  }

  fn resolve_field_rec<'a, P: Parse, Q: Parse>(&self, paths: &'a DataQuotePaths, ty: &syn::Type, val: &(impl Parse + ToTokens), lhs: &(impl Parse + ToTokens), rhs: &(impl Parse + ToTokens)) -> (P, Q)
  {
    let lhs_val = match self {
      DataQuote::OwnOwn => quote!(#lhs.#val),
      DataQuote::RefOwn => quote!((&#lhs.#val)),
      DataQuote::MutOwn => quote!((&mut #lhs.#val)),
    };
    
    let parsed_val = match ty {
      syn::Type::Array(xs) => {
        let len = &xs.len;
        let x_type = xs.elem.to_token_stream();
        // let x_type: syn::Type = parse_quote!(#x_type);
        let x_type: syn::TypePath = parse_quote!(#x_type);
        let fn_path = paths.select_path_for(&x_type.path).to_token_stream();
        // Self::resolve_field_rec(paths, ty, val, lhs, rhs)

        if self.in_place() {
          parse_quote!({
            let lhs_iter = #lhs_val.into_iter();
            let rhs_iter = #rhs.#val.into_iter();
            for (x, y) in lhs_iter.zip(rhs_iter) {
              x.#fn_path(y)
            }
          })
        } else {
          parse_quote!({
            let lhs_iter = #lhs_val.into_iter();
            let rhs_iter = #rhs.#val.into_iter();
            let mut res_iter = lhs_iter.zip(rhs_iter)
            // .map(|(x, y)| #x_fn_y);
              .map(|(x, y)| x.#fn_path(y));
            [(); #len].map(|()| res_iter.next().unwrap())
          })
        }
      },
      syn::Type::Path(p) => {
        let fn_path = paths.select_path_for(&p.path).to_token_stream();
        // let r: TokenStream = parse_quote!(#val: #lhs.#val.#fn_path(#rhs.#val));
        // panic!("{r}");
        parse_quote!(#lhs_val.#fn_path(#rhs.#val))
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
      _ => panic!("Not implementation for case `{}`", ty.to_token_stream().to_string()),
    };
    (parse_quote!(#val), parsed_val)
  }

  pub fn in_place(&self) -> bool {
    match self {
      Self::OwnOwn | Self::RefOwn => false,
      Self::MutOwn => true,
    }
  }
}


impl DataQuotePaths {
  pub fn select_path_for(&self, p: &Path) -> &Path {
    if let Some(p) = p.get_ident() {
      for q in &self.t_idents {
        if q == p {
          return &self.t_fn_path;
        }
      }
    }
    &self.default_fn_path
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
