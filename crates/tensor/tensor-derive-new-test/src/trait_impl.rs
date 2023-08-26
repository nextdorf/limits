use ::core::{marker::PhantomData, fmt::Debug};
use crate::AbelG;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SomeUnit;

// #[automatically_derived]
// impl<T> ::core::marker::StructuralPartialEq for AbelG<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for AbelG<T> {
    #[inline]
    fn eq(&self, other: &AbelG<T>) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}
// #[automatically_derived]
// impl<T> ::core::marker::StructuralEq for AbelG<T> {}
#[automatically_derived]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for AbelG<T> { }
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for AbelG<T> {
    #[inline]
    fn partial_cmp(
        &self,
        other: &AbelG<T>,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&self.2, &other.2) {
                            ::core::option::Option::Some(
                                ::core::cmp::Ordering::Equal,
                            ) => ::core::cmp::PartialOrd::partial_cmp(&self.3, &other.3),
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Ord> ::core::cmp::Ord for AbelG<T> {
    #[inline]
    fn cmp(&self, other: &AbelG<T>) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.0, &other.0) {
            ::core::cmp::Ordering::Equal => {
                match ::core::cmp::Ord::cmp(&self.1, &other.1) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.2, &other.2) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.3, &other.3)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl<T: ::core::hash::Hash> ::core::hash::Hash for AbelG<T> {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state);
        ::core::hash::Hash::hash(&self.1, state);
        ::core::hash::Hash::hash(&self.2, state);
        ::core::hash::Hash::hash(&self.3, state)
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for AbelG<T> {
    #[inline]
    fn clone(&self) -> AbelG<T> {
        AbelG(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
            ::core::clone::Clone::clone(&self.3),
        )
    }
}



impl<T> AbelG<T> {
  pub fn rnds() -> impl Iterator<Item = Self> {
    use rand::random;
    Some(()).into_iter().cycle().map(|()| random::<AbelG<_>>())
  }
}


use rand::distributions::{Distribution, Standard};
impl<T> Distribution<AbelG<T>> for Standard {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> AbelG<T> {
    AbelG(self.sample(rng), self.sample(rng), PhantomData, SomeUnit)
  }
}

impl<T> Debug for AbelG<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("AbelG").field(&self.0).field(&self.1).field(&"..").finish()
  }
}
