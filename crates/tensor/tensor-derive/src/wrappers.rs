use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, spanned::Spanned, punctuated::Punctuated, Token};
use crate::macro_err;

#[inline]
pub(crate) fn wrapper_deref_impl(input: &DeriveInput) -> TokenStream {
  let wrap_deref_attr = input.attrs.iter().find(|a| a.path().to_token_stream().to_string() == "wrap_deref");
  // panic!("{}", wrap_deref_attr.to_token_stream().to_string());
  let (t_field, t_idx) = match &input.data {
    syn::Data::Struct(s) => {
      let fields_tokens = match &s.fields {
        syn::Fields::Unnamed(fs) => &fs.unnamed,
        syn::Fields::Named(fs) => return macro_err!(fs, "Only unnamed fields are supported"),
        _ => return macro_err!(input, "There is no Field to Wrap"),
      };

      let idx = if let Some(a) = wrap_deref_attr {
        match &a.meta {
          syn::Meta::Path(_) => Some(0),
          syn::Meta::List(idxs) => {
            let idxs = &idxs.tokens;
            // panic!("{}", idxs.to_token_stream().to_string());
            let idxs: Punctuated::<syn::Index, Token![,]> = syn::parse_quote!(#idxs);
            let mut idxs = idxs.into_iter();
            if let Some(idx) = idxs.next() {
              if let Some(x) = idxs.next() {
                return macro_err!(x, "To many fields, only one field can be derefed");
              } else {
                Some(idx.index as _)
              }
            } else {
              None
            }
          },
          syn::Meta::NameValue(_) => None,
      }} else {
        None
      };

      if let Some(idx) = idx {
        match fields_tokens.iter().nth(idx) {
          Some(f) => (f, idx),
          None => {
            let idx = syn::Index::from(idx);
            return macro_err!(idx, "Too few fields, specified field can't be derefed")
          },
        }
      } else {
        let mut fields = fields_tokens.iter();
        match fields.next() {
          Some(res) => match fields.next() {
            Some(x) => return macro_err!(x, "Too many fields, only one field can be derefed"),
            None => (res, 0),
          },
          None => return macro_err!(fields_tokens, "No field to wrap"),
        }
      }
    },
    _ => return macro_err!(input, "Only unnamed structs are supported!")
  };
  let t_idx = syn::Index::from(t_idx);

  let is_pub = if let syn::Visibility::Public(_) = t_field.vis {
    true
  } else {
    false
  };
  let t = &t_field.ty;
  // panic!("{}", t_field.to_token_stream());

  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

  let mut res = vec![
    quote! {
      impl #impl_gen std::ops::Deref for #ident #type_gen #where_clause {
        type Target = #t;
        
        fn deref(&self) -> &#t {
          &self.#t_idx
        }
      }

      impl #impl_gen #ident #type_gen #where_clause {
        // pub fn from<U: Into<#t>>(value: U) -> Self {
        //   Self(value.into())
        // }

        pub fn into<U>(self) -> U where #t: Into<U> {
          self.#t_idx.into()
        }
      }
    }.into()
  ];
  if is_pub {
    res.push(
      quote! {
        impl #impl_gen std::ops::DerefMut for #ident #type_gen #where_clause {
          fn deref_mut(&mut self) -> &mut #t {
            &mut self.#t_idx
          }
        }
      }.into()
    );
  }

  <_ as std::iter::FromIterator<TokenStream>>::from_iter(res)
}



