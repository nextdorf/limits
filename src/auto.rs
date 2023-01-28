use std::marker::PhantomData;

/// Small Helper type for wrapping Types together with a Phantomdata Type. Commonly used in this
/// library to trivially lift certain Types into implementations of traits. For example the `CstFct`
/// type is generic over the output type `Y`, i.e. for every type `X` `CstFct` (logically)
/// implements `Fct<X, Y>`. But practically the trait it is not possible to implement the trait in
/// Rust for `CstFct<Y>` since `X` is nowhere specified in the type. However `Auto<CstFct<Y>, X>`
/// specifies the type in a `PhantomData`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Auto<X, A>(pub X, PhantomData<A>);


impl<X, A> Auto<X, A> {
  pub fn wrap(value: X) -> Self {
    Self(value, PhantomData)
  }
  
  pub fn unwrap(self) -> X {
    self.0
  }
  
  pub fn rewrap<B>(self) -> Auto<X, B> {
    Auto(self.0, PhantomData)
  }
}


