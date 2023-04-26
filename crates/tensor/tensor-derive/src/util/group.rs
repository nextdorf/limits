use syn::parse_quote;

use crate::util::data_quote::{DataQuote, DataQuotePaths};

use super::attr_repr::AttrRepr;
type TokenStream = quote::__private::TokenStream;

#[derive(Clone)]
pub struct TraitImplPaths {
  pub fn_path: syn::Ident,
  pub trait_path: syn::Path,
}

#[derive(Clone)]
pub struct TraitImplExtPaths {
  pub t_impl_path: syn::Path,
  pub trait_impl_paths: TraitImplPaths,
}

pub struct GroupDataQuotePaths {
  pub inner: DataQuotePaths,

  pub mult_expr: TraitImplExtPaths,
  pub ref_mult_expr: TraitImplExtPaths,
  pub mult_bor_expr: TraitImplExtPaths,
  pub ref_mult_bor_expr: TraitImplExtPaths,
  pub mult_assign_expr: TraitImplExtPaths,
  pub mult_assign_bor_expr: TraitImplExtPaths,

  pub mult_inv_expr: TraitImplExtPaths,
  pub ref_mult_inv_expr: TraitImplExtPaths,
  pub mult_inv_bor_expr: TraitImplExtPaths,
  pub ref_mult_inv_bor_expr: TraitImplExtPaths,
  pub mult_assign_inv_expr: TraitImplExtPaths,
  pub mult_assign_inv_bor_expr: TraitImplExtPaths,

  pub unit_expr: TraitImplExtPaths,
  pub set_unit_expr: TraitImplExtPaths,
  pub is_unit_expr: TraitImplExtPaths,

  pub inv_expr: TraitImplExtPaths,
  pub ref_inv_expr: TraitImplExtPaths,
}

pub enum GenGroupKind {
  Abel,
  Mult,
}

pub struct QuoteParams<'a> {
  pub lhs_path: &'a syn::Path,
  pub rhs_path: &'a syn::Path,
  pub ident: &'a syn::Ident,
  pub data: &'a syn::Data
}

pub struct QuoteRes {
  pub expr: TokenStream,
  pub trait_impl_paths: TraitImplPaths,
}


impl TraitImplPaths {
  pub const fn new(fn_path: syn::Ident, trait_path: syn::Path) -> Self {
    Self {
      fn_path,
      trait_path,
    }
  }
  pub const fn ext(self, t_impl_path: syn::Path) -> TraitImplExtPaths {
    TraitImplExtPaths {
      t_impl_path,
      trait_impl_paths: self,
    }
  }
  pub fn unpack(self) -> (syn::Ident, syn::Path) {
    let Self { fn_path, trait_path } = self;
    (fn_path, trait_path)
  }
}


impl TraitImplExtPaths {
  pub fn unpack(self) -> (syn::Path, syn::Ident, syn::Path) {
    let Self { t_impl_path, trait_impl_paths } = self;
    let (fn_path, trait_path) = trait_impl_paths.unpack();
    (t_impl_path, fn_path, trait_path)
  }
}


impl GroupDataQuotePaths {
  pub fn new(t_idents: Vec<syn::Ident>, kind: GenGroupKind, attrs: &AttrRepr) -> Self {
    let new_fn = match kind {
      GenGroupKind::Abel => Self::new_abel,
      GenGroupKind::Mult => Self::new_mult,
    };
    new_fn(t_idents, attrs)
  }

