use std::{marker::PhantomData, ops::Mul};

use crate::{multivar::{MultiVar, DualMultiVar, Index}, num::{Zero, One}, vector::UnitVec};

use super::{Fct, Diffable, Var, cst::ZeroFct};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdFct<X>(PhantomData<X>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdMatrix<I>(PhantomData<I>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DIdFct<X>(PhantomData<X>);


impl<X> Fct for IdFct<X> {
  type X = X;
  type Y = X;

  fn eval_fct(&self, x: X) -> X {
    x
  }
}


// impl<Xs> Diffable<IdFct<Xs>> for IdFct<Var<Var<Xs>>> where Xs: Clone + MultiVar {
//     type Args = Var<Var<Xs>>;

//     type Xs = Var<Xs>; 

//     // type DFct = IdFct<Var<Var<Xs>>>;
//     type DFct = (UnitVec<Xs::I>, PhantomData<Var<Var<Xs>>>);

//     fn diff(&self, _: ()) -> IdFct<Xs> {
//       IdFct(PhantomData)
//     }
// }


impl<I, X> MultiVar for (IdMatrix<I>, PhantomData<X>) where I: Index, X: Zero + One {
  type I = I;
  type X = (UnitVec<I>, PhantomData<X>);

  fn elem_at_index(&self, i: I) -> (UnitVec<I>, PhantomData<X>) {
    (UnitVec(i), PhantomData)
  }
}

impl<I, X> Fct for (IdMatrix<I>, PhantomData<X>) where I: Index, X: Zero + One {
    type X;
    type Y;

    fn eval_fct(&self, x: Self::X) -> Self::Y {
        todo!()
    }
}

impl<Xs> DualMultiVar<IdMatrix<Xs::I>> for Var<Xs> where Xs: MultiVar + Clone {
  type DualI = Xs::I;

  fn dot(&self, dual: &IdMatrix<Xs::I>) -> Xs {
    dual.eval_fct(self.0.clone())
  }

  fn try_as_dual(&self) -> Option<IdMatrix<Xs::I>> {
    None
  }
}

impl<I> IdMatrix<I> where I: Index {
  pub fn elem_at_index(&self, i: I) -> UnitVec<I> {
    UnitVec(i)
  }

  pub fn 
} 



// impl<X> Diffable<DIdFct<Var<X>>> for IdFct<Var<Var<X>>> where X: Clone + Zero {
//   type Args = Var<Self::Xs>;
//   type Xs = Var<X>;
//   type DFct = (ZeroFct, PhantomData<(Var<Var<X>>, Var<Var<X>>)>);

//   fn diff(&self, _: ()) -> DIdFct<Var<X>> {
//     DIdFct(PhantomData)
//   }
// }
// impl<X> Diffable<DIdFct<Var<X>>> for IdFct<Var<Var<X>>> where X: Clone + Zero {
//   type Args = Var<Self::Xs>;
//   type Xs = Var<X>;
//   type DFct = (UnitVec<I>, PhantomData<X>);

//   fn diff(&self, _: ()) -> DIdFct<Var<X>> {
//     DIdFct(PhantomData)
//   }
// }


// impl<X> IdFct<X> where X: Clone + Zero {
//   pub fn diff(&self) -> DIdFct<Var<X>> {
//     DIdFct(PhantomData)
//   }

//   pub fn dot_diff(&self, dir: &Var<X>) -> X {
//     dir.dot(&self.diff())
//   }
// }

// impl<Xs> MultiVar for DIdFct<Xs> where Xs: MultiVar, Xs::X: Zero + One {
//   type I = Xs::I;
//   // type X = ZeroFct;
//   // type X = (ZeroFct, PhantomData<(Var<Xs>, Var<Xs>)>);
//   type X = Xs::X;

//   fn elem_at_index(&self, _: Xs::I) -> (ZeroFct, PhantomData<(Var<Xs>, Var<Xs>)>) {
//     &(ZeroFct, PhantomData)
//   }
// }

// impl<X> DualMultiVar<DIdFct<Var<X>>> for Var<X> where X: Clone {
//   type DualI = ();

//   fn dot(&self, _: &DIdFct<Var<X>>) -> Self::X {
//     self.0.clone()
//   }

//   fn as_dual(self) -> DIdFct<Var<X>> {
//     DIdFct(PhantomData)
//   }
// }
// impl<Xs> DualMultiVar<DIdFct<Var<X>>> for Var<X> where Xs: Clone {
//   type DualI = ();

//   fn dot(&self, _: &DIdFct<Var<X>>) -> Self::X {
//     self.0.clone()
//   }

//   fn as_dual(self) -> DIdFct<Var<X>> {
//     DIdFct(PhantomData)
//   }
// }


