use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse2, punctuated::Punctuated, Token, parse::Parser, spanned::Spanned, visit::Visit, parse_quote, Type, parse_str};

use crate::util::{return_err, struct_vis::{StructLookup, StructLookupPaths, LookupKind, LookupAccess}, get_unit_types, tests::eq_tokens, get_zero_path, get_one_path, get_derive_generic, get_inv_path};

type TokenStream1 = proc_macro::TokenStream;


fn degenerify_type_path(path: &syn::TypePath) -> Option<syn::TypePath> {
  let no_generics = path.path.segments.last()?.arguments.is_empty();
  if no_generics {
    return None;
  }
  let mut res = path.clone();
  let last = res.path.segments.last_mut()?;
  last.arguments = syn::PathArguments::default();
  Some(res)
}

#[derive(Clone)]
struct GroupAuxImpl<'a> {
  input: &'a DeriveInput,
  self_ty: Option<Type>,
  borrow_ty: Option<syn::TypeParam>,
  pub exprs: Vec<syn::Expr>,
  // pub group_impls: Vec<syn::ItemImpl>,
  // syn_values: Option<GroupAuxSyn<'a>>,
  // borrow_syn_values: Option<GroupAuxSyn<'a>>,
}

#[derive(Clone)]
struct GroupAuxBody<'a> {
  lookup: StructLookupPaths<'a>,
  pub unit_types: Vec<syn::Type>,
  pub input_ident: &'a syn::Ident,
  pub exprs: &'a Vec<syn::Expr>,
  pub derive_gen: Option<syn::Path>,
}

#[derive(Clone, Default)]
struct GroupImplStore{
  pub inner: Vec<syn::ItemImpl>,
}

#[derive(Clone)]
struct GroupAuxSyn<'a> {
  self_ty: &'a Type,
  impl_gen: syn::ImplGenerics<'a>,
  ty_gen: syn::TypeGenerics<'a>,
  where_clause: Option<&'a syn::WhereClause>,
  param1: &'a syn::Expr,
}
#[derive(Clone)]
struct GroupAuxBorrowSyn<'a> {
  b_ty: &'a syn::Ident,
  generics: syn::Generics,
  self_ty: &'a Type,
  ty_gen: syn::TypeGenerics<'a>,
  param1: &'a syn::Expr,
  // syn: GroupAuxSyn<'a>,
}