  pub fn new_abel(t_idents: Vec<syn::Ident>, attrs: &AttrRepr) -> Self {
    let zero_path = attrs.zero_path.get();

    Self::new_base(
      t_idents,
      TraitImplPaths::new(parse_quote!(add), parse_quote!(std::ops::Add)),
      TraitImplPaths::new(parse_quote!(add), parse_quote!(std::ops::Add)),
      TraitImplPaths::new(parse_quote!(add), parse_quote!(std::ops::Add)),
      TraitImplPaths::new(parse_quote!(add), parse_quote!(std::ops::Add)),
      TraitImplPaths::new(parse_quote!(add_assign), parse_quote!(std::ops::AddAssign)),
      TraitImplPaths::new(parse_quote!(add_assign), parse_quote!(std::ops::AddAssign)),

      TraitImplPaths::new(parse_quote!(sub), parse_quote!(std::ops::Sub)),
      TraitImplPaths::new(parse_quote!(sub), parse_quote!(std::ops::Sub)),
      TraitImplPaths::new(parse_quote!(sub), parse_quote!(std::ops::Sub)),
      TraitImplPaths::new(parse_quote!(sub), parse_quote!(std::ops::Sub)),
      TraitImplPaths::new(parse_quote!(sub_assign), parse_quote!(std::ops::SubAssign)),
      TraitImplPaths::new(parse_quote!(sub_assign), parse_quote!(std::ops::SubAssign)),

      TraitImplPaths::new(parse_quote!(zero), zero_path.clone()),
      TraitImplPaths::new(parse_quote!(set_zero), zero_path.clone()),
      TraitImplPaths::new(parse_quote!(is_zero), zero_path.clone()),

      TraitImplPaths::new(parse_quote!(neg), parse_quote!(std::ops::Neg)),
      TraitImplPaths::new(parse_quote!(neg), parse_quote!(std::ops::Neg)),
    )
  }

  pub fn new_mult(t_idents: Vec<syn::Ident>, attrs: &AttrRepr) -> Self {
    let one_path = attrs.one_path.get();
    let inv_path = attrs.inv_path.get();

    Self::new_base(
      t_idents,
      TraitImplPaths::new(parse_quote!(mul), parse_quote!(std::ops::Mul)),
      TraitImplPaths::new(parse_quote!(mul), parse_quote!(std::ops::Mul)),
      TraitImplPaths::new(parse_quote!(mul), parse_quote!(std::ops::Mul)),
      TraitImplPaths::new(parse_quote!(mul), parse_quote!(std::ops::Mul)),
      TraitImplPaths::new(parse_quote!(mul_assign), parse_quote!(std::ops::MulAssign)),
      TraitImplPaths::new(parse_quote!(mul_assign), parse_quote!(std::ops::MulAssign)),

      TraitImplPaths::new(parse_quote!(div), parse_quote!(std::ops::Div)),
      TraitImplPaths::new(parse_quote!(div), parse_quote!(std::ops::Div)),
      TraitImplPaths::new(parse_quote!(div), parse_quote!(std::ops::Div)),
      TraitImplPaths::new(parse_quote!(div), parse_quote!(std::ops::Div)),
      TraitImplPaths::new(parse_quote!(div_assign), parse_quote!(std::ops::DivAssign)),
      TraitImplPaths::new(parse_quote!(div_assign), parse_quote!(std::ops::DivAssign)),

      TraitImplPaths::new(parse_quote!(one), one_path.clone()),
      TraitImplPaths::new(parse_quote!(set_one), one_path.clone()),
      TraitImplPaths::new(parse_quote!(is_one), one_path.clone()),

      TraitImplPaths::new(parse_quote!(inv), inv_path.clone()),
      TraitImplPaths::new(parse_quote!(inv), inv_path.clone()),
    )
  }


