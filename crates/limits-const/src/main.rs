use limits::{self, ops::Sum, func::{Fct, cst::CstFct, Var, Diffable}};

fn main() {
  let s = Sum([1,2,3,4]);
  let fn_100 = CstFct(100);
  assert_eq!(100, fn_100.eval_fct());
  let dfn_100 = fn_100.diff_fct();
  assert_eq!(0, dfn_100.eval_fct());
}
