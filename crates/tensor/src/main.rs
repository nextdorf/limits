use ndarray::Array;


fn main() {
  let eps = Array::from_shape_fn((3,3,3), |(i, j, k)| 
  match [i, j, k] {
    [0, 1, 2] | [1, 2, 0] | [2, 0, 1] => 1.,
    [2, 1, 0] | [1, 0, 2] | [0, 2, 1] => -1.,
    _ => 0.
  });

  // let v1 = ndarray::array![1., 0., 0.];
  // let v2 = ndarray::array![0., 1., 0.];
  // let v3 = ndarray::array![0., 0., 1.];

  let a = ndarray::array![1.654f64, 0.456, -1.5464];
  let b = ndarray::array![-0.4564f64, 0.5464, 1.87978];

  // let c = <NDArrayAlgebra<_> as TensorAlgebra>::contraction_with(a.clone().into_dyn(), b.clone().into_dyn(), 0, 0).unwrap();
  let c_0 = a.dot(&b);
  let c = c_0;

  println!("{a} * {b} = {c} [{c_0}]");
  println!("Eps: {eps}");
}
