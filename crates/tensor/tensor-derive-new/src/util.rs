use proc_macro::TokenStream;

pub mod struct_vis;


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



