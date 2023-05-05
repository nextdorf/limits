use quote::ToTokens;
use syn::{Ident, Type, FnArg, parse_quote, parse::{self, Parse}, ItemFn, Signature, Generics, Token, punctuated::Punctuated, Block, spanned::Spanned, ReturnType};

use crate::{types::{InputDataAccess, AccessExprOwned}};

#[derive(Clone)]
pub struct FnArgBuilder {
  pub access: InputDataAccess,
  pub var_name: Option<Ident>,
  pub ty: Type,
}

pub struct FnBuilder {
  pub vis: syn::Visibility,
  pub ident: Ident,
  pub input: Vec<FnArgBuilder>,
  pub output: ReturnType,
  pub generics: Generics,
  pub var_store: Option<Box<dyn ::core::any::Any>>,
  pub calc_body: Box<dyn Fn(Option<&dyn std::any::Any>) -> syn::Block>
}


impl FnArgBuilder {
  pub fn get_access_expr(&self) -> AccessExprOwned {
    AccessExprOwned {
      access: self.access,
      base: match &self.var_name {
        Some(name) => parse_quote!(#name),
        None => parse_quote!(self),
      }
    }
  }

  pub fn build(&self) -> FnArg {
    let FnArgBuilder { access, var_name, ty } = self;
    match var_name {
      Some(name) => {
        let pat = parse_quote!(#name);
        let ty = Box::new(match access {
          InputDataAccess::Owned => ty.clone(),
          InputDataAccess::Ref | InputDataAccess::Borrowed => Type::Reference(parse_quote!(& #ty)),
          InputDataAccess::MutRef => Type::Reference(parse_quote!(&mut #ty)),
        });
        FnArg::Typed(syn::PatType { attrs: Vec::new(), pat, colon_token: parse_quote!(:), ty })
      },
      None => {
        let reference = match access {
          InputDataAccess::Owned => None,
          _ => Some((parse_quote!(&), None)),
        };
        let mutability = match access {
          InputDataAccess::MutRef => Some(parse_quote!(mut)),
          _ => None,
        };
        let ty = Box::new(ty.clone());
        FnArg::Receiver(syn::Receiver { attrs: Vec::new(), reference, mutability, self_token: parse_quote!(self), colon_token: None, ty})
      },
    }
  }

  pub fn inner_parse(fn_arg: FnArg) -> syn::Result<Self> {
    match fn_arg {
      FnArg::Receiver(syn::Receiver { reference, mutability, ty, .. }) => {
        let access = match (reference, mutability) {
          (None, None) => InputDataAccess::Owned,
          (None, Some(m)) => return Err(syn::Error::new(m.span, "Did you mean 'ref mut'?")),
          (Some((_, None)), None) => InputDataAccess::Ref,
          (Some((_, None)), Some(_)) => InputDataAccess::MutRef,
          (Some((_, Some(l))), _) => return Err(syn::Error::new(l.span(), "Lifetimes are not supported yet")),
        };
        let var_name = None;
        let ty = *ty;
        Ok(Self { access, var_name, ty })
      },
      FnArg::Typed(syn::PatType { pat, mut ty, colon_token, .. }) => {
        let var_name = match *pat {
          syn::Pat::Ident(syn::PatIdent { by_ref: None, ident, subpat: None, .. }) => {
            Some(ident)
          },
          _ => {
            let span = colon_token.span;
            return Err(syn::Error::new(span, "Unsupported naming pattern"))
          },
        };
        while let Type::Paren(syn::TypeParen { elem, .. }) = *ty {
          ty = elem
        }
        let (access, ty) = match *ty {
          Type::Paren(_) => unreachable!(),
          Type::Ptr(syn::TypePtr { star_token, .. }) => {
            return Err(syn::Error::new(star_token.span, "Pointers not recognized yet"))
          },
          Type::Reference(syn::TypeReference { lifetime, mutability, elem, .. }) => {
            if let Some(l) = lifetime {
              return Err(syn::Error::new(l.span(), "Lifetimes are not supported yet"))
            } else {
              let access = mutability.and(Some(InputDataAccess::MutRef)).unwrap_or(InputDataAccess::Ref);
              (access, *elem)
            }
          },
          ty => (InputDataAccess::Owned, ty),
        };
        Ok(Self { access, var_name, ty })
      },
    }
  }
}

impl FnBuilder {
  pub fn get_signature(&self) -> Signature {
    let ident = &self.ident;
    let input = self.input.iter()
      .map(FnArgBuilder::build)
      .collect::<Punctuated<_, Token![,]>>();
    let output = &self.output;
    let generics = &self.generics;
    let mut res: Signature = parse_quote!(fn #ident(#input) #output);
    res.generics = generics.clone();
    res
  }
  
  pub fn get_body(&self) -> Block {
    self.calc_body.as_ref()(self.var_store.as_deref())
  }

  pub fn build(&self) -> ItemFn {
    ItemFn {
      attrs: Vec::new(),
      vis: self.vis.clone(),
      sig: self.get_signature(),
      block: Box::new(self.get_body()),
    }
  }
}



impl From<FnArgBuilder> for FnArg {
  fn from(value: FnArgBuilder) -> Self {
    value.build()
  }
}
impl From<FnBuilder> for ItemFn {
  fn from(value: FnBuilder) -> Self {
    value.build()
  }
}


impl ToTokens for FnArgBuilder {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.build().to_tokens(tokens)
  }
}
impl ToTokens for FnBuilder {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.build().to_tokens(tokens)
  }
}


impl Parse for FnArgBuilder {
  fn parse(input: parse::ParseStream) -> syn::Result<Self> {
    FnArg::parse(input).and_then(Self::inner_parse)
  }
}


impl Parse for FnBuilder {
  fn parse(input: parse::ParseStream) -> syn::Result<Self> {
    fn just_return(x: Option<&dyn ::core::any::Any>) -> Block {
      if let Some(x) = x {
        if let Some(x) = x.downcast_ref::<Block>() {
          x.clone()
        } else {
          parse_quote!({ todo!() })
        }
      }
      else {
        parse_quote!({})
      }
    }

    let ItemFn { attrs, vis, sig, block } = ItemFn::parse(input)?;
    if let Some(attr) = attrs.first() {
      return Err(syn::Error::new(attr.pound_token.span, "Attributes aren't supported yet"));
    }

    let (ident, generics, input, output) = match sig {
      Signature { constness: Some(x), .. } => return Err(syn::Error::new(x.span, "Const not supported yet")),
      Signature { asyncness: Some(x), .. } => return Err(syn::Error::new(x.span, "Async not supported yet")),
      Signature { unsafety: Some(x), .. } => return Err(syn::Error::new(x.span, "Unsafe not supported yet")),
      Signature { abi: Some(x), .. } => return Err(syn::Error::new(x.span(), "Abi not supported yet")),
      Signature { variadic: Some(x), .. } => return Err(syn::Error::new(x.span(), "Variadic not supported yet")),
      Signature { ident, generics, inputs, output, .. } => {
        let inputs = {
          let mut xs = Vec::with_capacity(inputs.len());
          for x in inputs.into_iter().map(FnArgBuilder::inner_parse) {
            xs.push(x?)
          }
          xs
        };
        (ident, generics, inputs, output)
      }
    };

    let var_store: Option<Box<dyn ::core::any::Any>> = Some(block);
    let calc_body = Box::new(just_return);

    Ok(Self { vis, ident, input, output, generics, var_store, calc_body })
  }
}

#[test]
fn parse_fn_arg() {
  use std::str::FromStr;
  use crate::tests::assert_eq_wo_whitespace;

  fn parse_assert(s: &str) {
    parse_assert_against(s, s)
  }

  fn parse_assert_against(s: &str, target: &str) {
    let s_tokens = proc_macro2::TokenStream::from_str(s).unwrap();
    let res: FnArgBuilder = syn::parse2(s_tokens).unwrap();
    assert_eq_wo_whitespace(res.to_token_stream(), target)
  }

  parse_assert("&mut self");
  // parse_assert("mut self");
  parse_assert("&self");
  parse_assert("x: A");
  parse_assert("x: &A");
  parse_assert("x: &mut A");
  parse_assert("x: (A, B)");
  parse_assert("x: (A,)");
  parse_assert_against("x: (A)", "x: A");
}

#[test]
fn parse_fn() {
  use std::str::FromStr;
  use crate::tests::assert_eq_wo_whitespace;

  fn parse_assert(s: &str) {
    parse_assert_against(s, s)
  }

  fn parse_assert_against(s: &str, target: &str) {
    let s_tokens = proc_macro2::TokenStream::from_str(s).unwrap();
    let res: FnBuilder = syn::parse2(s_tokens).unwrap();
    assert_eq_wo_whitespace(res.to_token_stream(), target)
  }

  parse_assert("fn foo_bar(self) {}");
  parse_assert("pub(crate) fn foo_bar(&mut self, x: &A) -> Self { Self(x.value) }");
  parse_assert("pub fn foo_bar<'a>(x: &A) where A: Clone { x.clone(); }");
  parse_assert("fn foo_bar(&self, x: &B) -> Self where B: ::core::borrow::Borrow<Self> { Self(&self.0 + &x.borrow().0) }");
}

