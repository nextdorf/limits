pub use tensor::*;
use tensor::group::{Permutation, Group};
// use tensor::vector_space::{Vec3D, FiniteVS};



fn main() {
  println!("Hello, world!");

  let p1 = Group(Permutation::try_from([1,3,0,2]).unwrap());
  let p2 = Group(Permutation::try_from([3,2,0,1]).unwrap());

  println!("{} * {} = {}", p1.to_string(), p2.to_string(), (p1 * p2).to_string());
  println!("{} / {} = {}", p1.to_string(), p2.to_string(), (p1 / p2).to_string());
  println!("{}^-1 = {}", p1.to_string(), p1.ref_inv().to_string());
  println!("{}^-1 = {}", p2.to_string(), p2.ref_inv().to_string());


  // let x: Vec3D<_> = VectorSpace(FiniteVS([1., 0., 0.]));
  // let [x, y, z]: [Vec3D<f64>; 3] = FiniteVS::unit_vecs().map(VectorSpace);
  // println!("{:?}, {:?}, {:?}", x.as_slice(), y.as_slice(), z.as_slice());
  // let t = x*1.25 + y*3. + z/2.4;
  // println!("{:?}", t.as_slice());
}


