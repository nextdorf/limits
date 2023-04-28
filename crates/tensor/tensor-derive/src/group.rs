use proc_macro::TokenStream;
use syn::{DeriveInput, GenericParam, parse_quote, TypeParam};
use quote::quote;

use crate::util::{
  attr_repr::AttrRepr,
  data_quote::{
    DataQuote,
    DataQuotePaths
  },
  group::{
    GenGroupKind,
    GroupDataQuotePaths,
    QuoteParams
  }
};


#[inline]
pub(crate) fn group_wrapper_impl(input: &DeriveInput, kind: GenGroupKind) -> TokenStream {
  let attrs = AttrRepr::new_with(input.attrs.iter());

  let gen_group = match kind {
    GenGroupKind::Abel => &*attrs.gen_abel_group_path,
    GenGroupKind::Mult => &*attrs.gen_group_path,
  };
  let unit_idents = attrs.unit_idents.clone();

  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

  let self_path = parse_quote!(self);
  let rhs_path = parse_quote!(rhs);
  let t_idents = DataQuotePaths::t_idents_from(gen_group, input.generics.params.iter());

  // panic!("{:?}", unit_idents);

  let mut path_choice = GroupDataQuotePaths::new(t_idents.clone(), unit_idents, kind, &attrs);


  let quote_params = QuoteParams::new(&self_path, &rhs_path, ident, &input.data);

  let own_rhs_quote = {
    let (mult_expr, mult_fn_path, mult_trait_path) = quote_params.quote(
      DataQuote::OwnOwn,
      path_choice.mult_expr()
    ).unpack();
    let (ref_mult_expr, ref_mult_fn_path, ref_mult_trait_path) = quote_params.quote(
      DataQuote::RefOwn,
      path_choice.ref_mult_expr()
    ).unpack();
    let (mult_assign_expr, mult_assign_fn_path, mult_assign_trait_path) = quote_params.quote(
      DataQuote::MutOwn,
      path_choice.mult_assign_expr()
    ).unpack();
    let (mult_inv_expr, mult_inv_fn_path, mult_inv_trait_path) = quote_params.quote(
      DataQuote::OwnOwn,
      path_choice.mult_inv_expr()
    ).unpack();
    let (ref_mult_inv_expr, ref_mult_inv_fn_path, ref_mult_inv_trait_path) = quote_params.quote(
      DataQuote::RefOwn,
      path_choice.ref_mult_inv_expr()
    ).unpack();
    let (mult_assign_inv_expr, mult_assign_inv_fn_path, mult_assign_inv_trait_path) = quote_params.quote(
      DataQuote::MutOwn,
      path_choice.mult_assign_inv_expr()
    ).unpack();

    quote!{
      // Addition +++++++++++++++++++++++++++++++++++++++++++
      impl #impl_gen #mult_trait_path<#ident #type_gen> for #ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #mult_fn_path(self, rhs: #ident #type_gen) -> #ident #type_gen {
          #mult_expr
        }
      }
      
      impl #impl_gen #ref_mult_trait_path<#ident #type_gen> for &#ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #ref_mult_fn_path(self, rhs: #ident #type_gen) -> #ident #type_gen {
          #ref_mult_expr
        }
      }
      
      impl #impl_gen #mult_assign_trait_path<#ident #type_gen> for #ident #type_gen #where_clause {
        #[inline]
        fn #mult_assign_fn_path(&mut self, rhs: #ident #type_gen) {
          #mult_assign_expr
        }
      }

      // Subtraction ++++++++++++++++++++++++++++++++++++++++
      impl #impl_gen #mult_inv_trait_path<#ident #type_gen> for #ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #mult_inv_fn_path(self, rhs: #ident #type_gen) -> #ident #type_gen {
          #mult_inv_expr
        }
      }
      
      impl #impl_gen #ref_mult_inv_trait_path<#ident #type_gen> for &#ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #ref_mult_inv_fn_path(self, rhs: #ident #type_gen) -> #ident #type_gen {
          #ref_mult_inv_expr
        }
      }
      
      impl #impl_gen #mult_assign_inv_trait_path<#ident #type_gen> for #ident #type_gen #where_clause {
        #[inline]
        fn #mult_assign_inv_fn_path(&mut self, rhs: #ident #type_gen) {
          #mult_assign_inv_expr
        }
      }
    }
  }.into();


  let unary_quote = {
    let (unit_expr, unit_fn_path, unit_trait_path) = quote_params.quote(
      DataQuote::Unit,
      path_choice.unit_expr()
    ).unpack();
    let (set_unit_expr, set_unit_fn_path, _) = quote_params.quote(
      DataQuote::Mut,
      path_choice.set_unit_expr()
    ).unpack();
    let (is_unit_expr, is_unit_fn_path, _) = quote_params.chain_bool(
      DataQuote::Ref,
      path_choice.is_unit_expr()
    ).unpack();
    let (inv_expr, inv_fn_path, inv_trait_path) = quote_params.quote(
      DataQuote::Own,
      path_choice.inv_expr()
    ).unpack();
    let (ref_inv_expr, ref_inv_fn_path, ref_inv_trait_path) = quote_params.quote(
      DataQuote::Ref,
      path_choice.ref_inv_expr()
    ).unpack();

    quote! {
      // Invert / Negate +++++++++++++++++++++++++++++++++++
      impl #impl_gen #inv_trait_path for #ident #type_gen #where_clause {
        type Output = #ident #type_gen;

        #[inline]
        fn #inv_fn_path(self) -> #ident #type_gen {
          #inv_expr
        }
      }

      impl #impl_gen #ref_inv_trait_path for &#ident #type_gen #where_clause {
        type Output = #ident #type_gen;

        #[inline]
        fn #ref_inv_fn_path(self) -> #ident #type_gen {
          #ref_inv_expr
        }
      }

      // One / Zero +++++++++++++++++++++++++++++++++++++++++
      impl #impl_gen #unit_trait_path for #ident #type_gen #where_clause {
        #[inline]
        fn #unit_fn_path() -> Self {
          #unit_expr
        }

        #[inline]
        fn #set_unit_fn_path(&mut self) {
          #set_unit_expr
        }
    
        #[inline]
        fn #is_unit_fn_path(&self) -> bool {
          #is_unit_expr
        }
      }
    }
  }.into();

  let b_gen: TypeParam = parse_quote!(B: std::borrow::Borrow<#ident #type_gen>);
  let b_ident: syn::Ident = b_gen.ident.clone();
  let mut generics_ref = input.generics.clone();
  generics_ref.params.push(GenericParam::Type(b_gen));
  let (impl_gen, _, where_clause) = generics_ref.split_for_impl();

  let bor_rhs_quote = {
    let (mult_bor_expr, mult_bor_fn_path, mult_bor_trait_path) = quote_params.quote(
      DataQuote::OwnBor,
      path_choice.mult_bor_expr()
    ).unpack();
    let (ref_mult_bor_expr, ref_mult_bor_fn_path, ref_mult_bor_trait_path) = quote_params.quote(
      DataQuote::RefBor,
      path_choice.ref_mult_bor_expr()
    ).unpack();
    let (mult_assign_bor_expr, mult_assign_bor_fn_path, mult_assign_bor_trait_path) = quote_params.quote(
      DataQuote::MutBor,
      path_choice.mult_assign_bor_expr()
    ).unpack();
    let (mult_inv_bor_expr, mult_inv_bor_fn_path, mult_inv_bor_trait_path) = quote_params.quote(
      DataQuote::OwnBor,
      path_choice.mult_inv_bor_expr()
    ).unpack();
    let (ref_mult_inv_bor_expr, ref_mult_inv_bor_fn_path, ref_mult_inv_bor_trait_path) = quote_params.quote(
      DataQuote::RefBor,
      path_choice.ref_mult_inv_bor_expr()
    ).unpack();
    let (mult_assign_inv_bor_expr, mult_assign_inv_bor_fn_path, mult_assign_inv_bor_trait_path) = quote_params.quote(
      DataQuote::MutBor,
      path_choice.mult_assign_inv_bor_expr()
    ).unpack();

    quote! {
      // Addition +++++++++++++++++++++++++++++++++++++++++++
      impl #impl_gen #mult_bor_trait_path<&#b_ident> for #ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #mult_bor_fn_path(self, rhs: &#b_ident) -> #ident #type_gen {
          #mult_bor_expr
        }
      }

      impl #impl_gen #ref_mult_bor_trait_path<&#b_ident> for &#ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #ref_mult_bor_fn_path(self, rhs: &#b_ident) -> #ident #type_gen {
          #ref_mult_bor_expr
        }
      }

      impl #impl_gen #mult_assign_bor_trait_path<&#b_ident> for #ident #type_gen #where_clause {
        #[inline]
        fn #mult_assign_bor_fn_path(&mut self, rhs: &#b_ident) {
          #mult_assign_bor_expr
        }
      }

      // Subtraction ++++++++++++++++++++++++++++++++++++++++
      impl #impl_gen #mult_inv_bor_trait_path<&#b_ident> for #ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #mult_inv_bor_fn_path(self, rhs: &#b_ident) -> #ident #type_gen {
          #mult_inv_bor_expr
        }
      }

      impl #impl_gen #ref_mult_inv_bor_trait_path<&#b_ident> for &#ident #type_gen #where_clause {
        type Output = #ident #type_gen;
      
        #[inline]
        fn #ref_mult_inv_bor_fn_path(self, rhs: &#b_ident) -> #ident #type_gen {
          #ref_mult_inv_bor_expr
        }
      }

      impl #impl_gen #mult_assign_inv_bor_trait_path<&#b_ident> for #ident #type_gen #where_clause {
        #[inline]
        fn #mult_assign_inv_bor_fn_path(&mut self, rhs: &#b_ident) {
          #mult_assign_inv_bor_expr
        }
      }
    }
  }.into();


  <_ as std::iter::FromIterator<TokenStream>>::from_iter([
    own_rhs_quote,
    unary_quote,
    bor_rhs_quote,
  ])
}


