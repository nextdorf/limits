use syn::{DeriveInput, ItemFn};

use crate::{AttributeBuilder, TokenStream};

pub struct DeriveTraitBuilder {
  pub attributes: Vec<AttributeBuilder>,
  pub functions: Vec<ItemFn>,
}

impl DeriveTraitBuilder {
  pub fn derive_impl(&self, _input: &DeriveInput) -> TokenStream {
    todo!()
  }
}