  pub fn new_base(
    t_idents: Vec<syn::Ident>,

    mult: TraitImplPaths,
    ref_mult: TraitImplPaths,
    mult_bor: TraitImplPaths,
    ref_mult_bor: TraitImplPaths,
    mult_assign: TraitImplPaths,
    mult_assign_bor: TraitImplPaths,
    mult_inv: TraitImplPaths,
    ref_mult_inv: TraitImplPaths,
    mult_inv_bor: TraitImplPaths,
    ref_mult_inv_bor: TraitImplPaths,
    mult_assign_inv: TraitImplPaths,
    mult_assign_inv_bor: TraitImplPaths,
    unit: TraitImplPaths,
    set_unit: TraitImplPaths,
    is_unit: TraitImplPaths,
    inv: TraitImplPaths,
    ref_inv: TraitImplPaths,
  ) -> Self {
    Self {
      inner: Self::new_data_quote_paths(t_idents),

      mult_expr: mult.ext(parse_quote!(mult)),
      ref_mult_expr: ref_mult.ext(parse_quote!(ref_mult)),
      mult_bor_expr: mult_bor.ext(parse_quote!(mult_ref)),
      ref_mult_bor_expr: ref_mult_bor.ext(parse_quote!(ref_mult_ref)),
      mult_assign_expr: mult_assign.ext(parse_quote!(mult_assign)),
      mult_assign_bor_expr: mult_assign_bor.ext(parse_quote!(mult_assign_ref)),

      mult_inv_expr: mult_inv.ext(parse_quote!(mult_inv)),
      ref_mult_inv_expr: ref_mult_inv.ext(parse_quote!(ref_mult_inv)),
      mult_inv_bor_expr: mult_inv_bor.ext(parse_quote!(mult_inv_ref)),
      ref_mult_inv_bor_expr: ref_mult_inv_bor.ext(parse_quote!(ref_mult_inv_ref)),
      mult_assign_inv_expr: mult_assign_inv.ext(parse_quote!(mult_assign_inv)),
      mult_assign_inv_bor_expr: mult_assign_inv_bor.ext(parse_quote!(mult_assign_inv_ref)),

      unit_expr: unit.ext(parse_quote!(unit)),
      set_unit_expr: set_unit.ext(parse_quote!(set_unit)),
      is_unit_expr: is_unit.ext(parse_quote!(is_unit)),

      inv_expr: inv.ext(parse_quote!(inv)),
      ref_inv_expr: ref_inv.ext(parse_quote!(ref_inv)),
    }
  }

  fn new_data_quote_paths(t_idents: Vec<syn::Ident>) -> DataQuotePaths {
    DataQuotePaths {
      t_idents,
      t_fn_path: parse_quote!(uninitialized),
      default_fn_path: parse_quote!(uninitialized),
    }
  }
}


// impl<'x> QuoteParams<'x> {
//   pub const fn new<'a:'x,'b:'x, 'c:'x, 'd:'x>(lhs_path: &'a syn::Path, rhs_path: &'b syn::Path, ident: &'c syn::Ident, data: &'d syn::Data) -> Self {
impl<'a> QuoteParams<'a> {
  pub const fn new(lhs_path: &'a syn::Path, rhs_path: &'a syn::Path, ident: &'a syn::Ident, data: &'a syn::Data) -> Self {
    Self {
      lhs_path,
      rhs_path,
      ident,
      data,
    }
  }

  pub fn quote(&self, data_quote: DataQuote, args: (&DataQuotePaths, TraitImplPaths)) -> QuoteRes {
    let (paths, trait_impl_paths) = args;
    let expr = data_quote.quote(self.lhs_path, self.rhs_path, paths, self.ident, self.data);
    QuoteRes {
      expr,
      trait_impl_paths,
    }
  }
}


impl QuoteRes {
  pub fn unpack(self) -> (TokenStream, syn::Ident, syn::Path) {
    let Self { expr, trait_impl_paths } = self;
    let (fn_path, trait_path) = trait_impl_paths.unpack();
    (expr, fn_path, trait_path)
  }
}


macro_rules! impl_group_data_quote_paths_setter {
  ($($name:ident)*) => ($(
    impl GroupDataQuotePaths {
      pub fn $name(&mut self) -> (&DataQuotePaths, TraitImplPaths) {
        let TraitImplExtPaths {
          t_impl_path: t_fn_path,
          trait_impl_paths
        } = self.$name.clone();

        self.inner.t_fn_path = t_fn_path;
        self.inner.default_fn_path = trait_impl_paths.fn_path.clone().into();
        (&self.inner, trait_impl_paths)
      }
    }
  )*)
}

impl_group_data_quote_paths_setter!(
  mult_expr
  ref_mult_expr
  mult_bor_expr
  ref_mult_bor_expr
  mult_assign_expr
  mult_assign_bor_expr

  mult_inv_expr
  ref_mult_inv_expr
  mult_inv_bor_expr
  ref_mult_inv_bor_expr
  mult_assign_inv_expr
  mult_assign_inv_bor_expr

  unit_expr
  set_unit_expr
  is_unit_expr

  inv_expr
  ref_inv_expr
);


impl From<TraitImplExtPaths> for TraitImplPaths {
  fn from(value: TraitImplExtPaths) -> Self {
    value.trait_impl_paths
  }
}
