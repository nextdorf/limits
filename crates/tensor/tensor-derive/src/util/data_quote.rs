use quote::{quote, ToTokens};
use syn::{Path, Data, Ident, DataStruct, parse_quote, Fields, FieldsNamed, punctuated::Punctuated, GenericParam, TypeParam, TraitBound, TypeParamBound, parse::Parse};

pub struct DataQuote;

pub struct DataQuotePaths {
  pub t_idents: Vec<Ident>,
  pub t_fn_path: Path,
  pub default_fn_path: Path,
}

type TokenStream = quote::__private::TokenStream;


impl DataQuote {
  pub fn quote_own_own(lhs_path: &Path, rhs_path: &Path, paths: &DataQuotePaths, ident: &Ident, data: &Data) -> TokenStream {
    match data {
      Data::Struct(DataStruct {fields, ..}) => {
        match fields {
          Fields::Named(FieldsNamed {named, ..}) => {
            let mut fields = Punctuated::<syn::FieldValue, syn::Token![,]>::new();
            for n in named {
              let val = n.ident.as_ref().unwrap();
              fields.push(Self::resolve_field(paths, &n.ty, val, lhs_path, rhs_path));
              // if let syn::Type::Path(p) = &n.ty {
              //   let fn_path = paths.select_path_for(&p.path).to_token_stream();
              //   fields.push(parse_quote!(#val: #lhs_path.#val.#fn_path(#rhs_path.#val)));
              // }
            }
            // println!("{:?}", fields.iter().collect::<Vec<_>>());
            quote! {
              #ident { #fields }
            }
          },
          Fields::Unnamed(_) => panic!("Named or units"),
          Fields::Unit => ident.to_token_stream(),
        }
      },
      _ => panic!("Use structs")
    }
  }

  fn resolve_field<'a>(paths: &'a DataQuotePaths, ty: &syn::Type, val: &Ident, lhs: &Path, rhs: &Path) -> syn::FieldValue {
    let (a, b): (Path, syn::Expr) = Self::resolve_field_rec(paths, ty, val, lhs, rhs);
    // panic!("{}: {}", a.to_token_stream(), b.to_token_stream());
    parse_quote!(#a: #b)
  }

  fn resolve_field_rec<'a, P: Parse, Q: Parse>(paths: &'a DataQuotePaths, ty: &syn::Type, val: &(impl Parse + ToTokens), lhs: &(impl Parse + ToTokens), rhs: &(impl Parse + ToTokens)) -> (P, Q)
  {
    match ty {
      // syn::Type::Array(_) => todo!(),
      // syn::Type::BareFn(_) => todo!(),
      // syn::Type::Group(_) => todo!(),
      // syn::Type::ImplTrait(_) => todo!(),
      // syn::Type::Infer(_) => todo!(),
      // syn::Type::Macro(_) => todo!(),
      // syn::Type::Never(_) => todo!(),
      syn::Type::Path(p) => {
        let fn_path = paths.select_path_for(&p.path).to_token_stream();
        // let r: TokenStream = parse_quote!(#val: #lhs.#val.#fn_path(#rhs.#val));
        // panic!("{r}");
        (parse_quote!(#val), parse_quote!(#lhs.#val.#fn_path(#rhs.#val)))
      },
      // syn::Type::Ptr(_) => todo!(),
      // syn::Type::Reference(_) => todo!(),
      // syn::Type::Slice(_) => todo!(),
      // syn::Type::TraitObject(_) => todo!(),
      syn::Type::Tuple(t) => {
        let res = t.elems
          .iter()
          .enumerate()
          .map(|(i, t)| Self::resolve_field_rec::<TokenStream, syn::Expr>(
              paths,
              t,
              &syn::Index::from(i),
              &parse_quote!(#lhs.#val) as &TokenStream,
              &parse_quote!(#rhs.#val) as &TokenStream
            ).1
          // ).collect::<Vec<_>>();
          ).collect::<Punctuated::<_, syn::Token![,]>>();
        // let res = Punctuated::<_, Token![,]>::from_iter(res);
        // panic!("{}", res.to_token_stream());
        (parse_quote!(#val), parse_quote!((#res)))
      },
      // syn::Type::Verbatim(_) => todo!(),
      _ => panic!("Not implementation for case `{}`", ty.to_token_stream().to_string()),
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

  pub fn t_idents_from_yoo<'a>(path: &Path, params: impl Iterator<Item = &'a GenericParam>) -> Vec<Ident> {
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
