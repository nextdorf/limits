mod group;
mod wrappers;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(WrapperDeref)]
pub fn wrapper_deref(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  wrappers::wrapper_deref_impl(&input)
}

#[proc_macro_derive(PlainGroupWrapper)]
pub fn plain_group_wrapper(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  group::group_wrapper_impl(&input)
}

#[proc_macro_derive(GroupWrapper)]
pub fn group_wrapper(input: TokenStream) -> TokenStream {
  plain_group_wrapper(input)
}

