pub use tensor_derive_new::{
  wrapper_deref,
  derive_generic,
  inv_path,
  one_path,
  zero_path,
  neg_path,
  unit_types,
};

#[macro_export]
/// Forward a method to an inherent method or a base trait method.
macro_rules! forward {
  ($( Self :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
      => {$(
          #[inline]
          fn $method(self $( , $arg : $ty )* ) -> $ret {
              Self::$method(self $( , $arg )* )
          }
      )*};
  ($( $base:ident :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
      => {$(
          #[inline]
          fn $method(self $( , $arg : $ty )* ) -> $ret {
              <Self as $base>::$method(self $( , $arg )* )
          }
      )*};
  ($( $base:ident :: $method:ident ( $( $arg:ident : $ty:ty ),* ) -> $ret:ty ; )*)
      => {$(
          #[inline]
          fn $method( $( $arg : $ty ),* ) -> $ret {
              <Self as $base>::$method( $( $arg ),* )
          }
      )*};
  ($( $imp:path as $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
      => {$(
          #[inline]
          fn $method(self $( , $arg : $ty )* ) -> $ret {
              $imp(self $( , $arg )* )
          }
      )*};
}


#[macro_export]
/// Implements Deref Coersion for a trivial wrapper over some generic wrapper trait
/// 
/// It can either be used to wrap a trait:
/// ```
/// trait GenericWrapper {
///   //...
/// }
/// 
/// struct Wrapper<T: GenericWrapper>(pub T);
/// 
/// wrapper_deref!(GenericWrapper, Wrapper);
/// ```
/// 
/// or tt can either be used to wrap specific type:
/// ```
/// trait MyType {
///   //...
/// }
/// 
/// struct Wrapper(pub MyType);
/// 
/// wrapper_deref!(MyType, Wrapper);
/// ```
macro_rules! wrapper_deref {
  ($gen:tt, $w:tt) => (
    impl<T: $gen> std::ops::Deref for $w<T> {
      type Target = T;
      
      fn deref(&self) -> &T {
        &self.0
      }
    }
    
    impl<T: $gen> std::ops::DerefMut for $w<T> {
      fn deref_mut(&mut self) -> &mut T {
        &mut self.0
      }
    }
  );
}


