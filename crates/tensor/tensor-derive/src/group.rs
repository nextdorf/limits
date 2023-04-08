use proc_macro::TokenStream;
use syn::{DeriveInput, GenericParam, parse_quote, TypeParam};
use quote::quote;

#[inline]
pub(crate) fn group_wrapper_impl(input: &DeriveInput) -> TokenStream {
  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

  let own_lhs_quote = quote! {
    impl #impl_gen std::ops::Add<#ident #type_gen> for #ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn add(self, rhs: #ident #type_gen) -> #ident #type_gen {
        #ident(self.0.add(rhs.0))
      }
    }

    impl #impl_gen std::ops::Add<#ident #type_gen> for &#ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn add(self, rhs: #ident #type_gen) -> #ident #type_gen {
        #ident(self.0.ref_add(rhs.0))
      }
    }
  }.into();

  let b_gen: TypeParam = parse_quote!(B: std::borrow::Borrow<#ident #type_gen>);
  let b_ident: syn::Ident = b_gen.ident.clone();
  let mut generics_ref = input.generics.clone();
  generics_ref.params.push(GenericParam::Type(b_gen));
  let (impl_gen, _, where_clause) = generics_ref.split_for_impl();

  let bor_lhs_quote = quote! {
    impl #impl_gen std::ops::Add<&#b_ident> for #ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn add(self, rhs: &#b_ident) -> #ident #type_gen {
        #ident(self.0.add_ref(&rhs.borrow().0))
      }
    }

    impl #impl_gen std::ops::Add<&#b_ident> for &#ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn add(self, rhs: &#b_ident) -> #ident #type_gen {
        #ident(self.0.ref_add_ref(&rhs.borrow().0))
      }
    }
  }.into();

  <_ as std::iter::FromIterator<TokenStream>>::from_iter([own_lhs_quote, bor_lhs_quote])
}


