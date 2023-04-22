use proc_macro::TokenStream;
use syn::{DeriveInput, GenericParam, parse_quote, TypeParam, TypeParamBound, TraitBound};
use quote::{quote, ToTokens};

use crate::util::{attr_repr::AttrRepr, data_quote::{DataQuote, DataQuotePaths}, self};




#[inline]
pub(crate) fn group_wrapper_impl(input: &DeriveInput) -> TokenStream {
  let attrs = AttrRepr::new_with(input.attrs.iter());
  // let attrs = AttrRepr::default();
  let zero_trait = attrs.zero_path.get();
  let gen_group = attrs.gen_group_path.get();

  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

  let self_path = parse_quote!(self);
  let rhs_path = parse_quote!(rhs);
  let t_idents = DataQuotePaths::t_idents_from(gen_group, input.generics.params.iter());
  // panic!("{:?}", t_idents);

  let path_choice = DataQuotePaths {
    t_idents,
    t_fn_path: parse_quote!(add),
    // default_fn_path: parse_quote!(xxxxxxx),
    default_fn_path: parse_quote!(add),
  };
  let add_expr = DataQuote::OwnOwn.quote(&self_path, &rhs_path, &path_choice, ident, &input.data);

  let path_choice = DataQuotePaths {
    t_fn_path: parse_quote!(ref_add),
    ..path_choice
  };
  let ref_add_expr = DataQuote::RefOwn.quote(&self_path, &rhs_path, &path_choice, ident, &input.data);

  let path_choice = DataQuotePaths {
    t_fn_path: parse_quote!(add_assign),
    default_fn_path: parse_quote!(add_assign),
    ..path_choice
  };
  let add_assign_expr = DataQuote::MutOwn.quote(&self_path, &rhs_path, &path_choice, ident, &input.data);

  // panic!("{}", add_assign_expr);

  let own_rhs_quote = quote! {
    // Addition +++++++++++++++++++++++++++++++++++++++++++
    impl #impl_gen std::ops::Add<#ident #type_gen> for #ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn add(self, rhs: #ident #type_gen) -> #ident #type_gen {
        // #ident(self.0.add(rhs.0))
        #add_expr
      }
    }

    impl #impl_gen std::ops::Add<#ident #type_gen> for &#ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn add(self, rhs: #ident #type_gen) -> #ident #type_gen {
        // #ident(self.0.ref_add(rhs.0))
        #ref_add_expr
      }
    }

    impl #impl_gen std::ops::AddAssign<#ident #type_gen> for #ident #type_gen #where_clause {
      fn add_assign(&mut self, rhs: #ident #type_gen) {
        // self.0.add_assign(rhs.0)
        #add_assign_expr
      }
    }

  };

  own_rhs_quote.into()
}

#[inline]
pub(crate) fn _group_wrapper_impl(input: &DeriveInput) -> TokenStream {
  let attrs = AttrRepr::new_with(input.attrs.iter());
  // let attrs = AttrRepr::default();
  let zero_trait = attrs.zero_path.get();

  // Group
  let ident = &input.ident;
  // (<T: GenGroup>, <T>, None)
  let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();


  let own_rhs_quote = quote! {
    // Addition +++++++++++++++++++++++++++++++++++++++++++
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

    impl #impl_gen std::ops::AddAssign<#ident #type_gen> for #ident #type_gen #where_clause {
      fn add_assign(&mut self, rhs: #ident #type_gen) {
        self.0.add_assign(rhs.0)
      }
    }

    // Subtraktion ++++++++++++++++++++++++++++++++++++++++
    impl #impl_gen std::ops::Sub<#ident #type_gen> for #ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn sub(self, rhs: #ident #type_gen) -> #ident #type_gen {
        #ident(self.0.sub(rhs.0))
      }
    }

    impl #impl_gen std::ops::Sub<#ident #type_gen> for &#ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn sub(self, rhs: #ident #type_gen) -> #ident #type_gen {
        #ident(self.0.ref_sub(rhs.0))
      }
    }

    impl #impl_gen std::ops::SubAssign<#ident #type_gen> for #ident #type_gen #where_clause {
      fn sub_assign(&mut self, rhs: #ident #type_gen) {
        self.0.sub_assign(rhs.0)
      }
    }
  }.into();

  let unary_quote = quote! {
    impl #impl_gen std::ops::Neg for #ident #type_gen #where_clause {
      type Output = #ident #type_gen;

      fn neg(self) -> #ident #type_gen {
        #ident(self.0.neg())
      }
    }

    impl #impl_gen std::ops::Neg for &#ident #type_gen #where_clause {
      type Output = #ident #type_gen;

      fn neg(self) -> #ident #type_gen {
        #ident(self.0.ref_neg())
      }
    }

    // impl #impl_gen tensor_derive::reexport::Zero for #ident #type_gen #where_clause {
    impl #impl_gen #zero_trait for #ident #type_gen #where_clause {
      fn zero() -> Self {
        todo!()
      }

      fn set_zero(&mut self) {
        self.0.set_zero()
      }
  
      fn is_zero(&self) -> bool {
        self.0.is_zero()
      }
    }
  }.into();

  let b_gen: TypeParam = parse_quote!(B: std::borrow::Borrow<#ident #type_gen>);
  let b_ident: syn::Ident = b_gen.ident.clone();
  let mut generics_ref = input.generics.clone();
  generics_ref.params.push(GenericParam::Type(b_gen));
  let (impl_gen, _, where_clause) = generics_ref.split_for_impl();

  let bor_rhs_quote = quote! {
    // Addition +++++++++++++++++++++++++++++++++++++++++++
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

    impl #impl_gen std::ops::AddAssign<&#b_ident> for #ident #type_gen #where_clause {
      fn add_assign(&mut self, rhs: &#b_ident) {
        self.0.add_assign_ref(&rhs.borrow().0)
      }
    }

    // Subtraktion ++++++++++++++++++++++++++++++++++++++++
    impl #impl_gen std::ops::Sub<&#b_ident> for #ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn sub(self, rhs: &#b_ident) -> #ident #type_gen {
        #ident(self.0.sub_ref(&rhs.borrow().0))
      }
    }

    impl #impl_gen std::ops::Sub<&#b_ident> for &#ident #type_gen #where_clause {
      type Output = #ident #type_gen;
    
      fn sub(self, rhs: &#b_ident) -> #ident #type_gen {
        #ident(self.0.ref_sub_ref(&rhs.borrow().0))
      }
    }

    impl #impl_gen std::ops::SubAssign<&#b_ident> for #ident #type_gen #where_clause {
      fn sub_assign(&mut self, rhs: &#b_ident) {
        self.0.sub_assign_ref(&rhs.borrow().0)
      }
    }
  }.into();

  <_ as std::iter::FromIterator<TokenStream>>::from_iter([
    own_rhs_quote,
    bor_rhs_quote,
    unary_quote
  ])
}


