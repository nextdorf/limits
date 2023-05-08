use std::{rc::Rc, str::FromStr};

use quote::{ToTokens, quote};
use syn::{DeriveInput, ItemFn, Attribute, Ident};

use crate::{AttributesBuilder, FnBuilder, DynLookup, AttrMetaBuilder};

pub struct TraitDerivationBuilder {
  pub ident: Ident,
  pub attributes: AttributesBuilder,
  pub functions: Vec<FnBuilder>,
  pub apply_lookup: Rc<dyn Fn(&DynLookup, &mut Vec<FnBuilder>)>,
}

pub struct TraitDerivation<'a> {
  pub ident: Ident,
  pub input: &'a DeriveInput,
  pub functions: Vec<ItemFn>,
}

impl TraitDerivationBuilder {
  pub fn new(ident: Ident) -> Self {
    fn noop(_: &DynLookup, _: &mut Vec<FnBuilder>) {}
    Self {
      ident,
      attributes: AttributesBuilder::new(),
      functions: Vec::new(),
      apply_lookup: Rc::new(noop),
    }
  }


  pub fn override_ident(mut self, ident: Ident) -> Self {
    self.ident = ident;
    self
  }

  pub fn set_ident(self, ident: impl ToString) -> Self {
    self.override_ident(syn::parse_str(ident.to_string().as_str()).unwrap())
  }

  pub fn push_attribute(self, attr: AttrMetaBuilder<DynLookup>) -> Self {
    self.push_attributes(Some(attr).into_iter())
  }
  pub fn push_attributes(mut self, attrs: impl Iterator<Item = AttrMetaBuilder<DynLookup>>) -> Self {
    for a in attrs {
      self.attributes.attrs.push(a);
    }
    self
  }

  pub fn push_function(self, f: FnBuilder) -> Self {
    self.push_functions(Some(f).into_iter())
  }
  pub fn push_functions(mut self, fs: impl Iterator<Item = FnBuilder>) -> Self {
    for f in fs {
      self.functions.push(f);
    }
    self
  }

  pub fn set_lookup_action(mut self, f: impl Fn(&DynLookup, &mut Vec<FnBuilder>) + 'static) -> Self {
    self.apply_lookup = Rc::new(f);
    self
  }


  pub fn populate_lookup(&self, lookup: &mut DynLookup, attrs: &Vec<Attribute>) -> syn::Result<()> {
    self.attributes.update(lookup, attrs)
  }

  pub fn function_builders(&self, lookup: &DynLookup) -> Vec<FnBuilder> {
    let mut res = self.functions.clone();
    (self.apply_lookup)(lookup, &mut res);
    res
  }

  pub fn functions(&self, lookup: &DynLookup) -> Vec<ItemFn> {
    self.function_builders(lookup).iter().map(FnBuilder::build).collect()
  }

  pub fn derive_impl<'a>(&self, lookup: &DynLookup, input: &'a DeriveInput) -> TraitDerivation<'a> {
    let ident = self.ident.clone();
    let functions = self.functions(lookup);
    TraitDerivation { ident, input, functions }
  }

  pub fn derive_impls_with_lookup<'a>(builders: Vec<&Self>, input: &'a DeriveInput, lookup: &mut DynLookup) -> syn::Result<Vec<TraitDerivation<'a>>> {
    for b in &builders {
      b.populate_lookup(lookup, &input.attrs)?
    }
    let res = builders.iter().map(|b| b.derive_impl(&lookup, input)).collect();
    Ok(res)
  }

  pub fn derive_impls<'a>(builders: Vec<&Self>, input: &'a DeriveInput) -> syn::Result<(Vec<TraitDerivation<'a>>, DynLookup)> {
    let mut lookup = DynLookup::new();
    let traits = Self::derive_impls_with_lookup(builders, input, &mut lookup)?;
    Ok((traits, lookup))
  }
}


impl FromStr for TraitDerivationBuilder {
  type Err = syn::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ident = syn::parse_str(s)?;
    Ok(Self::new(ident))
  }
}


impl ToTokens for TraitDerivation<'_> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let t_ident = &self.ident;
    let ident = &self.input.ident;
    let (impl_gen, ty_gen, where_cl) = self.input.generics.split_for_impl();
    let mut fns = proc_macro2::TokenStream::new();
    for f in &self.functions {
      f.to_tokens(&mut fns)
    }
    tokens.extend(quote!(
      impl #impl_gen #t_ident for #ident #ty_gen #where_cl {
        #fns
      }
    ));
    
  }
}

// impl syn::TraitItem