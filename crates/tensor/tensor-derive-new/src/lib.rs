mod wrappers;
pub(crate) mod util;


use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use util::unwrap_ts;

#[proc_macro_attribute]
pub fn wrapper_deref(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}
 

/// Derives `Deref` and optionally `DerefMut`. Derivation of `DerefMut` happens if the wrapped
/// field is `pub`.
#[proc_macro_derive(WrapperDeref)]
pub fn wrapper_deref_derive(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  unwrap_ts("WrapperDeref", wrappers::wrapper_deref_impl(&input))
}



