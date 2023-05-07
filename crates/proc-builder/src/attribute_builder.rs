use std::{collections::HashMap, any::Any};

use proc_macro2::TokenStream;
use syn::{Path, Expr, Attribute, Meta, punctuated::Punctuated, Token, parse_quote};

pub type DynLookup = HashMap<String, Box<dyn Any>>;

#[derive(Default)]
pub struct AttributesBuilder {
  pub attrs: Vec<AttrMetaBuilder<DynLookup>>,
}

pub enum AttrMetaBuilder<State> {
  Path(Path, Box<dyn Fn(&mut State)>),
  List(Path, Box<dyn Fn(&mut State, &dyn Iterator<Item = TokenStream>)>),
  NameValue(Path, Box<dyn Fn(&mut State, &Expr)>),
}

pub trait DynLookupExt {
  type ErrT;

  fn lookup<'a, T: Any>(&'a self, at: impl ToString) -> Result<&'a T, Self::ErrT>;
  fn lookup_mut<'a, T: Any>(&'a mut self, at: impl ToString) -> Result<&'a mut T, Self::ErrT>;
}

pub enum LookupErr {
  KeyNotFound,
  WrongTypeCast,
}


impl AttributesBuilder {
  pub const fn new() -> Self {
    Self { attrs: Vec::new() }
  }

  pub fn update(&self, lookup: &mut DynLookup, attrs: &Vec<Attribute>) {
    for attr in &self.attrs {
      attr.update_state(lookup, attrs)
    }
  }

  pub fn generate_lookup(&self, attrs: &Vec<Attribute>) -> DynLookup {
    let mut res = DynLookup::new();
    self.update(&mut res, attrs);
    res
  }
}

impl DynLookupExt for DynLookup {
  type ErrT = LookupErr;

  fn lookup<'a, T: Any>(&'a self, at: impl ToString) -> Result<&'a T, Self::ErrT> {
    match self.get(&at.to_string()) {
      Some(val) => match val.downcast_ref::<T>() {
        Some(val) => Ok(val),
        None => Err(LookupErr::WrongTypeCast),
      },
      None => Err(LookupErr::KeyNotFound),
    }
  }

  fn lookup_mut<'a, T: Any>(&'a mut self, at: impl ToString) -> Result<&'a mut T, Self::ErrT> {
    match self.get_mut(&at.to_string()) {
      Some(val) => match val.downcast_mut::<T>() {
        Some(val) => Ok(val),
        None => Err(LookupErr::WrongTypeCast),
      },
      None => Err(LookupErr::KeyNotFound),
    }
  }
}


impl<S> AttrMetaBuilder<S> {
  pub fn update_state<'a>(&self, state: &mut S, attrs: impl IntoIterator<Item = &'a Attribute>) {
    fn cmp(p1: &Path, p2: &Path) -> bool {
      if p1.segments.len() != p2.segments.len() {
        return false;
      }
      p1.segments.iter()
        .zip(&p2.segments)
        .find(|(x, y)| x.ident != y.ident)
        .and(Some(false))
        .unwrap_or(true)
    }
    let path = self.path();
    for attr in attrs {
      if !cmp(path, attr.path()) {
        continue;
      }
      match (self, &attr.meta) {
        (AttrMetaBuilder::Path(_, f), Meta::Path(_)) => f(state),
        (AttrMetaBuilder::List(_, f), Meta::List(xs)) => {
          let tokens = &xs.tokens;
          let tokens: Punctuated<TokenStream, Token![,]> = parse_quote!(#tokens);
          f(state, &tokens.into_iter())
        },
        (AttrMetaBuilder::NameValue(_, f), Meta::NameValue(x)) => {
          f(state, &x.value)
        },
        _ => {}
      }
    }
  }

  const fn path(&self) -> &Path {
    match self {
      AttrMetaBuilder::Path(p, _) => p,
      AttrMetaBuilder::List(p, _) => p,
      AttrMetaBuilder::NameValue(p, _) => p,
    }
  }
}


impl<S> ToString for AttrMetaBuilder<S> {
  fn to_string(&self) -> String {
    let path = self.path().segments.iter()
      .map(|x| x.ident.to_string())
      .collect::<Vec<_>>()
      .join("::");
    match self {
      AttrMetaBuilder::Path(_, _) => format!("#[{}]", path),
      AttrMetaBuilder::List(_, _) => format!("#[{}(...)]", path),
      AttrMetaBuilder::NameValue(_, _) => format!("#[{} = ...]", path),
    }
  }
}


impl ToString for AttributesBuilder {
  fn to_string(&self) -> String {
    let attrs = self.attrs.iter()
      .map(ToString::to_string)
      .collect::<Vec<_>>()
      .join(", ");
    format!("{{{}}}", attrs)
  }
}


