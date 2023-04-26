pub mod attr_repr;
pub mod data_quote;
pub(crate) mod group;

pub fn eq_iter<T: Eq>(mut a: impl Iterator<Item = T>, mut b: impl Iterator<Item = T>) -> bool {
  loop {
    match (a.next(), b.next()) {
      (None, None) => return true,
      (Some(x), Some(y)) if x == y => continue,
      _ => return false,
    }
  }
}

pub fn eq_iter_over<T, U: Eq>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>, f: &impl Fn(T) -> U) -> bool {
  eq_iter(a.map(f), b.map(f))
}

