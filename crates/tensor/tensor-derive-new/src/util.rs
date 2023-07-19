use proc_macro::TokenStream;

pub mod struct_vis;
pub use struct_vis::{StructLookup, StructLookupPaths, LookupKind, LookupAccess};
pub use syn::visit::Visit;

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


#[cfg(test)]
pub mod tests {
  pub fn assert_eq_wo_whitespace(s1: impl ToString, s2: impl ToString) {
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
    assert_eq!(s1, s2)
  }
}

