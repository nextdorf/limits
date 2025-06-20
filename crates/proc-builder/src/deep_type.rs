pub mod iter;

use iter::Iter;
use proc_macro2::{TokenStream};
use quote::{ToTokens, quote};
use syn::{Type, Expr, punctuated::Punctuated, Token, parse_quote, Ident, DataStruct};

use crate::types::AccessExpr;


#[derive(Clone)]
pub struct DeepTypeValue {
  pub path: Vec<TokenStream>,
  pub ty: Type,
  pub kind: DeepTypeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeepTypeKind {
  NamedStructElem,
  UnnamedStructElem,
  UnitStructElem,
}


impl DeepTypeValue {
  // pub fn as_expr(&self, access: InputDataAccess, base: &Expr) -> Expr {
  pub fn as_expr(&self, access: AccessExpr) -> Expr {
    // let base = match access {
    //   InputDataAccess::Owned => quote!(#base),
    //   InputDataAccess::Ref => quote!(&#base),
    //   InputDataAccess::MutRef => quote!(&mut #base),
    //   InputDataAccess::Borrowed => quote!(&#base.borrow()),
    // };
    let base = access.into_token_stream();
    if !self.path.is_empty() {
      let path = Punctuated::<TokenStream, Token![.]>::from_iter(self.path.clone()).into_token_stream();
      parse_quote!(#base.#path)
    } else {
      parse_quote!(#base)
    }
  }
}


pub fn join_structs<'a>(
  ident: &Ident,
  data: &DataStruct,
  expr_inputs: impl IntoIterator<Item = AccessExpr<'a>>,
  collector: &impl Fn(Vec<Expr>) -> TokenStream
) -> Result<Expr, ()>
{
  if let syn::Fields::Unit = data.fields {
    return Ok(parse_quote!(#ident));
  }
  let iter = Iter::new_from_data(ident, data).collect::<Vec<_>>();
  let expr_inputs = expr_inputs.into_iter().collect::<Vec<_>>();
  // let xss = expr_inputs.map(|(acc, expr)| iter.clone().into_iter());
  let xss = match expr_inputs.len() {
    0 => return Err(()),
    n => {
      let mut xss = Vec::with_capacity(n);
      for _ in 0..(n-1) {
        xss.push(iter.clone().into_iter())
      }
      xss.push(iter.into_iter());
      xss
  }};
  let collector = |xs: Vec<DeepTypeValue>| {
    debug_assert_eq!(expr_inputs.len(), xs.len());
    let xs = xs.into_iter()
      .zip(&expr_inputs)
      // .map(|(x, (access, base))| {x.as_expr(*access, base)})
      .map(|(x, access)| {x.as_expr(*access)})
      .collect();
    collector(xs)
  };

  match build_struct_from_iter(xss, &collector) {
    Ok(es) => match &data.fields {
      syn::Fields::Named(syn::FieldsNamed {named, ..}) => {
        debug_assert_eq!(named.len(), es.len());
        let es = named.into_iter().zip(es)
          .map(|(f, e)| {
            let f_ident = f.ident.as_ref().unwrap();
            quote!(#f_ident: #e)
          }).collect::<Punctuated<_, Token![,]>>();
        Ok(parse_quote!(#ident {#es}))
      },
      syn::Fields::Unnamed(_) => {
        let es = es.into_iter().collect::<Punctuated<_, Token![,]>>();
        Ok(parse_quote!(#ident (#es)))
      },
      syn::Fields::Unit => unreachable!(),
    },
    Err(()) => Err(()),
  }
}

pub fn collect_struct<Sep: Default>(
  ident: &Ident,
  data: &DataStruct,
  expr_access: AccessExpr,
  collector: &impl Fn(Expr) -> TokenStream
) -> Punctuated<Expr, Sep>
{
  Iter::new_from_data(ident, data)
    .map(|val| syn::parse2::<Expr>(collector(val.as_expr(expr_access))).unwrap())
    .collect()
}

pub fn map_struct(ident: &Ident, data: &DataStruct, access: AccessExpr, f: &impl Fn(Expr) -> TokenStream) -> Result<Expr, ()> {
  let collector = |mut es: Vec<_>| {
    debug_assert_eq!(es.len(), 1);
    f(es.pop().unwrap())
  };
  let expr_inputs = [access].into_iter();
  join_structs(ident, data, expr_inputs, &collector)
}

pub fn build_struct_from_iter(
  mut xss: Vec<impl Iterator<Item = DeepTypeValue>>,
  collector: &impl Fn(Vec<DeepTypeValue>) -> TokenStream
) -> Result<Vec<Expr>, ()>
{
  fn build_expr_from_src_rec(mut src: Vec<Vec<DeepTypeValue>>, collector: &impl Fn(Vec<DeepTypeValue>) -> TokenStream, depth: usize) -> Expr {
    fn get_path_at_depth(path: &Vec<TokenStream>, depth: usize) -> String {
      path.iter().nth(depth).map(ToString::to_string).unwrap_or_default()
    }

    let mut exprs = Punctuated::<Expr, Token![,]>::new();

    while !src.is_empty() {
      let (path_at_depth, path_len) = {
        let DeepTypeValue { path, .. } = src.first().unwrap().first().unwrap();
        (get_path_at_depth(path, depth), path.len())
      };
      let count_equals = src.iter()
        .take_while(|xs| get_path_at_depth(&xs.first().unwrap().path, depth) == path_at_depth)
        .count();

      let equals = {
        let mut tmp = src.split_off(count_equals);
        std::mem::swap(&mut tmp, &mut src);
        tmp
      };
      debug_assert_eq!(equals.len(), count_equals);
      let new_expr = if count_equals == 1 {
        let expr = collector(equals.into_iter().next().unwrap());
        let mut expr = parse_quote!(#expr);
        for _ in 1..(path_len-depth) {
          expr = parse_quote!((#expr,));
        }
        expr
      } else if path_at_depth == "" {
        let collected = equals.into_iter().map(collector).collect::<Punctuated<_, Token![,]>>();
        parse_quote!((#collected))
      } else {
        // let equals_without_path0 = equals.into_iter()
        //   .map(|es| {
        //     es.into_iter().map(|DeepTypeValue { mut path, ty, kind }| {
        //       DeepTypeValue { path: path.split_off(1), ty, kind }
        //     }).collect()
        //   }).collect();
        // build_expr_from_src(equals_without_path0, collector)
        build_expr_from_src_rec(equals, collector, depth + 1)
      };
      exprs.push(new_expr)
    }

    if exprs.len() == 1 {
      exprs.into_iter().next().unwrap()
    } else {
      parse_quote!((#exprs))
    }
  }

  fn build_expr_from_src(src: Vec<Vec<DeepTypeValue>>, collector: &impl Fn(Vec<DeepTypeValue>) -> TokenStream) -> Expr {
    build_expr_from_src_rec(src, collector, 0)
  }

  let mut res = Vec::new();
  let mut expr_build_src: Vec<Vec<DeepTypeValue>> = Vec::new();
  let xss_len = xss.len();
  loop {
    let mut next_vals = xss.iter_mut().map(Iterator::next);
    let (next_vals, path0) = match next_vals.next() {
      Some(Some(x)) => { //First iterator in xss has a value, all others should have one, too. In addition their pathes should be equal
        let mut buf = Vec::with_capacity(xss_len);
        let path = x.path.iter().map(ToString::to_string).collect::<Vec<_>>();
        buf.push(x);
        for x in next_vals {
          if let Some(x) = x {
            if x.path.len() == path.len() {
              for (p1, p2) in path.iter().zip(&x.path) {
                if *p1 != p2.to_string() {
                  return Err(());
                }
              }
            } else {
              return Err(());
            }
            buf.push(x)
          } else {
            return Err(());
          }
        }
        let path0 = path.into_iter().next().unwrap_or_default();
        (buf, path0.to_string())
      },
      Some(None) => { //First iterator in xss is drained, all others should be drained, too
        let all_none = next_vals.find(Option::is_some).is_none();
        if all_none {
          break;
        } else {
          return Err(());
        }
      },
      None => break, //xss is empty
    };

    if let Some(in_curr_expr) = expr_build_src.first() {
      let curr_path = &in_curr_expr.first().unwrap().path;
      if let Some(curr_path0) = curr_path.first() {
        // Non empty expression tree
        if path0 != curr_path0.to_string() {
          // belongs to new expression tree, and last one is fully "found"
          res.push(build_expr_from_src(std::mem::take(&mut expr_build_src), collector));
        }
        expr_build_src.push(next_vals)
      } else {
        unreachable!()
      }
    } else {
      expr_build_src.push(next_vals)
    }
  }
  if !expr_build_src.is_empty() {
    res.push(build_expr_from_src(std::mem::take(&mut expr_build_src), collector))
  }
  Ok(res)
}


#[cfg(test)]
mod tests {
  use proc_macro2::TokenStream;
  use quote::{quote, ToTokens};
  use syn::{parse_quote, Token, punctuated::Punctuated};
  // use crate::new_access_expr;

  use crate::{types::macros::new_access_expr, tests::assert_eq_wo_whitespace};

  use super::{DeepTypeValue, iter::Iter, build_struct_from_iter, map_struct};

  #[test]
  fn build_struct_unit() {
    use syn::DeriveInput;

    // let data: DeriveInput = parse_quote!(
    //   struct A(pub (i32, char), &'static usize, [u8; 9]);
    // );
    let data: DeriveInput = parse_quote!(
      struct B {
        pub vals: (i32, char),
        num: &'static usize,
        bytes: [u8; 9]
      }
    );
    // let base = parse_quote!(x);
    let iter = if let syn::Data::Struct(s) = &data.data {
      Iter::new_from_data(&data.ident, s)
    } else {
      panic!()
    };

    let iter = iter.collect::<Vec<_>>();
    let collect_fn = |mut xs: Vec<DeepTypeValue>| {
      let a = xs.pop().unwrap().as_expr(new_access_expr!(Owned, x));
      let b = xs.pop().unwrap().as_expr(new_access_expr!(Ref, y));
      // let a = xs.pop().unwrap().as_expr(InputDataAccess::Owned, &parse_quote!(x));
      // let b = xs.pop().unwrap().as_expr(InputDataAccess::Ref, &parse_quote!(y));
      assert!(xs.is_empty());
      quote!(#a + #b)
    };
    let res = build_struct_from_iter(
      vec![iter.clone().into_iter(), iter.into_iter()],
      &collect_fn
    ).unwrap();

    let res = Punctuated::<_, Token![;]>::from_iter(res.into_iter());
    // assert_eq!(res.to_token_stream().to_string(), "(x . vals . 0 + & y . vals . 0 , x . vals . 1 + & y . vals . 1) ; x . num + & y . num ; x . bytes + & y . bytes")
    // assert_eq!(
    //   without_whitespace(&res.into_token_stream()),
    //   without_whitespace(&"(x.vals.0 + &y.vals.0, x.vals.1 + &y.vals.1); x.num + &y.num; x.bytes + &y.bytes")
    // )
    assert_eq_wo_whitespace(
      res.into_token_stream(),
      "(x.vals.0 + &y.vals.0, x.vals.1 + &y.vals.1); x.num + &y.num; x.bytes + &y.bytes"
    )
  }

  #[test]
  fn map_struct_unit() {
    use syn::DeriveInput;
    fn assert_derived_input(data: DeriveInput, target: TokenStream) {
      let res = map_struct(
        &data.ident,
        match &data.data {syn::Data::Struct(s) => s, _ => unreachable!()},
        new_access_expr!(Ref, x),
        &|e| parse_quote!(#e.calc_smth())
      ).unwrap();
      assert_eq_wo_whitespace(res.to_token_stream(), target)
    }
    let call_fn = quote!(calc_smth);

    assert_derived_input(
      parse_quote!(
        struct A {
          pub vals: (i32, char),
          num: &usize,
          bytes: [u8; 9]
        }
      ),
      quote!(A {
        vals: (&x.vals.0.#call_fn(), &x.vals.1.#call_fn()),
        num: &x.num.#call_fn(),
        bytes: &x.bytes.#call_fn()
      })
    );

    assert_derived_input(
      parse_quote!(
        struct B(i32, (i32, i32), ((i32, ), (i32, i32)));
      ),
      quote!(B(
        &x.0.#call_fn(),
        (&x.1.0.#call_fn(), &x.1.1.#call_fn()),
        ((&x.2.0.0.#call_fn(), ), (&x.2.1.0.#call_fn(), &x.2.1.1.#call_fn()))
      ))
    );

    assert_derived_input(
      parse_quote!(
        struct C;
      ),
      quote!(C)
    );
  }
}

