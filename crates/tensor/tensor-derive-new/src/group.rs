use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, parse2, punctuated::Punctuated, Token, parse::Parser, spanned::Spanned, visit::Visit};

use crate::util::{return_err, struct_vis::StructLookup};


pub fn group_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
  let mut visitor = StructLookup::new();
  let q = parse2(input.to_token_stream())?;
  visitor.visit_expr_reference(&q);
  return_err!(input.span(), "..")
}