use std::ops::Add;

pub struct BigUint {
  vals: &'static [u32]
}

pub struct SBigUint<const N: usize> {
  pub vals: [u32; N]
}

pub struct BU32<const Vals: [u32]>;

const fn add_bu32_h(a: &[u32], b: &[u32]) -> &'static [u32] { todo!() }

const fn add_bu32<const xs: [u32], const ys: [u32]>(a: BU32<xs>, b: BU32<ys>) { todo!() }



// #[const_trait]
// trait TBint {
//   const N: usize;
//   fn new(x: &[u32; Self::N]) -> Self;
//   fn repr(&self) -> [u32; Self::N];
// }

// impl<const N: usize> const TBint for [u32; N] {
//   const N: usize = N;
//   fn new(x: &[u32; Self::N]) -> Self {
//     unsafe {std::mem::transmute_copy(x)}
//   }
//   fn repr(&self) -> [u32; Self::N] {
//     unsafe {std::mem::transmute_copy(self)}
//   }
// }

// const fn add_big<A: ~const TBint + Copy, B: ~const TBint + Copy>(a: A, b: B) -> impl TBint where [(); A::N]:, [(); B::N]: {
//   let a_repr = a.repr();
//   let b_repr = b.repr();
//   if a_repr.len() > 0 {
//     <[u32; 0] as TBint>::new(&[0u32;0])
//   } else {
//     <[u32; 1] as TBint>::new(&[0u32;1])
//   }
// }

impl BigUint {
  pub const fn new(vals: &'static [u32]) -> Self {
    Self { vals }
  }

  const fn new_with_carry(vals: &'static [u32], carries: &'static [bool]) -> Self {
    let a = BigUint::new(&[1,2,3]);
    let b = BigUint::new(&[1,2,3, 4]);
    let c = [2,4,6,4];
    a
  }
}

impl const Add for BigUint {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    todo!()
  }
}



