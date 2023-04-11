mod group;
mod wrappers;
pub(crate) mod util;

use proc_macro::TokenStream;
use syn::parse_macro_input;


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
pub fn plain_group_wrapper(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  group::group_wrapper_impl(&input)
}

/// Derives `Group` functionalities
#[proc_macro_derive(GroupWrapper)]
pub fn group_wrapper(input: TokenStream) -> TokenStream {
  plain_group_wrapper(input)
}

#[proc_macro_attribute]
pub fn num_traits_zero_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
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

