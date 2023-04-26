mod group;
mod wrappers;
pub(crate) mod util;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use util::group::GenGroupKind;


/// Derives `Deref` and optionally `DerefMut`. Derivation of `DerefMut` happens if the wrapped
/// field is `pub`.
#[proc_macro_derive(WrapperDeref)]
pub fn wrapper_deref(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  wrappers::wrapper_deref_impl(&input)
}

/// Derives `Group` functionalities only. It is recommended to derive from `GroupWrapper` instead.
/// Deriving from `PlainGroupWrapper` can be useful if you want to replace or extend some of the
/// functionality and need more fine grained control over the derived functionality.
#[proc_macro_derive(PlainGroupWrapper)]
pub fn plain_mult_group_wrapper(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  group::group_wrapper_impl(&input, GenGroupKind::Mult)
}

/// Derives `Group` functionalities only. It is recommended to derive from `GroupWrapper` instead.
/// Deriving from `PlainGroupWrapper` can be useful if you want to replace or extend some of the
/// functionality and need more fine grained control over the derived functionality.
#[proc_macro_derive(PlainAbelGroupWrapper)]
pub fn plain_abel_group_wrapper(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  group::group_wrapper_impl(&input, GenGroupKind::Abel)
}

/// Derives `Group` functionalities without assumed commutativity
#[proc_macro_derive(GroupWrapper)]
pub fn mult_group_wrapper(input: TokenStream) -> TokenStream {
  plain_mult_group_wrapper(input)
}

/// Derives `Group` functionalities with assumed commutativity
#[proc_macro_derive(AbelGroupWrapper)]
pub fn abel_group_wrapper(input: TokenStream) -> TokenStream {
  plain_abel_group_wrapper(input)
}


#[proc_macro_attribute]
pub fn num_traits_zero_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}

#[proc_macro_attribute]
pub fn num_traits_one_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}

#[proc_macro_attribute]
pub fn num_traits_inv_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}


#[proc_macro_attribute]
pub fn gen_abel_group_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}

#[proc_macro_attribute]
pub fn gen_group_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}


macro_rules! macro_err {
  ($input:ident, $msg:expr) => {
    syn::Error::new($input.span(), $msg).to_compile_error().into()
  };
}

pub(crate) use macro_err;

