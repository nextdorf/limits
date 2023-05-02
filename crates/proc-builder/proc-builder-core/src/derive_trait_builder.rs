use syn::DeriveInput;

use crate::{AttributeBuilder, TokenStream};

pub struct DeriveTraitBuilder {
  pub attributes: Vec<AttributeBuilder>,
}

impl DeriveTraitBuilder {
  pub fn derive_impl(&self, input: &DeriveInput) -> TokenStream {
    todo!()
  }
}
