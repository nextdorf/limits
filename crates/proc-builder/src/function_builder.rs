use std::rc::Rc;

use quote::ToTokens;
use syn::{Ident, Type, FnArg, parse_quote, parse::{self, Parse}, ItemFn, Signature, Generics, Token, punctuated::Punctuated, Block, spanned::Spanned, ReturnType};

use crate::{types::{InputDataAccess, AccessExprOwned}};

#[derive(Clone)]
pub struct FnArgBuilder {
  pub access: InputDataAccess,
  pub var_name: Option<Ident>,
  pub ty: Type,
}


#[derive(Clone)]
pub struct FnBuilder {
  pub vis: syn::Visibility,
  pub ident: Ident,
  pub input: Vec<FnArgBuilder>,
  pub output: ReturnType,
  pub generics: Generics,
  pub var_store: Option<Rc<dyn ::core::any::Any>>,
  pub calc_body: Rc<dyn Fn(Option<&dyn std::any::Any>) -> syn::Block>
}


impl FnArgBuilder {
  pub fn get_access_expr(&self) -> AccessExprOwned {
    AccessExprOwned {
      access: self.access,
      base: match &self.var_name {
        Some(name) => parse_quote!(#name),
        None => parse_quote!(self),
      }
    }
  }

  pub fn build(&self) -> FnArg {
    let FnArgBuilder { access, var_name, ty } = self;
    match var_name {
      Some(name) => {
        let pat = parse_quote!(#name);
        let ty = Box::new(match access {
          InputDataAccess::Owned => ty.clone(),
          InputDataAccess::Ref | InputDataAccess::Borrowed => Type::Reference(parse_quote!(& #ty)),
          InputDataAccess::MutRef => Type::Reference(parse_quote!(&mut #ty)),
        });
        FnArg::Typed(syn::PatType { attrs: Vec::new(), pat, colon_token: parse_quote!(:), ty })
      },
      None => {
        let reference = match access {
          InputDataAccess::Owned => None,
          _ => Some((parse_quote!(&), None)),
        };
        let mutability = match access {
          InputDataAccess::MutRef => Some(parse_quote!(mut)),
          _ => None,
        };
        let ty = Box::new(ty.clone());
        FnArg::Receiver(syn::Receiver { attrs: Vec::new(), reference, mutability, self_token: parse_quote!(self), colon_token: None, ty})
      },
    }
  }

  pub fn inner_parse(fn_arg: FnArg) -> syn::Result<Self> {
    match fn_arg {
      FnArg::Receiver(syn::Receiver { reference, mutability, ty, .. }) => {
        let access = match (reference, mutability) {
          (None, None) => InputDataAccess::Owned,
          (None, Some(m)) => return Err(syn::Error::new(m.span, "Did you mean 'ref mut'?")),
          (Some((_, None)), None) => InputDataAccess::Ref,
          (Some((_, None)), Some(_)) => InputDataAccess::MutRef,
          (Some((_, Some(l))), _) => return Err(syn::Error::new(l.span(), "Lifetimes are not supported yet")),
        };
        let var_name = None;
        let ty = *ty;
        Ok(Self { access, var_name, ty })
      },
      FnArg::Typed(syn::PatType { pat, mut ty, colon_token, .. }) => {
        let var_name = match *pat {
          syn::Pat::Ident(syn::PatIdent { by_ref: None, ident, subpat: None, .. }) => {
            Some(ident)
          },
          _ => {
            let span = colon_token.span;
            return Err(syn::Error::new(span, "Unsupported naming pattern"))
          },
        };
        while let Type::Paren(syn::TypeParen { elem, .. }) = *ty {
          ty = elem
        }
        let (access, ty) = match *ty {
          Type::Paren(_) => unreachable!(),
          Type::Ptr(syn::TypePtr { star_token, .. }) => {
            return Err(syn::Error::new(star_token.span, "Pointers not recognized yet"))
          },
          Type::Reference(syn::TypeReference { lifetime, mutability, elem, .. }) => {
            if let Some(l) = lifetime {
              return Err(syn::Error::new(l.span(), "Lifetimes are not supported yet"))
            } else {
              let access = mutability.and(Some(InputDataAccess::MutRef)).unwrap_or(InputDataAccess::Ref);
              (access, *elem)
            }
          },
          ty => (InputDataAccess::Owned, ty),
        };
        Ok(Self { access, var_name, ty })
      },
    }
  }
}

impl FnBuilder {
  pub fn get_signature(&self) -> Signature {
    let ident = &self.ident;
    let input = self.input.iter()
      .map(FnArgBuilder::build)
      .collect::<Punctuated<_, Token![,]>>();
    let output = &self.output;
    let generics = &self.generics;
    let mut res: Signature = parse_quote!(fn #ident(#input) #output);
    res.generics = generics.clone();
    res
  }
  
  pub fn new(ident: Ident) -> Self {
    fn todo_fn(_: Option<&dyn ::core::any::Any>) -> Block {
      parse_quote!({})
    }
    Self {
      vis: syn::Visibility::Inherited,
      ident,
      input: Vec::new(),
      output: ReturnType::Default,
      generics: Generics::default(),
      var_store: None,
      calc_body: Rc::new(todo_fn),
    }
  }


  pub fn set_visibility(mut self, vis: syn::Visibility) -> Self {
    self.vis = vis;
    self
  }

  pub fn set_ident(mut self, ident: Ident) -> Self {
    self.ident = ident;
    self
  }

  pub fn set_input(mut self, input: impl IntoIterator<Item = FnArgBuilder>) -> Self {
    self.input = input.into_iter().collect();
    self
  }

  pub fn set_output(mut self, output: Option<Type>) -> Self {
    self.output = output
      .map(|ty| ReturnType::Type(parse_quote!(->), Box::new(ty)))
      .unwrap_or(ReturnType::Default);
    self
  }


  pub fn set_body_to_struct_builder(
    mut self,
    ident: &Ident,
    data: &syn::DataStruct,
    build_collector: &impl Fn(&Vec<FnArgBuilder>) -> Box<dyn Fn(Vec<syn::Expr>) -> proc_macro2::TokenStream>
  ) -> Self {
    use crate::deep_type::join_structs;
    let exprs = self.input.iter().map(FnArgBuilder::get_access_expr).collect::<Vec<_>>();
    let collector = build_collector(&self.input);

    let ident = ident.clone();
    let data = data.clone();
    self.var_store = Some(Rc::new((ident.clone(), data.clone())));
    self.calc_body = Rc::new(move |var| {
      let (ident, data) = match var {
        Some(var) => match var.downcast_ref() {
          Some((ident, data)) => (ident, data),
          None => (&ident, &data),
        }
        None => (&ident, &data),
      };
      let exprs = exprs.iter().map(AccessExprOwned::as_ref);
      let collector = |es| collector(es);
      let body_expr = join_structs(ident, data, exprs, &collector)
        .expect("Could not join struct");
      parse_quote!({#body_expr})
    });
    self
  }

  pub fn set_body_to_struct_map_builder(
    mut self,
    ident: &Ident,
    data: &syn::DataStruct,
    build_collector: &impl Fn(&FnArgBuilder) -> Box<dyn Fn(syn::Expr) -> proc_macro2::TokenStream>
  ) -> Self {
    use crate::deep_type::map_struct;

    let input0 = match self.get_and_assert_one_arg() {
      Some(x) => x,
      None => return self,
    };
    let expr = input0.get_access_expr();
    let collector = build_collector(input0);

    let ident = ident.clone();
    let data = data.clone();
    self.var_store = Some(Rc::new((ident.clone(), data.clone())));
    self.calc_body = Rc::new(move |var| {
      let (ident, data) = match var {
        Some(var) => match var.downcast_ref() {
          Some((ident, data)) => (ident, data),
          None => (&ident, &data),
        }
        None => (&ident, &data),
      };
      let expr = expr.as_ref();
      let collector = |e| collector(e);
      let body_expr = map_struct(ident, data, expr, &collector)
        .expect("Could not map struct");
      parse_quote!({#body_expr})
    });
    self
  }

  pub fn set_body_to_collect_builder<Sep: Default + ToTokens>(
    mut self,
    ident: &Ident,
    data: &syn::DataStruct,
    build_collector: &impl Fn(&FnArgBuilder) -> Box<dyn Fn(syn::Expr) -> proc_macro2::TokenStream>
  ) -> Self {
    use crate::deep_type::collect_struct;

    let input0 = match self.get_and_assert_one_arg() {
      Some(x) => x,
      None => return self,
    };
    let expr = input0.get_access_expr();
    let collector = build_collector(input0);

    let ident = ident.clone();
    let data = data.clone();
    self.var_store = Some(Rc::new((ident.clone(), data.clone())));
    self.calc_body = Rc::new(move |var| {
      let (ident, data) = match var {
        Some(var) => match var.downcast_ref() {
          Some((ident, data)) => (ident, data),
          None => (&ident, &data),
        }
        None => (&ident, &data),
      };
      let expr = expr.as_ref();
      let collector = |e| collector(e);
      let body_expr = collect_struct::<Sep>(ident, data, expr, &collector);
        // .expect("Could not collect struct");
      parse_quote!({#body_expr})
    });
    self
  }


  fn get_and_assert_one_arg(&self) -> Option<&FnArgBuilder> {
    let check = if cfg!(test) {
      assert_eq!(self.input.len(), 1);
      true
    } else {
      self.input.len() == 1
    };

    if check {
      self.input.first()
    } else {
      None
    }
  }

  pub fn get_body(&self) -> Block {
    self.calc_body.as_ref()(self.var_store.as_deref())
  }

  pub fn build(&self) -> ItemFn {
    ItemFn {
      attrs: Vec::new(),
      vis: self.vis.clone(),
      sig: self.get_signature(),
      block: Box::new(self.get_body()),
    }
  }
}



impl From<FnArgBuilder> for FnArg {
  fn from(value: FnArgBuilder) -> Self {
    value.build()
  }
}
impl From<FnBuilder> for ItemFn {
  fn from(value: FnBuilder) -> Self {
    value.build()
  }
}


impl ToTokens for FnArgBuilder {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.build().to_tokens(tokens)
  }
}
impl ToTokens for FnBuilder {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.build().to_tokens(tokens)
  }
}


impl Parse for FnArgBuilder {
  fn parse(input: parse::ParseStream) -> syn::Result<Self> {
    FnArg::parse(input).and_then(Self::inner_parse)
  }
}


impl Parse for FnBuilder {
  fn parse(input: parse::ParseStream) -> syn::Result<Self> {
    fn just_return(x: Option<&dyn ::core::any::Any>) -> Block {
      if let Some(x) = x {
        if let Some(x) = x.downcast_ref::<Block>() {
          x.clone()
        } else {
          parse_quote!({ todo!() })
        }
      }
      else {
        parse_quote!({})
      }
    }

    let ItemFn { attrs, vis, sig, block } = ItemFn::parse(input)?;
    if let Some(attr) = attrs.first() {
      return Err(syn::Error::new(attr.pound_token.span, "Attributes aren't supported yet"));
    }

    let (ident, generics, input, output) = match sig {
      Signature { constness: Some(x), .. } => return Err(syn::Error::new(x.span, "Const not supported yet")),
      Signature { asyncness: Some(x), .. } => return Err(syn::Error::new(x.span, "Async not supported yet")),
      Signature { unsafety: Some(x), .. } => return Err(syn::Error::new(x.span, "Unsafe not supported yet")),
      Signature { abi: Some(x), .. } => return Err(syn::Error::new(x.span(), "Abi not supported yet")),
      Signature { variadic: Some(x), .. } => return Err(syn::Error::new(x.span(), "Variadic not supported yet")),
      Signature { ident, generics, inputs, output, .. } => {
        let inputs = {
          let mut xs = Vec::with_capacity(inputs.len());
          for x in inputs.into_iter().map(FnArgBuilder::inner_parse) {
            xs.push(x?)
          }
          xs
        };
        (ident, generics, inputs, output)
      }
    };

    let var_store: Option<Rc<dyn ::core::any::Any>> = Some(Rc::new(*block));
    let calc_body = Rc::new(just_return);

    Ok(Self { vis, ident, input, output, generics, var_store, calc_body })
  }
}


#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use quote::{ToTokens, quote};
  use syn::{parse::Parse, parse_quote, DeriveInput, Data, Expr, Token};
  use crate::{tests::assert_eq_wo_whitespace, FnArgBuilder, FnBuilder};

  fn parse_assert<T: Parse + ToTokens>(s: impl ToString + Clone) {
    parse_assert_against::<T>(s.clone(), s)
  }
  
  fn parse_assert_against<T: Parse + ToTokens>(s: impl ToString, target: impl ToString) {
    let s_tokens = proc_macro2::TokenStream::from_str(s.to_string().as_str()).unwrap();
    let res: T = syn::parse2(s_tokens).unwrap();
    assert_eq_wo_whitespace(res.to_token_stream(), target)
  }

  #[test]
  fn parse_fn_arg() {
    parse_assert::<FnArgBuilder>("&mut self");
    // parse_assert::<FnArgBuilder>("mut self");
    parse_assert::<FnArgBuilder>("&self");
    parse_assert::<FnArgBuilder>("x: A");
    parse_assert::<FnArgBuilder>("x: &A");
    parse_assert::<FnArgBuilder>("x: &mut A");
    parse_assert::<FnArgBuilder>("x: (A, B)");
    parse_assert::<FnArgBuilder>("x: (A,)");
    parse_assert_against::<FnArgBuilder>("x: (A)", "x: A");
  }

  #[test]
  fn parse_fn() {
    parse_assert::<FnBuilder>("fn foo_bar(self) {}");
    parse_assert::<FnBuilder>("pub(crate) fn foo_bar(&mut self, x: &A) -> Self { Self(x.value) }");
    parse_assert::<FnBuilder>("pub fn foo_bar<'a>(x: &A) where A: Clone { x.clone(); }");
    parse_assert::<FnBuilder>("fn foo_bar(&self, x: &B) -> Self where B: ::core::borrow::Borrow<Self> { Self(&self.0 + &x.borrow().0) }");
  }

  #[test]
  fn build_and_collect_bools() {
    let data: DeriveInput = parse_quote!(
      struct X {
        pub vals: (i32, char),
        num: &'static usize,
        bytes: [u8; 9]
      }
    );
    let (ident, data) = if let Data::Struct(s) = &data.data {
      (&data.ident, s)
    } else {panic!()};
    let build_f = |_: &FnArgBuilder| {
      let res: Box<dyn Fn(Expr) -> _> = Box::new(|e| {
        quote!((#e).eval())
      });
      res
    };
    let fn_builder = FnBuilder::new(parse_quote!(foo_bar))
      .set_visibility(parse_quote!(pub))
      .set_output(Some(parse_quote!(bool)))
      .set_input([parse_quote!(&self)])
      .set_body_to_collect_builder::<Token![&&]>(ident, data, &build_f);
    // panic!("{}", fn_builder.to_token_stream())
    assert_eq_wo_whitespace(
      fn_builder.to_token_stream(),
      "pub fn foo_bar(&self) -> bool { (&self.vals.0).eval() && (&self.vals.1).eval() && (&self.num).eval() && (&self.bytes).eval() }"
    );
  }
}

