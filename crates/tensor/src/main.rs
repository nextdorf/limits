pub use tensor::*;
use tensor::vector_space::{Vec3D, FiniteVS};

fn main() {
  println!("Hello, world!");

  // let x: Vec3D<_> = VectorSpace(FiniteVS([1., 0., 0.]));
  let [x, y, z]: [Vec3D<f64>; 3] = FiniteVS::unit_vecs().map(VectorSpace);
  println!("{:?}, {:?}, {:?}", x.as_slice(), y.as_slice(), z.as_slice());
  let t = x*1.25 + y*3. + z/2.4;
  println!("{:?}", t.as_slice());
}

