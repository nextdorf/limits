use std::marker::PhantomData;

pub use tensor_derive_new::*;

#[derive(WrapperDeref)]
#[wrapper_deref(value)]
pub struct A<T> {
  internal: PhantomData<T>,
  pub value: u32,
}



#[derive(WrapperDeref)]
#[wrapper_deref]
pub struct B(pub (), pub u32);


#[derive(Group)]
pub struct G(pub i32);




