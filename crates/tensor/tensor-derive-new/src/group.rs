mod common;
mod mult_group;
mod abel_group;


pub use mult_group::group_impl;
pub use abel_group::abel_group_impl;


// use tensor_traits::GenGroup;
// pub struct G(pub i32);

// impl GenGroup for G {
//   fn mult(self, _: Self) -> Self { unimplemented!() }
//   fn ref_mult(&self, _: Self) -> Self { unimplemented!() }
//   fn mult_ref(self, _: &Self) -> Self { unimplemented!() }
//   fn ref_mult_ref(&self, _: &Self) -> Self { unimplemented!() }
//   fn mult_assign(&mut self, _: Self) { unimplemented!() }
//   fn mult_assign_ref(&mut self, _: &Self) { unimplemented!() }

//   fn mult_inv(self, _: Self) -> Self { unimplemented!() }
//   fn ref_mult_inv(&self, _: Self) -> Self { unimplemented!() }
//   fn mult_inv_ref(self, _: &Self) -> Self { unimplemented!() }
//   fn ref_mult_inv_ref(&self, _: &Self) -> Self { unimplemented!() }
//   fn mult_assign_inv(&mut self, _: Self) { unimplemented!() }
//   fn mult_assign_inv_ref(&mut self, _: &Self) { unimplemented!() }

//   fn inv(self) -> Self { unimplemented!() }
//   fn ref_inv(&self) -> Self { unimplemented!() }

//   fn unit() -> Self { unimplemented!() }
//   fn set_unit(&mut self) { unimplemented!() }
//   fn is_unit(&self) -> bool { unimplemented!() }
// }



