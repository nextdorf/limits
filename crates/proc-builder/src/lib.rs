mod trait_derivation_builder;
mod attribute_builder;
mod function_builder;
pub mod types;
pub mod deep_type;
pub mod iter_async;

pub use trait_derivation_builder::*;
pub use attribute_builder::*;
pub use function_builder::*;
pub use iter_async::IterAsync;

pub type TokenStream = proc_macro2::TokenStream;


#[cfg(test)]
pub(crate) mod tests {
  pub fn assert_eq_wo_whitespace(s1: impl ToString, s2: impl ToString) {
    let mut s1 = s1.to_string();
    let mut s2 = s2.to_string();
    s1.retain(|ch| !ch.is_whitespace());
    s2.retain(|ch| !ch.is_whitespace());
    assert_eq!(s1, s2)
  }
}