use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};
use crate::macro_err;

#[inline]
pub(crate) fn wrapper_deref_impl(input: &DeriveInput) -> TokenStream {
  let t_field = match &input.data {
    syn::Data::Struct(s) => {
      let fields_tokens = match &s.fields {
        syn::Fields::Unnamed(fs) => &fs.unnamed,
        syn::Fields::Named(fs) => return macro_err!(fs, "Only unnamed fields are supported"),
        _ => return macro_err!(input, "There is no Field to Wrap"),
      };
      let mut fields = fields_tokens.iter();
      match fields.next() {
        Some(res) => match fields.next() {
          Some(x) => return macro_err!(x, "To many fields, only one field can be derefed"),
          None => res,
        },
        None => return macro_err!(fields_tokens, "No field to wrap"),
      }
    },
    _ => return macro_err!(input, "Only unnamed structs are supported!")
  };
  let is_pub = if let syn::Visibility::Public(_) = t_field.vis {
    true
  } else {
    false
  };
  let t = &t_field.ty;

  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

  let mut res = vec![
    quote! {
      impl #impl_gen std::ops::Deref for #ident #type_gen #where_clause {
        type Target = #t;
        
        fn deref(&self) -> &#t {
          &self.0
        }
      }
    }.into()
  ];
  if is_pub {
    res.push(
      quote! {
        impl #impl_gen std::ops::DerefMut for #ident #type_gen #where_clause {
          fn deref_mut(&mut self) -> &mut #t {
            &mut self.0
          }
        }
      }.into()
    );
  }

  <_ as std::iter::FromIterator<TokenStream>>::from_iter(res)
}



