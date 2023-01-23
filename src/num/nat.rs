use std::{ops::Add, boxed::ThinBox};

pub type NatRepr = u64;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nat {
  pub low_repr: NatRepr,
  pub high_repr: Option<Box<Nat>>,
}

pub struct OptNat<'a>(Option<&'a Nat>);


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

impl Nat {
  pub const fn iter(&self) -> OptNat {
    OptNat(Some(self))
  }
} 

impl Add for Nat {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let mut repr = Vec::new();
    let mut carry = 0;
    let (mut lhs_i, mut rhs_i) = (self.iter(), rhs.iter());
    loop {
      match (lhs_i.next(), rhs_i.next()) {
        (Some(x), Some(y)) => todo!(),
        (Some(x), None) | (None, Some(x)) => todo!(),
        (None, None) => {
          if carry > 0 {
            repr.push(carry);
          }
          break;
        },
      }
    }
    Nat::from_iter(repr)
  }
}

impl FromIterator<NatRepr> for Nat {
  fn from_iter<T: IntoIterator<Item = NatRepr>>(iter: T) -> Self {
    let mut iter = iter.into_iter();
    let mut res = if let Some(val) = iter.next() {
      Nat { low_repr: val, high_repr: None }
    } else {
      return Nat { low_repr: 0, high_repr: None }
    };
    let mut val_i = &mut res;
    while let Some(val) = iter.next() {
      val_i.high_repr = Some(Box::new(Nat { low_repr: val, high_repr: None }));
      val_i = &mut *val_i.high_repr.unwrap();
    }
    res
  }
}

impl Iterator for OptNat<'_> {
  type Item = NatRepr;

  fn next(&mut self) -> Option<Self::Item> {
    if let OptNat(Some(Nat { low_repr, high_repr })) = self {
      if let Some(high_repr) = high_repr {
        *self = OptNat(Some(&**high_repr));
      }
      Some(*low_repr)
    } else {
      None
    }
  }
}


