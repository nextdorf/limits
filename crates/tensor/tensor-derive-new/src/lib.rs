mod wrappers;
mod group;
pub(crate) mod util;


use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use util::unwrap_ts;

/// Allows to specify which element in the struct will be wrapped. Without this attribute the
/// wrapper has to be a new-type of the wrapped value. If the wrapped value is not specified, then
/// the first element in the data structure is used.
#[proc_macro_attribute]
pub fn wrapper_deref(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}



/// Derives `Deref` and optionally `DerefMut`. Derivation of `DerefMut` happens if the wrapped
/// field is `pub`. If the wrapper is not a new-type consider adding the `#wrapper_deref` attribute.
#[proc_macro_derive(WrapperDeref)]
pub fn wrapper_deref_derive(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  unwrap_ts("WrapperDeref", wrappers::wrapper_deref_impl(&input))
}


#[proc_macro_derive(Group)]
pub fn group_derive(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  unwrap_ts("Group", group::group_impl(&input))
}



