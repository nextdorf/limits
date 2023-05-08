use std::{collections::HashMap, any::Any, rc::Rc};

use proc_macro2::TokenStream;
use syn::{Path, Expr, Attribute, Meta, punctuated::Punctuated, Token, parse_quote};

pub struct DynLookup(HashMap<String, (Box<dyn Any>, &'static dyn Fn(&dyn Any) -> Box<dyn Any>)>);



#[derive(Default)]
pub struct AttributesBuilder {
  pub attrs: Vec<AttrMetaBuilder<DynLookup>>,
}

#[derive(Clone)]
pub enum AttrMetaBuilder<State> {
  Path(Path, Rc<dyn Fn(&mut State) -> syn::Result<()>>),
  List(Path, Rc<dyn Fn(&mut State, Box<dyn Iterator<Item = TokenStream>>) -> syn::Result<()>>),
  NameValue(Path, Rc<dyn Fn(&mut State, &Expr) -> syn::Result<()>>),
}

pub enum LookupErr {
  KeyNotFound,
  WrongTypeCast,
}


impl AttributesBuilder {
  pub const fn new() -> Self {
    Self { attrs: Vec::new() }
  }

  pub fn update(&self, lookup: &mut DynLookup, attrs: &Vec<Attribute>) -> syn::Result<()> {
    for attr in &self.attrs {
      attr.update_state(lookup, attrs)?
    }
    Ok(())
  }

  pub fn generate_lookup(&self, attrs: &Vec<Attribute>) -> syn::Result<DynLookup> {
    let mut res = DynLookup::new();
    self.update(&mut res, attrs)?;
    Ok(res)
  }
}

impl DynLookup {
  pub fn new() -> Self {
    Self(HashMap::new())
  }

  pub fn insert<T: Clone + Any>(&mut self, k: String, v: T) -> Option<Box<dyn Any>> {
    fn clone<T: Clone + Any>(x: &dyn Any) -> Box<dyn Any> {
      Box::new(x.downcast_ref::<T>().unwrap().clone())
    }
    let v: (Box<dyn Any>, &'static dyn Fn(&dyn Any) -> Box<dyn Any>) = (Box::new(v), &clone::<T>);
    match self.0.insert(k, v) {
      Some((v, _)) => Some(v),
      None => None,
    }
  }

  pub fn get<'a, T: Any + Clone>(&'a self, at: impl ToString) -> Result<&'a T, LookupErr> {
    match self.0.get(&at.to_string()) {
      Some((val, _)) => match val.downcast_ref::<T>() {
        Some(val) => Ok(val),
        None => Err(LookupErr::WrongTypeCast),
      }
      None => Err(LookupErr::KeyNotFound),
    }
  }

  pub fn get_mut<'a, T: Any + Clone>(&'a mut self, at: impl ToString) -> Result<&'a mut T, LookupErr> {
    match self.0.get_mut(&at.to_string()) {
      Some((val, _)) => match val.downcast_mut::<T>() {
        Some(val) => Ok(val),
        None => Err(LookupErr::WrongTypeCast),
      }
      None => Err(LookupErr::KeyNotFound),
    }
  }
}


impl<S> AttrMetaBuilder<S> {
  pub fn update_state<'a>(&self, state: &mut S, attrs: impl IntoIterator<Item = &'a Attribute>) -> syn::Result<()> {
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
        (AttrMetaBuilder::Path(_, f), Meta::Path(_)) => f(state)?,
        (AttrMetaBuilder::List(_, f), Meta::List(xs)) => {
          let tokens = &xs.tokens;
          let tokens: Punctuated<TokenStream, Token![,]> = parse_quote!(#tokens);
          f(state, Box::new(tokens.into_iter()))?
        },
        (AttrMetaBuilder::NameValue(_, f), Meta::NameValue(x)) => {
          f(state, &x.value)?
        },
        _ => {}
      }
    }
    Ok(())
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


// impl Deref for DynLookup {
//   type Target = HashMap<String, Box<dyn Any>>;
//   fn deref(&self) -> &Self::Target { &self.0 }
// }
// impl DerefMut for DynLookup {
//   fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
// }


impl Clone for DynLookup {
  fn clone(&self) -> Self {
    let inner = self.0.iter().map(|(k, (v, clone))| {
      let v = clone(v);
      (k.clone(), (v, *clone))
    }).collect();
    Self(inner)
  }
}
