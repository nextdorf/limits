use proc_macro::TokenStream;

pub mod struct_vis;
pub mod unit_types;
pub mod path_attrs;
pub use struct_vis::{StructLookup, StructLookupPaths, LookupKind, LookupAccess};
pub use unit_types::get_unit_types;
pub use path_attrs::*;
pub use syn::visit::Visit;

pub(crate) fn many_results<T, U>(x: &T, fns: &[fn(&T) -> syn::Result<TokenStream>]) -> syn::Result<TokenStream> {
  let mut res = Vec::with_capacity(fns.len());
  for f in fns {
    res.push(f(x)?)
  }
  Ok(TokenStream::from_iter(res))
}
pub(crate) fn unwrap_ts(src_ident: &str, res: syn::Result<TokenStream>) -> TokenStream {
  match res {
    Ok(res) => res,
    Err(err) => {
      syn::Error::new(err.span(), format!("{}: {}", src_ident, err.to_string()))
        .into_compile_error()
        .into()
    },
  }
}


macro_rules! return_err {
  ($span:expr, $($arg:tt)* ) => {
    return Err(syn::Error::new($span, format!($($arg)*)))
  };
}
pub(crate) use return_err;

/// Calling
/// ```ignore
/// new_derive! {
///   /// Optional documentation
///   $name using $fn_name [from $path]
/// }
/// ```
/// yields
/// ```ignore
/// /// Optional documentation
/// #[proc_macro_derive($name)]
/// pub fn {$fn_name}_derive (input: TokenStream) -> TokenStream {
///   let input: DeriveInput = parse_macro_input!(input);
///   unwrap_ts("{$name}", [$path::]{$fn_name}_impl(&input))
/// }
/// ```
macro_rules! new_derive {
  (
    $(#[$outer:meta])*
    $name:ident using $fn_name:ident from $p:path
  ) => {
    paste::paste! {
      $(#[$outer])*
      #[proc_macro_derive($name)]
      pub fn [<$fn_name _derive>] (input: TokenStream) -> TokenStream {
        let input: DeriveInput = parse_macro_input!(input);
        unwrap_ts(stringify!($name), $p::[<$fn_name _impl>](&input))
      }
    }
  };
  (
    $(#[$outer:meta])*
    $name:ident using $fn_name:ident
  ) => {
    new_derive!(
      $(#[$outer])*
      $name using $fn_name from crate
    );
  };
}
pub(crate) use new_derive;

/// Calling
/// ```ignore
/// marker_attribute! {
///   /// Optional documentation
///   $name
/// }
/// ```
/// yields
/// ```ignore
/// /// Optional documentation
/// #[proc_macro_attribute]
/// pub fn $name(_attr: TokenStream, item: TokenStream) -> TokenStream {
///   item
/// }
/// ```
macro_rules! marker_attribute {
  (
    $(#[$outer:meta])*
    $name:ident
  ) => {
    $(#[$outer])*
    #[proc_macro_attribute]
    pub fn $name(_attr: TokenStream, item: TokenStream) -> TokenStream {
      item
    }
  };
}
pub(crate) use marker_attribute;


// #[cfg(test)]
pub mod tests {
  use quote::ToTokens;

  fn pre_eq_wo_whitespace(s1: impl ToString, s2: impl ToString) -> (String, String) {
    let s1 = {
      let mut s = s1.to_string();
      s.retain(|ch| !ch.is_whitespace());
      s
    };
    let s2 = {
      let mut s = s2.to_string();
      s.retain(|ch| !ch.is_whitespace());
      s
    };
    (s1, s2)
  }

  pub fn assert_eq_wo_whitespace(s1: impl ToString, s2: impl ToString) {
    let (s1, s2) = pre_eq_wo_whitespace(s1, s2);
    assert_eq!(s1, s2)
  }

  fn pre_eq_tokens(s1: &impl ToTokens, s2: &impl ToTokens) -> (String, String) {
    pre_eq_wo_whitespace(s1.to_token_stream(), s2.to_token_stream())
  }

  pub fn assert_eq_tokens(s1: &impl ToTokens, s2: &impl ToTokens) {
    let (s1, s2) = pre_eq_tokens(s1, s2);
    assert_eq!(s1, s2)
  }

  pub fn eq_tokens(s1: &impl ToTokens, s2: &impl ToTokens) -> bool {
    let (s1, s2) = pre_eq_tokens(s1, s2);
    s1 == s2
  }
}


// struct QW;

// impl ::num_traits::Zero for QW {
//     fn zero() -> Self {
//         todo!()
//     }
    

//     fn is_zero(&self) -> bool {
//         todo!()
//     }

//     fn set_zero(&mut self) {
//         *self = num_traits::Zero::zero();
//     }
// }
