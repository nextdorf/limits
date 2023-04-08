use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};

#[inline]
pub(crate) fn wrapper_deref_impl(input: &DeriveInput) -> TokenStream {
  // TODO: T should be self.0 instead of unique type_parameter
  let mut type_params = input.generics.type_params();
  let impl_t = {
    match type_params.next() {
      Some(res) => match type_params.next() {
        Some(_) =>
          return syn::Error::new(input.span(), "To many Type Parameters").to_compile_error().into(),
        None => res,
      },
      None =>
        return syn::Error::new(input.span(), "No Type Parameter to Wrap").to_compile_error().into(),
    }
  };

  let t_ident = &impl_t.ident;

  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

  quote! {
    impl #impl_gen std::ops::Deref for #ident #type_gen #where_clause {
      type Target = #t_ident;
    
      fn deref(&self) -> &#t_ident {
        &self.0
      }
    }

    impl #impl_gen std::ops::DerefMut for #ident #type_gen #where_clause {
      fn deref_mut(&mut self) -> &mut #t_ident {
        &mut self.0
      }
    }
  }.into()
}



