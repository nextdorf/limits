pub use tensor_derive_new::*;

#[derive(WrapperDeref)]
#[wrapper_deref(value)]
pub struct A {
  wrong: (),
  pub value: u32,
}



