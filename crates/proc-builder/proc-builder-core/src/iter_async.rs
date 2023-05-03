use std::{task::{Waker, Context}, pin::Pin};
use futures::{Future, FutureExt, task::noop_waker};

pub struct IterAsync<'a, T> {
  waker: Waker,
  next_val: Pin<Box<Option<T>>>,
  future: Pin<Box<dyn Future<Output = ()> + 'a>>,
}

#[macro_export]
macro_rules! yield_async {
  ($out:ident <- $val:expr) => {{
    *$out = Some($val);
    futures::pending!()
  }};
  ($out:ident) => {
    *$out = None
  }
}


impl<'a, 't: 'a, T: 't> IterAsync<'a, T> {
  pub fn new<F: Future<Output = ()> + 'a>(f: impl Fn(&'a mut Option<T>) -> F, waker: Waker) -> Self {
    let (future, next_val) = unsafe {
      // f(&mut *(next_val.borrow_mut())).boxed_local()
      let next_val = Box::into_raw(Box::new(None));
      let f = f(&mut *next_val).boxed_local();
      let next_val = Box::into_pin(Box::from_raw(next_val));
      (f, next_val)
    };
    Self { waker, next_val, future }
  }

  pub fn new_without_waker<F: Future<Output = ()> + 'a>(f: impl Fn(&'a mut Option<T>) -> F) -> Self {
    Self::new(f, noop_waker())
  }
}


impl<T> IterAsync<'_, T> {
  fn next_one(&mut self) {
    loop {
      let mut ctx = Context::from_waker(&self.waker);
      match self.future.as_mut().poll(&mut ctx) {
        std::task::Poll::Pending if self.next_val.is_none() => {},
        _ => break,
      }
    }
  }
}


impl<'a, 't: 'a, T: Clone + 't> IterAsync<'a, T> {
  pub fn into_pinned_iter(mut self) -> impl Iterator<Item = T> + 'a {
    Some(()).into_iter().cycle().map_while(move |()| {
      self.next_val.set(None);
      self.next_one();
      (*self.next_val).clone()
    })
  }
}


impl<'a, T: Unpin> Iterator for IterAsync<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    self.next_one();
    self.next_val.take()
  }
}

// impl<'a, T: Clone> Iterator for IterAsync<'a, T> {
//   type Item = T;

//   fn next(&mut self) -> Option<T> {
//     self.next_one();
//     self.next_val.clone()
//   }
// }



#[test]
fn test_iter_async() {
  use std::fmt::Debug;

  async fn inner_f<T: Clone>(xs: &Vec<T>, out: &mut Option<T>) {
    let mut i = 0;
    let mut j = 0;
    while let Some(x) = xs.get(i) {
      yield_async!(out <- x.clone());
      j+=1;
      i+=j+1;
    }
  }

  fn assert_iter_a<T: Clone + PartialEq + Debug + Unpin>(xs: Vec<T>) {
    let it = IterAsync::new_without_waker(
      |out| inner_f(&xs, out)
    );
    let mut xs = xs.iter();
    for (i, x) in it.enumerate() {
      let y = xs.nth(i).unwrap();
      assert_eq!(x, y.clone())
    }
  }

  fn assert_iter_b<T: Clone + PartialEq + Debug>(xs: Vec<T>) {
    let it = IterAsync::new_without_waker(
      |out| inner_f(&xs, out)
    );
    let mut xs = xs.iter();
    for (i, x) in it.into_pinned_iter().enumerate() {
      let y = xs.nth(i).unwrap();
      assert_eq!(x, y.clone())
    }
  }

  fn assert_iter<T: Clone + PartialEq + Debug + Unpin>(xs: Vec<T>) {
    assert_iter_a(xs.clone());
    assert_iter_b(xs);
  }

  assert_iter(vec![1,2,3,4,5,6,7,8,9,10]);
  assert_iter(vec!["a","b","c","d","e","f","g","h","i","j","k"]);
}

