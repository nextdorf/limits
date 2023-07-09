use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, spanned::Spanned, Ident, parse2, Type, punctuated::Punctuated, Token, parse::Parser};

use crate::util::return_err;


pub fn wrapper_deref_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
  const WRAP_DEREF_STR: &str = "wrapper_deref";
  let s_fields = match &input.data {
    syn::Data::Struct(syn::DataStruct {fields, ..}) => match fields {
      syn::Fields::Named(syn::FieldsNamed {named, ..}) if named.len() > 0 => fields,
      syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, ..}) if unnamed.len() > 0 => fields,
      _ => return_err!(input.span(), "There needs to be at least one element in the struct"),
    },
    _ => return_err!(input.span(), "Only structs can be wrapped"),
  };
  let wrap_deref_attr = input.attrs.iter().find(|a| a.path().is_ident(WRAP_DEREF_STR));
  let target_idx = match &wrap_deref_attr {
    Some(attr) => match &attr.meta {
      syn::Meta::Path(_) => 0,
      syn::Meta::NameValue(syn::MetaNameValue {value, ..}) => { //TODO: key-value macros are not supported ATM :(
        let ident = syn::parse2::<Ident>(value.to_token_stream())?;
        let found_field = s_fields.iter().enumerate().find(|(_, f)| *f.ident.as_ref().unwrap() == ident);
        match found_field {
          Some((idx, _)) => idx,
          None => return_err!(ident.span(), "{}::{} not found", &input.ident, &ident),
        }
      },
      syn::Meta::List(syn::MetaList {tokens, ..}) => { //TODO: Remove this once key-value macros are supported
        let l = Punctuated::<proc_macro2::TokenStream, Token![,]>::parse_terminated.parse2(tokens.clone())?;
        let mut l = l.into_iter();
        if let Some(ident) = l.next() {
          if let Some(ident) = l.next() {
            return_err!(ident.span(), "Too many elements to wrap")
          }
          let found_field = match s_fields {
            syn::Fields::Named(_) => {
              let ident = parse2::<Ident>(ident.clone())?;
              let found_field = s_fields.iter().enumerate().find(|(_, f)| *f.ident.as_ref().unwrap() == ident);
              if found_field.is_none() {
                return_err!(ident.span(), "{}::{} not found", &input.ident, &ident);
              }
              found_field
            },
            syn::Fields::Unnamed(_) => {
              let idx = parse2::<syn::Index>(ident.clone())?.index as usize;
              s_fields.iter().enumerate().nth(idx)
            },
            syn::Fields::Unit => unreachable!(),
          };
          match found_field {
            Some((idx, _)) => idx,
            None => return_err!(ident.span(), "{}::{} not found", &input.ident, &ident),
          }
        } else {
          0
        }
      },
      // syn::Meta::List(l) => { //TODO: Use this once key-value macros are supported
      //   return_err!(l.span(), "Can not be a list. Use {WRAP_DEREF_STR} = .. instead")
      // },
    },
    None if s_fields.len() == 1 => 0,
    // None => return_err!(s_fields.span(), "Only one element can be wrapped. Consider adding #[{WRAP_DEREF_STR} {{ = .. }}?] and specifying the wrapped element")
    None => return_err!(s_fields.span(), "Only one element can be wrapped. Consider adding #[{WRAP_DEREF_STR} ( .. )?] and specifying the wrapped element")
  };
  let target = match s_fields.iter().nth(target_idx) {
    Some(t) => t,
    None => return_err!(input.span(), "{}th element of {} not found", target_idx, &input.ident),
  };
  // panic!("Here: {}", target_idx.to_token_stream());

  let is_pub = match target.vis {
    syn::Visibility::Public(_) => true,
    _ => false,
  };

  let (impl_gen, ty_gen, where_clause) = input.generics.split_for_impl();
  let self_ty = parse2::<Type>(input.ident.to_token_stream())?;
  let target_ty = &target.ty;
  // let target_path = target.ident.as_ref().unwrap();
  let target_path = match s_fields {
    syn::Fields::Named(_) => target.ident.as_ref().unwrap().to_token_stream(),
    syn::Fields::Unnamed(_) => syn::Index::from(target_idx).into_token_stream(),
    syn::Fields::Unit => unreachable!(),
  };

  let mut wrapper_impls = Vec::<syn::ItemImpl>::new();
  wrapper_impls.push(parse2(quote!(
    impl #impl_gen ::core::ops::Deref for #self_ty #ty_gen #where_clause {
      type Target = #target_ty;

      fn deref(&self) -> &#target_ty {
        &self.#target_path
      }
    }
  ))?);
  if is_pub {
    wrapper_impls.push(parse2(quote!(
      impl #impl_gen ::core::ops::DerefMut for #self_ty #ty_gen #where_clause {
        fn deref_mut(&mut self) -> &mut #target_ty {
          &mut self.#target_path
        }
      }
    ))?);
  }

  Ok(TokenStream::from_iter(wrapper_impls.into_iter().map(|i| TokenStream::from(i.into_token_stream()))))
}






