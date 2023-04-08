use std::borrow::Borrow;

pub use tensor::*;
// use tensor::vector_space::{Vec3D, FiniteVS};

struct Unit;

fn qqq<RUnit: Borrow<Unit>>(u: &RUnit) {
  u.borrow();
}

fn qq<RUnit: AsRef<Unit>>(u: &RUnit) {
  u.as_ref();
}

fn q(_: &Unit) {
}

impl AsRef<Unit> for () {
  fn as_ref(&self) -> &Unit {
    &Unit
  }
}

impl Borrow<Unit> for () {
  fn borrow(&self) -> &Unit {
    &Unit
  }
}


fn main() {
  println!("Hello, world!");

  qqq(&Unit);
  qqq(&());

  // qq(&Unit);
  qq(&());

  q(&Unit);
  // q(&());

  // let x: Vec3D<_> = VectorSpace(FiniteVS([1., 0., 0.]));
  // let [x, y, z]: [Vec3D<f64>; 3] = FiniteVS::unit_vecs().map(VectorSpace);
  // println!("{:?}, {:?}, {:?}", x.as_slice(), y.as_slice(), z.as_slice());
  // let t = x*1.25 + y*3. + z/2.4;
  // println!("{:?}", t.as_slice());
}


