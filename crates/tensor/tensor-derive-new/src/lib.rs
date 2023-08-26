mod wrappers;
mod group;
pub(crate) mod util;


use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use util::{unwrap_ts, new_derive, marker_attribute};

marker_attribute!(
  /// Allows to specify which element in the struct will be wrapped. Without this attribute the
  /// wrapper has to be a new-type of the wrapped value. If the wrapped value is not specified, then
  /// the first element in the data structure is used.
  wrapper_deref
);

// #[proc_macro_attribute]
// pub fn wrapper_deref(_attr: TokenStream, item: TokenStream) -> TokenStream {
//   item
// }

new_derive!(
  /// Derives `Deref` and optionally `DerefMut`. Derivation of `DerefMut` happens if the wrapped
  /// field is `pub`. If the wrapper is not a new-type consider adding the `#wrapper_deref` attribute.
  WrapperDeref using wrapper_deref from wrappers
);


new_derive!(
  GroupWrapper using group from group
);

new_derive!(
  AbelGroupWrapper using abel_group from group
);


marker_attribute!(zero_path);
marker_attribute!(one_path);
marker_attribute!(inv_path);
marker_attribute!(neg_path);
marker_attribute!(unit_types);
marker_attribute!(derive_generic);



