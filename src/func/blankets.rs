use std::marker::PhantomData;

use super::{MultiVar, Fct};

// impl<Xs: MultiVar, F: Fct<X = Xs> + Clone, Fi: Fct<X = Xs::X, Y = F::Y>> const MultiVar for F {
//   type I = Xs::I;
//   type X = Fi;

// impl<Xs: MultiVar, F: Fct<X = Xs> + Clone, Fi: Fct<X = Xs::X, Y = F::Y>> const MultiVar for (F, PhantomData<Fi>) {
//   type I = Xs::I;
//   type X = Fi;

// // impl<Xs: MultiVar, F: Fct<X = Xs> + Clone> const MultiVar for F
// //   where Self::X: Fct<X = Xs::X, Y = F::Y>
// // {
// //   type I = Xs::I;

//   fn get_idx(&self, args: Self::I) -> Self::X {
//       todo!()
//   }

//   fn trace_idxs(self) -> Self::X {
//       todo!()
//   }

//   fn mult_idxs(self, rhs: Self) -> Self {
//       todo!()
//   }
// }





