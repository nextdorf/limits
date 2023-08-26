use quote::quote;
use syn::DeriveInput;

use crate::util::{get_neg_path, get_zero_path};

use super::common::{
  TokenStream1,
  GroupAuxImpl,
  GroupAuxBody,
  GroupImplStore,
  GroupAuxSyn
};


pub fn abel_group_impl(input: &DeriveInput) -> syn::Result<TokenStream1> {
  let aux_impl = GroupAuxImpl::new(input)?;
  let aux_body_impl = GroupAuxBody::new(&aux_impl)?;
  let mut impl_store = GroupImplStore::default();

  let GroupAuxSyn {
    self_ty,
    impl_gen,
    ty_gen,
    where_clause,
    param1
  } = aux_impl.get_syn_values();
  let _syn_borrow_values = aux_impl.get_syn_borrow_values();
  let (
    b_ty,
    b_impl_gen,
    b_where_clause
  ) = _syn_borrow_values.get();

  let zero_path = get_zero_path(&input.attrs);
  let neg_path = get_neg_path(&input.attrs);


  impl_store.push_impl({
    let body = aux_body_impl.mult_body();
    quote!(
      impl #impl_gen ::core::ops::Add<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn add(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_body();
    quote!(
      impl #impl_gen ::core::ops::Add<#self_ty #ty_gen> for &#self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn add(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Add<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn add(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Add<&#b_ty> for &#self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn add(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_body();
    quote!(
      impl #impl_gen ::core::ops::AddAssign<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        #[inline]
        fn add_assign(&mut self, #param1: #self_ty #ty_gen) {
          #(#body);*
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::AddAssign<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        #[inline]
        fn add_assign(&mut self, #param1: &#b_ty) {
          #(#body);*
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.mult_inv_body();
    quote!(
      impl #impl_gen ::core::ops::Sub<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn sub(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_inv_body();
    quote!(
      impl #impl_gen ::core::ops::Sub<#self_ty #ty_gen> for &#self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn sub(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_inv_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Sub<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn sub(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_inv_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Sub<&#b_ty> for &#self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn sub(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_inv_body();
    quote!(
      impl #impl_gen ::core::ops::SubAssign<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        #[inline]
        fn sub_assign(&mut self, #param1: #self_ty #ty_gen) {
          #(#body);*
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_inv_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::SubAssign<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        #[inline]
        fn sub_assign(&mut self, #param1: &#b_ty) {
          #(#body);*
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.inv_body();
    quote!(
      impl #impl_gen #neg_path for #self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;
        
        #[inline]
        fn neg(self) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_inv_body();
    quote!(
      impl #impl_gen #neg_path for &#self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;
        
        #[inline]
        fn neg(self) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;

  impl_store.push_impl({
    let (
      get_one_body,
      is_one_body,
      set_one_body
    ) = aux_body_impl.get_is_set_unit_body();
    quote!(
      impl #impl_gen #zero_path for #self_ty #ty_gen #where_clause {
        #[inline]
        fn zero() -> Self {
          #get_one_body
        }

        #[inline]
        fn is_zero(&self) -> bool {
          #(#is_one_body)&&*
        }

        #[inline]
        fn set_zero(&mut self) {
          #(#set_one_body);*
        }
      }
    )
  })?;

  impl_store.into_result()
}
