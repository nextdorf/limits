use std::{ops::Add, boxed::ThinBox};

type NatRepr = u64;

pub struct Nat {
  pub low_repr: NatRepr,
  pub high_repr: Option<ThinBox<Nat>>,
}



// impl const Add for Nat {
//   type Output = Self;

//   fn add(self, rhs: Self) -> Self::Output {
//     let mut repr = Vec::with_capacity(self.repr.capacity().max(rhs.repr.capacity()));
//     let mut carry = 0;
//     let (mut lhs_i, mut rhs_i) = (self.repr.i, rhs.repr.iter());
//     loop {
//       match (lhs_i.next(), rhs_i.next()) {
//         (Some(x), Some(y)) => todo!(),
//         (Some(x), None) | (None, Some(x)) => todo!(),
//         (None, None) => {
//           if carry > 0 {
//             repr.push(carry);
//           }
//           break;
//         },
//       }
//     }
//     Self { repr }
//   }
// }

pub const fn iter_nat(n: Nat) -> (NatRepr, Option<Nat>) {
  match n {
    Nat { low_repr, high_repr: Some(high_repr) } => (low_repr, Some(*high_repr)),
    Nat { low_repr, high_repr: None } => (low_repr, None),
  }
}
