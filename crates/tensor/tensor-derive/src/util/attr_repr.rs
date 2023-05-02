use std::ops::{Deref, DerefMut};
use quote::quote;
use syn::{parse_quote, Meta, punctuated::Punctuated, Token, parse::{ParseBuffer, Parse}};

type TokenStream = syn::__private::TokenStream;


#[derive(Debug)]
pub struct OptField<T> {
  pub set_field: Option<T>,
  pub default_field: T,
}

pub struct PathSpecifier {
  pub base: syn::Path,
  pub kind: Option<syn::Path>
}

pub struct AttrRepr {
  pub zero_path: OptField<syn::Path>,
  pub one_path: OptField<syn::Path>,
  pub inv_path: OptField<syn::Path>,
  // pub gen_group_path: OptField<(syn::Path, Option<syn::Path>)>,
  // pub gen_abel_group_path: OptField<(syn::Path, Option<syn::Path>)>,
  pub gen_group_path: OptField<PathSpecifier>,
  pub gen_abel_group_path: OptField<PathSpecifier>,
  pub unit_idents: OptField<Vec<syn::Ident>>,
}


impl<T> OptField<T> {
  pub const fn new(default: T) -> Self {
    Self { set_field: None, default_field: default }
  }
}


impl AttrRepr {
  pub fn new_with<'a>(iter: impl Iterator<Item = &'a syn::Attribute>) -> Self {
    let mut res = Self::default();
    for x in iter.into_iter() {
      if let Meta::List(xs) = &x.meta {
        if let Some(ident) = xs.path.get_ident() {
          // println!("1: {}", xs.to_token_stream());
          match ident.to_string().as_str() {
            "num_traits_zero_path" => *res.zero_path = xs.parse_args().unwrap(),
            "num_traits_one_path" => *res.one_path = xs.parse_args().unwrap(),
            "num_traits_inv_path" => *res.inv_path = xs.parse_args().unwrap(),
            "gen_group_path" => *res.gen_group_path = xs.parse_args().unwrap(),
            // "gen_group_path" => *res.gen_group_path = Self::parse_with_opt(xs).unwrap(),
            // "gen_group_path" => panic!("{}", xs.tokens),
            "gen_abel_group_path" => *res.gen_abel_group_path = xs.parse_args().unwrap(),
            // "gen_abel_group_path" => *res.gen_abel_group_path = Self::parse_with_opt(xs).unwrap(),
            // "gen_abel_group_path" => panic!("{}", xs.tokens),
            //TODO
            s => eprintln!("Unknown: {}", s)
          }
        }
      }
    }
    res
  }

  fn parse_with_opt<P, Q>(ms: &syn::MetaList) -> syn::Result<(P, Option<Q>)> where P: Parse, Q: Parse {
    let inner_p = |input: &ParseBuffer| -> syn::Result<(P, Option<Q>)> {
      // let mut ps = input.parse_terminated(syn::Expr::parse, Token![:])?.iter();
      let ps = Punctuated::<syn::Expr, Token![:]>::parse_terminated(input)?;
      let mut ps = ps.iter();
      // let mut ps: Punctuated::<syn::Expr, Token![:]> = input.parse()?.iter();
      let p0 = ps.next().ok_or(input.error("Path is missing"))?;
      let p1 = ps.next();
      if ps.next().is_some() {
        return Err(input.error("Too many specifications"));
      }
      Ok((parse_quote!(#p0), p1.map(|x| parse_quote!(#x))))
    };
    ms.parse_args_with(inner_p)
    // todo!()
  }
}


impl PathSpecifier {
  pub const fn new(base: syn::Path, kind: Option<syn::Path>) -> Self {
    Self { base, kind }
  }
}


impl<T> Deref for OptField<T> {
  type Target = T;
  fn deref(&self) -> &T {
    self.set_field.as_ref().unwrap_or(&self.default_field)
  }
}

impl<T> DerefMut for OptField<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.set_field.as_mut().unwrap_or(&mut self.default_field)
  }
}


impl Default for AttrRepr {
  fn default() -> Self {
    Self {
      zero_path: OptField::new(parse_quote!(::tensor::group::Zero)),
      one_path: OptField::new(parse_quote!(::tensor::group::One)),
      inv_path: OptField::new(parse_quote!(::tensor::group::Inv)),
      gen_group_path: OptField::new(PathSpecifier::new(parse_quote!(::tensor::GenGroup), None)),
      gen_abel_group_path: OptField::new(PathSpecifier::new(parse_quote!(::tensor::GenAbelGroup), None)),
      unit_idents: OptField::new({
        let idents: Punctuated::<_, Token![,]> = parse_quote![PhantomData];
        idents.into_iter().collect()
      }),
    }
  }
}


impl Parse for PathSpecifier {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ps = Punctuated::<syn::Path, Token![:]>::parse_terminated(input)?;
    let mut ps = ps.into_iter();
    // let mut ps: Punctuated::<syn::Expr, Token![:]> = input.parse()?.iter();
    let base = ps.next().ok_or(input.error("Base path is missing"))?;
    let kind = ps.next();
    if ps.next().is_some() {
      return Err(input.error("Too many specifications"));
    }
    Ok(Self { base, kind })
  }
}


impl quote::ToTokens for PathSpecifier {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let Self { base, kind } = self;
    if kind.is_some() {
      quote!(#base: #kind).to_tokens(tokens)
    } else {
      base.to_tokens(tokens)
    }
  }
}


