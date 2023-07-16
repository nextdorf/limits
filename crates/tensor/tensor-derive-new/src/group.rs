use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse2, punctuated::Punctuated, Token, parse::Parser, spanned::Spanned, visit::Visit, parse_quote};

use crate::util::{return_err, struct_vis::{StructLookup, StructLookupPaths, LookupKind, LookupAccess}};


pub fn group_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
  let mut visitor = StructLookup::new();
  // let q = parse2(input.to_token_stream())?;
  // visitor.visit_expr_reference(&q);
  visitor.visit_derive_input(input);
  let paths = match StructLookupPaths::try_from(visitor) {
    Ok(val) => val,
    Err(_) => return_err!(input.span(), "Could not parse input"),
  };
  // match paths.kind {
  //   LookupKind::Named => todo!(),
  //   LookupKind::Unnamed => todo!(),
  //   LookupKind::Unit => todo!(),    
  //   _ =>  return_err!(input.span(), "..")
  // };

  let mut tstreams = Vec::new();
  let (impl_gen, ty_gen, where_clause) = input.generics.split_for_impl();
  let self_ty = parse2::<syn::Type>(input.ident.to_token_stream())?;

  let add_body = paths.with_access_and_call(&parse_quote!(self), LookupAccess::Own, |_| parse_quote!(add(other)));

  tstreams.push(quote!(
    impl #impl_gen ::core::ops::Add for #self_ty #ty_gen #where_clause {
      type Target = #self_ty;

      fn add(self, other: #self_ty) -> #self_ty {
        Self
      }
    }
  ));
  todo!()

}