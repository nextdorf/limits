use super::Fct;

pub struct Sin;


impl<X> Fct<X, X> for Sin {
    fn eval_fct(&self, x: X) -> X {
        sin
    }
}