pub fn group_impl(input: &DeriveInput) -> syn::Result<TokenStream1> {
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

  let one_path = get_one_path(&input.attrs);
  let inv_path = get_inv_path(&input.attrs);


  impl_store.push_impl({
    let body = aux_body_impl.mult_body();
    quote!(
      impl #impl_gen ::core::ops::Mul<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn mul(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_body();
    quote!(
      impl #impl_gen ::core::ops::Mul<#self_ty #ty_gen> for &#self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn mul(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Mul<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn mul(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Mul<&#b_ty> for &#self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn mul(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_body();
    quote!(
      impl #impl_gen ::core::ops::MulAssign<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        #[inline]
        fn mul_assign(&mut self, #param1: #self_ty #ty_gen) {
          #(#body);*
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::MulAssign<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        #[inline]
        fn mul_assign(&mut self, #param1: &#b_ty) {
          #(#body);*
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.mult_inv_body();
    quote!(
      impl #impl_gen ::core::ops::Div<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn div(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_inv_body();
    quote!(
      impl #impl_gen ::core::ops::Div<#self_ty #ty_gen> for &#self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn div(self, #param1: #self_ty #ty_gen) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_inv_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Div<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn div(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_mult_inv_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::Div<&#b_ty> for &#self_ty #ty_gen #b_where_clause {
        type Output = #self_ty #ty_gen;

        #[inline]
        fn div(self, #param1: &#b_ty) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_inv_body();
    quote!(
      impl #impl_gen ::core::ops::DivAssign<#self_ty #ty_gen> for #self_ty #ty_gen #where_clause {
        #[inline]
        fn div_assign(&mut self, #param1: #self_ty #ty_gen) {
          #(#body);*
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.mult_assign_inv_ref_body();
    quote!(
      impl #b_impl_gen ::core::ops::DivAssign<&#b_ty> for #self_ty #ty_gen #b_where_clause {
        #[inline]
        fn div_assign(&mut self, #param1: &#b_ty) {
          #(#body);*
        }
      }
    )
  })?;

  impl_store.push_impl({
    let body = aux_body_impl.inv_body();
    quote!(
      impl #impl_gen #inv_path for #self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;
        
        #[inline]
        fn inv(self) -> #self_ty #ty_gen {
          #body
        }
      }
    )
  })?;
  impl_store.push_impl({
    let body = aux_body_impl.ref_inv_body();
    quote!(
      impl #impl_gen #inv_path for &#self_ty #ty_gen #where_clause {
        type Output = #self_ty #ty_gen;
        
        #[inline]
        fn inv(self) -> #self_ty #ty_gen {
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
      impl #impl_gen #one_path for #self_ty #ty_gen #where_clause {
        #[inline]
        fn one() -> Self {
          #get_one_body
        }

        #[inline]
        fn is_one(&self) -> bool {
          #(#is_one_body)&&*
        }

        #[inline]
        fn set_one(&mut self) {
          #(#set_one_body);*
        }
      }
    )
  })?;

  impl_store.into_result()
}

impl<'a> GroupAuxImpl<'a> {
  pub fn new(input: &'a DeriveInput) -> syn::Result<Self> {
    let exprs = ["self", "rhs"].into_iter()
      .map(|s| syn::parse_str(s).unwrap())
      .collect::<Vec<syn::Expr>>();
    let mut res = Self {
      input,
      exprs,
      self_ty: None,
      borrow_ty: None,
    };
    res.self_ty()?;
    res.borrow_ty()?;
    Ok(res)
  }
}
impl GroupAuxImpl<'_> {
  pub fn self_ty(&mut self) -> syn::Result<&Type> {
    if self.self_ty.is_none() {
      self.self_ty = Some(parse2(self.input.ident.to_token_stream())?);
    }
    Ok(self.get_self_ty())
  }
  pub fn get_self_ty(&self) -> &Type {
    self.self_ty.as_ref().unwrap()
  }

  pub fn borrow_ty(&mut self) -> syn::Result<&syn::TypeParam> {
    if self.borrow_ty.is_none() {
      let self_ty: &Type = self.get_self_ty();
      let (_, ty_gen, _) = self.input.generics.split_for_impl();
      self.borrow_ty = Some(parse2(quote!(B: ::core::borrow::Borrow<#self_ty #ty_gen>))?);
    }
    Ok(self.get_borrow_ty())
  }
  pub fn get_borrow_ty(&self) -> &syn::TypeParam {
    self.borrow_ty.as_ref().unwrap()
  }

  pub fn get_syn_values(&self) -> GroupAuxSyn {
    let param1 = &self.exprs[1];
    let (impl_gen, ty_gen, where_clause) = self.input.generics.split_for_impl();
    let self_ty = self.get_self_ty();

    GroupAuxSyn {
      self_ty,
      impl_gen,
      ty_gen,
      where_clause,
      param1
    }
  }

  pub fn get_syn_borrow_values(&self) -> GroupAuxBorrowSyn {
    let GroupAuxSyn {
      self_ty,
      ty_gen,
      param1,
      ..
    } = self.get_syn_values();
    let (generics_ref, b_ty) = {
      let b_gen = self.get_borrow_ty();
      let b_ty = &b_gen.ident;
      let mut generics_ref = self.input.generics.clone();
      generics_ref.params.push(syn::GenericParam::Type(b_gen.clone()));
      (generics_ref, b_ty)
    };
    GroupAuxBorrowSyn {
      b_ty,
      generics: generics_ref,
      self_ty,
      ty_gen,
      param1,
    }
  }
}

impl<'a> GroupAuxBorrowSyn<'a> {
  pub fn get_full<'b>(&'a self) -> (&'b syn::Ident, GroupAuxSyn<'b>) where 'a: 'b {
    let GroupAuxBorrowSyn {
      b_ty,
      generics,
      self_ty,
      ty_gen,
      param1,
    } = self;
    let (
      impl_gen,
      _,
      where_clause
    ) = generics.split_for_impl();
    (
      b_ty,
      GroupAuxSyn {
        self_ty,
        impl_gen,
        ty_gen: ty_gen.clone(),
        where_clause,
        param1,
      }
    )
  }
  pub fn get<'b>(&'a self) -> (&'b syn::Ident, syn::ImplGenerics<'b>, Option<&'b syn::WhereClause>) where 'a: 'b {
    let (
      b_ty,
      GroupAuxSyn {
        impl_gen,
        where_clause,
        ..
      }
    ) = self.get_full();
    (b_ty, impl_gen, where_clause)
  }
}

impl GroupImplStore {
  pub fn push_impl(&mut self, impl_tokens: TokenStream) -> syn::Result<()> {
    self.inner.push(parse2(impl_tokens)?);
    Ok(())
  }
  pub fn into_result(self) -> syn::Result<TokenStream1> {
    Ok(TokenStream1::from_iter(
      self.inner.into_iter().map(|i|
        TokenStream1::from(i.into_token_stream())
    )))
  }
}

impl<'a> GroupAuxBody<'a> {
  pub fn new(aux_impl: &'a GroupAuxImpl<'a>) -> syn::Result<Self> {
    let mut lookup = StructLookup::new();
    lookup.visit_derive_input(&aux_impl.input);
    let lookup: StructLookupPaths = match lookup.try_into() {
      Ok(x) => x,
      Err(_) => return_err!(aux_impl.input.span(), ""),
    };

    let unit_types = get_unit_types(&aux_impl.input.attrs).unwrap_or_default();
    let derive_gen = get_derive_generic(&aux_impl.input.attrs);
  
    let input_ident = &aux_impl.input.ident;
    let exprs = &aux_impl.exprs;

    Ok(Self {
      lookup,
      unit_types,
      input_ident,
      exprs,
      derive_gen
    })
  }
}
impl GroupAuxBody<'_> {
  fn f_unit_default(ty: Type) -> syn::Expr {
    parse_quote!(#ty)
  }
  fn f_unit_none(_: Type) -> Option<syn::Expr> {
    None
  }

  pub fn body(
    &self,
    f: impl Fn(Vec<syn::Expr>, syn::Type) -> syn::Expr,
    accesses: impl IntoIterator<Item = LookupAccess>,
    f_unit: impl Fn(syn::Type) -> syn::Expr,
   ) -> syn::Expr {
    let full_f = |es: Vec<_>, ty: syn::Type| {
      let ty_in_unit_types = self.unit_types
        .iter()
        .find(|uty| eq_tokens(uty, &ty))
        .is_some();
      if ty_in_unit_types {
        let opt_ty = match &ty {
          Type::Path(p) => degenerify_type_path(p).map(Type::Path),
          Type::Tuple(_) => todo!(),
          Type::Verbatim(_) => todo!(),
          _ => todo!(),
        };
        let ty = opt_ty.unwrap_or(ty);
        // return parse_quote!(#ty);
        return f_unit(ty);
        // Todo dependency injection for handling this case
      }
      f(es, ty)
    };
    self.lookup
      .clone()
      .with_many_and_collect(self.input_ident, self.exprs, accesses, full_f)
  }

  pub fn list_elems(
    &self,
    f: impl Fn(Vec<syn::Expr>, syn::Type) -> Option<syn::Expr>,
    accesses: impl IntoIterator<Item = LookupAccess>,
    f_unit: impl Fn(syn::Type) -> Option<syn::Expr>,
    default: Option<syn::Expr>
  ) -> Vec<syn::Expr> {
    let full_f = |es: Vec<_>, ty: syn::Type| {
      let ty_in_unit_types = self.unit_types
        .iter()
        .find(|uty| eq_tokens(uty, &ty))
        .is_some();
      if ty_in_unit_types {
        let opt_ty = match &ty {
          Type::Path(p) => degenerify_type_path(p).map(Type::Path),
          Type::Tuple(_) => todo!(),
          Type::Verbatim(_) => todo!(),
          _ => todo!(),
        };
        let ty = opt_ty.unwrap_or(ty);
        // return parse_quote!(#ty);
        return f_unit(ty);
        // Todo dependency injection for handling this case
      }
      f(es, ty)
    };
    let dummy_expr = parse_str::<syn::Expr>("DUMMY_EXPR").unwrap();
    let full_f = |es, ty| {
      full_f(es, ty).unwrap_or(dummy_expr.clone())
    };
    let mut res = self.lookup
      .clone()
      .with_many(self.exprs, accesses, full_f)
      .into_iter()
      .filter(|e| !eq_tokens(e, &dummy_expr))
      .collect::<Vec<_>>();
    if res.is_empty() {
      if let Some(val) = default {
        res.push(val)
      }
    }
    res
  }


  pub fn mult_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult(#e0, #e1)),
        None => parse_quote!(#e0.mult(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Own],
      Self::f_unit_default
    )
  }
  pub fn ref_mult_body(&self) -> syn::Expr {
    // fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
    //   let e0 = &es[0];
    //   let e1 = &es[1];
    //   match derive_gen {
    //     Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::ref_mult(#e0, #e1)),
    //     None => parse_quote!((#e0).ref_mult(#e1)),
    //   }
    // }
    // self.body(
    //   |es, _| f(es, &self.derive_gen),
    //   [LookupAccess::Ref, LookupAccess::Own],
    //   Self::f_unit_default
    // )
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::ref_mult(&#e0, #e1)),
        None => parse_quote!(#e0.ref_mult(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Own],
      Self::f_unit_default
    )
  }
  pub fn mult_ref_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_ref(#e0, #e1)),
        None => parse_quote!(#e0.mult_ref(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Borrow],
      Self::f_unit_default
    )
  }
  pub fn ref_mult_ref_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::ref_mult_ref(&#e0, #e1)),
        None => parse_quote!(#e0.ref_mult_ref(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Borrow],
      Self::f_unit_default
    )
  }

  pub fn mult_assign_body(&self) -> Vec<syn::Expr> {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
      let e0 = &es[0];
      let e1 = &es[1];
      Some(match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_assign(&mut #e0, #e1)),
        None => parse_quote!(#e0.mult_assign(#e1)),
      })
    }
    self.list_elems(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Own],
      Self::f_unit_none,
      parse_str("()").ok()
    )
  }
  pub fn mult_assign_ref_body(&self) -> Vec<syn::Expr> {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
      let e0 = &es[0];
      let e1 = &es[1];
      Some(match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_assign_ref(&mut #e0, #e1)),
        None => parse_quote!(#e0.mult_assign_ref(#e1)),
      })
    }
    self.list_elems(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Borrow],
      Self::f_unit_none,
      parse_str("()").ok()
    )
  }

  pub fn mult_inv_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_inv(#e0, #e1)),
        None => parse_quote!(#e0.mult_inv(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Own],
      Self::f_unit_default
    )
  }
  pub fn ref_mult_inv_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::ref_mult_inv(&#e0, #e1)),
        None => parse_quote!(#e0.ref_mult_inv(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Own],
      Self::f_unit_default
    )
  }
  pub fn mult_inv_ref_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_inv_ref(#e0, #e1)),
        None => parse_quote!(#e0.mult_inv_ref(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Borrow],
      Self::f_unit_default
    )
  }
  pub fn ref_mult_inv_ref_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e0 = &es[0];
      let e1 = &es[1];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::ref_mult_inv_ref(&#e0, #e1)),
        None => parse_quote!(#e0.ref_mult_inv_ref(#e1)),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Borrow],
      Self::f_unit_default
    )
  }

  pub fn mult_assign_inv_body(&self) -> Vec<syn::Expr> {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
      let e0 = &es[0];
      let e1 = &es[1];
      Some(match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_assign_inv(&mut #e0, #e1)),
        None => parse_quote!(#e0.mult_assign_inv(#e1)),
      })
    }
    self.list_elems(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Own],
      Self::f_unit_none,
      parse_str("()").ok()
    )
  }
  pub fn mult_assign_inv_ref_body(&self) -> Vec<syn::Expr> {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
      let e0 = &es[0];
      let e1 = &es[1];
      Some(match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::mult_assign_inv_ref(&mut #e0, #e1)),
        None => parse_quote!(#e0.mult_assign_inv_ref(#e1)),
      })
    }
    self.list_elems(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own, LookupAccess::Borrow],
      Self::f_unit_none,
      parse_str("()").ok()
    )
  }

  pub fn inv_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e = &es[0];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::inv(#e)),
        None => parse_quote!(#e.inv()),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own],
      Self::f_unit_default,
    )
  }
  pub fn ref_inv_body(&self) -> syn::Expr {
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> syn::Expr {
      let e = &es[0];
      match derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::ref_inv(&#e)),
        None => parse_quote!(#e.ref_inv()),
      }
    }
    self.body(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own],
      Self::f_unit_default,
    )
  }

  pub fn get_unit_body(&self) -> syn::Expr {
    fn f(ty: Type, derive_gen: &Option<syn::Path>) -> syn::Expr {
      match derive_gen {
        Some(gen_t) => parse_quote!(<#ty as GenGroup::<#gen_t>>::unit()),
        None => parse_quote!(#ty::unit()),
      }
    }
    self.body(
      |_, ty| f(ty, &self.derive_gen),
      None,
      Self::f_unit_default,
    )
  }
  pub fn is_unit_body(&self) -> Vec<syn::Expr> {
    // fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
    //   let e = &es[0];
    //   Some(match &derive_gen {
    //     Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::is_unit(#e)),
    //     None => parse_quote!((#e).is_unit()),
    //   })
    // }
    // self.list_elems(
    //   |es, _| f(es, &self.derive_gen),
    //   LookupAccess::Ref,
    //   Self::f_unit_none,
    //   parse_str("true").ok()
    // )
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
      let e = &es[0];
      Some(match &derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::is_unit(&#e)),
        None => parse_quote!(#e.is_unit()),
      })
    }
    self.list_elems(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own],
      Self::f_unit_none,
      parse_str("true").ok()
    )
  }
  pub fn set_unit_body(&self) -> Vec<syn::Expr> {
    // fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
    //   let e = &es[0];
    //   Some(match &derive_gen {
    //     Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::set_unit(#e)),
    //     None => parse_quote!((#e).set_unit()),
    //   })
    // }
    // self.list_elems(
    //   |es, _| f(es, &self.derive_gen),
    //   LookupAccess::MutRef,
    //   Self::f_unit_none,
    //   parse_str("()").ok()
    // )
    fn f(es: Vec<syn::Expr>, derive_gen: &Option<syn::Path>) -> Option<syn::Expr> {
      let e = &es[0];
      Some(match &derive_gen {
        Some(gen_t) => parse_quote!(GenGroup::<#gen_t>::set_unit(&mut #e)),
        None => parse_quote!(#e.set_unit()),
      })
    }
    self.list_elems(
      |es, _| f(es, &self.derive_gen),
      [LookupAccess::Own],
      Self::f_unit_none,
      parse_str("()").ok()
    )
  }
  pub fn get_is_set_unit_body(&self) -> (syn::Expr, Vec<syn::Expr>, Vec<syn::Expr>) {
    (self.get_unit_body(), self.is_unit_body(), self.set_unit_body())
  }
}



use tensor_traits::GenGroup;
pub struct G(pub i32);

impl GenGroup for G {
  fn mult(self, _: Self) -> Self { unimplemented!() }
  fn ref_mult(&self, _: Self) -> Self { unimplemented!() }
  fn mult_ref(self, _: &Self) -> Self { unimplemented!() }
  fn ref_mult_ref(&self, _: &Self) -> Self { unimplemented!() }
  fn mult_assign(&mut self, _: Self) { unimplemented!() }
  fn mult_assign_ref(&mut self, _: &Self) { unimplemented!() }

  fn mult_inv(self, _: Self) -> Self { unimplemented!() }
  fn ref_mult_inv(&self, _: Self) -> Self { unimplemented!() }
  fn mult_inv_ref(self, _: &Self) -> Self { unimplemented!() }
  fn ref_mult_inv_ref(&self, _: &Self) -> Self { unimplemented!() }
  fn mult_assign_inv(&mut self, _: Self) { unimplemented!() }
  fn mult_assign_inv_ref(&mut self, _: &Self) { unimplemented!() }

  fn inv(self) -> Self { unimplemented!() }
  fn ref_inv(&self) -> Self { unimplemented!() }

  fn unit() -> Self { unimplemented!() }
  fn set_unit(&mut self) { unimplemented!() }
  fn is_unit(&self) -> bool { unimplemented!() }
}



