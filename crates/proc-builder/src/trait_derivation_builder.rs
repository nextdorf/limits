use std::rc::Rc;

use syn::{DeriveInput, ItemFn, Attribute};

use crate::{AttributesBuilder, FnBuilder, DynLookup};

pub struct TraitDerivationBuilder {
  pub attributes: AttributesBuilder,
  pub functions: Vec<FnBuilder>,
  pub apply_lookup: Rc<dyn Fn(&DynLookup, &mut Vec<FnBuilder>)>,
}

pub struct TraitDerivation<'a> {
  pub input: &'a DeriveInput,
  pub functions: Vec<ItemFn>,
}

impl TraitDerivationBuilder {
  pub fn new() -> Self {
    fn noop(_: &DynLookup, _: &mut Vec<FnBuilder>) {}
    Self {
      attributes: AttributesBuilder::new(),
      functions: Vec::new(),
      apply_lookup: Rc::new(noop),
    }
  }

  pub fn populate_lookup(&self, lookup: &mut DynLookup, attrs: &Vec<Attribute>) {
    self.attributes.update(lookup, attrs);
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
    let functions = self.functions(lookup);
    TraitDerivation { input, functions }
  }

  pub fn derive_impls_with_lookup<'a>(builders: Vec<&Self>, input: &'a DeriveInput, lookup: &mut DynLookup) -> Vec<TraitDerivation<'a>> {
    for b in &builders {
      b.populate_lookup(lookup, &input.attrs)
    }
    builders.iter().map(|b| b.derive_impl(&lookup, input)).collect()
  }

  pub fn derive_impls<'a>(builders: Vec<&Self>, input: &'a DeriveInput) -> (Vec<TraitDerivation<'a>>, DynLookup) {
    let mut lookup = DynLookup::new();
    let traits = Self::derive_impls_with_lookup(builders, input, &mut lookup);
    (traits, lookup)
  }
}



