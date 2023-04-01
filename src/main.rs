fn main() {
  let mut i = num_complex::Complex64::new(0., 1.);
  i = i.exp();
  let q = i + 1.;
  // let qq: num_complex::Complex64 = 1.;
  // println!("{}", limits::num::c128(0., 1.) / 0.);
  println!("{}", num_complex::Complex64::new(0., -1.));
}

