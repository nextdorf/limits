mod trait_impl;

pub mod num_field;
// pub mod vector_space;
pub mod group;

pub use group::{GenAbelGroup, GenGroup};
// pub use num_field::NumField;
// pub use vector_space::GenVectorSpace;

pub use ::num_traits::{Zero, One, Inv};
pub use ::core::ops::Neg;



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


pub(crate) use forward;




