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
  let target = match &wrap_deref_attr {
    Some(attr) => match &attr.meta {
      syn::Meta::Path(_) => s_fields.iter().next(),
      syn::Meta::NameValue(syn::MetaNameValue {value, ..}) => { //TODO: key-value macros are not supported ATM :(
        let ident = syn::parse2::<Ident>(value.to_token_stream())?;
        let found_field = s_fields.iter().find(|f| *f.ident.as_ref().unwrap() == ident);
        if found_field.is_none() {
          return_err!(ident.span(), "{}::{} not found", &input.ident, &ident)
        }
        found_field
      },
      syn::Meta::List(syn::MetaList {tokens, ..}) => { //TODO: Remove this once key-value macros are supported
        let l = Punctuated::<Ident, Token![,]>::parse_terminated.parse2(tokens.clone())?;
        let mut l = l.into_iter();
        if let Some(ident) = l.next() {
          if let Some(ident) = l.next() {
            return_err!(ident.span(), "Too many elements to wrap")
          }
          let found_field = s_fields.iter().find(|f| *f.ident.as_ref().unwrap() == ident);
          if found_field.is_none() {
            return_err!(ident.span(), "{}::{} not found", &input.ident, &ident)
          }
          found_field
        } else {
          s_fields.iter().next()
        }
      },
      // syn::Meta::List(l) => { //TODO: Use this once key-value macros are supported
      //   return_err!(l.span(), "Can not be a list. Use {WRAP_DEREF_STR} = .. instead")
      // },
    },
    None if s_fields.len() == 1 => s_fields.iter().next(),
    // None => return_err!(s_fields.span(), "Only one element can be wrapped. Consider adding #[{WRAP_DEREF_STR} {{ = .. }}?] and specifying the wrapped element")
    None => return_err!(s_fields.span(), "Only one element can be wrapped. Consider adding #[{WRAP_DEREF_STR} ( .. )?] and specifying the wrapped element")
  }.unwrap();
  let is_pub = match target.vis {
    syn::Visibility::Public(_) => true,
    _ => false,
  };

  let (impl_gen, ty_gen, where_clause) = input.generics.split_for_impl();
  let self_ty = parse2::<Type>(input.ident.to_token_stream())?;
  let target_ty = &target.ty;
  let target_path = target.ident.as_ref().unwrap();

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


// pub fn wrapper_deref_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
//   const WRAP_DEREF_STR: &str = "wrap_deref";
//   let wrap_deref_attrs = match &input.data {
//     syn::Data::Struct(syn::DataStruct {fields, ..}) => match fields {
//       syn::Fields::Named(syn::FieldsNamed {named, ..}) if named.len() > 0 => {
//         let fields = named.iter().map(|f| f.ident.as_ref().unwrap()).cloned().collect::<Vec<_>>();
//         let f0 = fields.first().unwrap().clone();
//         let wrap_deref_attr_path = AttrMetaBuilder::Path(parse_str(WRAP_DEREF_STR)?, Rc::new(move |lookup: &mut DynLookup| {
//           lookup.insert(WRAP_DEREF_STR.to_string(), f0.clone());
//           Ok(())
//         }));
//         let wrap_deref_attr_assign = AttrMetaBuilder::NameValue(parse_str(WRAP_DEREF_STR)?, Rc::new(move |lookup: &mut DynLookup, expr| {
//           let f = syn::parse2::<syn::Ident>(expr.to_token_stream())?;
//           if fields.iter().find(|g| **g == f).is_none() {
//             return Err(syn::Error::new(f.span(), format!("Struct has no element with name {f}")))
//           }
//           lookup.insert(WRAP_DEREF_STR.to_string(), f);
//           Ok(())
//         }));
//         vec![wrap_deref_attr_path, wrap_deref_attr_assign]
//       },
//       syn::Fields::Unnamed(syn::FieldsUnnamed {unnamed, ..}) if unnamed.len() > 0 => {
//         let data_len = unnamed.len();
//         let wrap_deref_attr_path = AttrMetaBuilder::Path(parse_str(WRAP_DEREF_STR)?, Rc::new(|lookup: &mut DynLookup| {
//           lookup.insert(WRAP_DEREF_STR.to_string(), syn::Index::from(0));
//           Ok(())
//         }));
//         let wrap_deref_attr_assign = AttrMetaBuilder::NameValue(parse_str(WRAP_DEREF_STR)?, Rc::new(move |lookup: &mut DynLookup, expr| {
//           let idx = syn::parse2::<syn::Index>(expr.to_token_stream())?;
//           if idx.index as usize >= data_len {
//             return Err(syn::Error::new(idx.span(), format!("Struct has only {data_len} elements")))
//           }
//           lookup.insert(WRAP_DEREF_STR.to_string(), idx);
//           Ok(())
//         }));
//         vec![wrap_deref_attr_path, wrap_deref_attr_assign]
//       },
//       _ => return Err(syn::Error::new(input.span(), "Wrappers need at least one element")),
//     },
//     _ => return Err(syn::Error::new(input.span(), "Only structs are supported"))
//   };

//   let deref = TraitDerivationBuilder::from_str("::core::ops::Deref")?
//     .push_attributes(wrap_deref_attrs.iter().cloned())
//     .push_function(parse_str("fn deref(&self) -> Self::Target {}")?);
//   // deref.
//   let deref_mut = TraitDerivationBuilder::from_str("::core::ops::DerefMut")?
//     .push_attributes(wrap_deref_attrs.into_iter());
//   let (t, _) = TraitDerivationBuilder::derive_impls(
//     vec![&deref, &deref_mut],
//     input
//   )?;
//   let mut t = t.into_iter();
//   let [deref, deref_mut] = [(); 2].map(|()| t.next().unwrap());
  
//   todo!()
// }



