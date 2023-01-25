use std::marker::PhantomData;

use crate::num::Zero;

use super::{Fct, Diffable, Var, NormedSpace, DualMultiVar, MultiVar, MultiVarFromIndex};

#[derive(Clone, Copy)]
pub struct IdFct<X>(pub X);

#[derive(Clone, Copy)]
pub struct DIdFct<F>(PhantomData<F>);

impl<X> Fct for IdFct<X> {
  type X = Var<X>;
  type Y = X;

  fn eval_fct(&self, x: Self::X) -> Self::Y {
    x.0
  }
}


// impl<F> MultiVar for DIdFct<F> {
//     type I = ();
//     type X = F;
//     type Mapped<Y> = DIdFct<Y> where Y: Copy;

//     fn get_idx(&self, arg: Self::I) -> Self::X {
//         todo!()
//     }

//     fn map<Y: Copy>(self, map_fn: &impl Fn(Self::X) -> Y) -> Self::Mapped<Y> {
//         todo!()
//     }
// }




// impl<X> Diffable for IdFct<X> where
//   X: NormedSpace + Copy + MultiVarFromIndex + ~const Zero, X::Dual: NormedSpace, <Self::Args as MultiVar>::I: PartialEq
// {
//   type Args = Var<Self::XVar>;
//   type XVar = X;
//   type DFct = Self;
//   type DF = DIdFct; // Kroneker delta

//   fn diff_fct(&self, _: <Self::Args as MultiVar>::I) -> Self::DF {
//     MultiVarFromIndex::new_from_index(&|i| Var(
//       NormedSpace::unit_dual_vector(i)
//     ))

//   }
// }




