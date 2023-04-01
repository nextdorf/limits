pub mod scalar;

pub trait SymExpr {
  type Param;
  type X;

  fn eval_expr(&self, args: Self::Param) -> Self::X;
}


impl<E: SymExpr> SymExpr for Vec<E> where E::Param: Clone {
  type Param = E::Param;
  type X = Vec<E::X>;

  fn eval_expr(&self, args: Self::Param) -> Self::X {
    let mut res = Vec::with_capacity(self.len());
    for e in self.iter() {
      res.push(e.eval_expr(args.clone()));
    }
    res
  }
}




