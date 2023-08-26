#[allow(unused_imports)]
use rand::random;
#[allow(unused_imports)]
use tensor_traits::Zero;
#[allow(unused_imports)]
use crate::AbelG;

#[test]
fn exits_0() {
  let g0 = AbelG::<()>::zero();
  for g in AbelG::rnds().take(100) {
    assert_eq!(g, &g + &g0);
    assert_eq!(g, &g0 + &g);
    assert_eq!(g0, &g - &g);
    assert_eq!(g0, &g + (-&g));
    assert_eq!(g0, (-&g) + &g);
  }
}

#[test]
  fn associative() {
  for a in AbelG::<()>::rnds().take(30) {
    for b in AbelG::rnds().take(30) {
      for c in AbelG::rnds().take(30) {
        assert_eq!((&a + &b) + &c, &a + (&b + &c));
      }
    }
  }
}

#[test]
fn commutative() {
  for a in AbelG::<()>::rnds().take(80) {
    for b in AbelG::rnds().take(80) {
      assert_eq!(&a + &b, &b + &a);
    }
  }
}  