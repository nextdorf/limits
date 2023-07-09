use std::mem::{take, swap, replace};

use quote::{ToTokens, quote};
use syn::{visit::{self, Visit}, punctuated::Punctuated, Ident, Token, Type, Expr, parse_quote, ExprCall, parse2};


#[derive(Clone, Copy)]
pub enum MemberOf<'a> {
  Tuple(&'a syn::TypeTuple, usize),
  UnnamedFields(&'a syn::FieldsUnnamed, usize),
  NamedFields(&'a syn::FieldsNamed, usize),
}

#[derive(Clone)]
pub struct StructLookup<'a, 'curr: 'a> {
  kind: Option<LookupKind>,
  paths: Vec<Lookup<'a>>,
  current_path: Vec<MemberOf<'curr>>,
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LookupKind {
  Named,
  Unnamed,
  Unit
}

#[derive(Clone)]
pub struct Lookup<'a> {
  pub path: Punctuated<MemberOf<'a>, Token!(.)>,
  pub ty: Type,
}

#[derive(Clone)]
pub struct StructLookupPaths<'a> {
  pub kind: LookupKind,
  pub paths: Vec<Lookup<'a>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LookupAccess {
  Own,
  Ref,
  MutRef,
  Borrow,
}


impl<'ast> Visit<'ast> for StructLookup<'ast, 'ast> {
  fn visit_fields(&mut self, fs: &'ast syn::Fields) {
    let kind = match fs {
      syn::Fields::Named(_) => LookupKind::Named,
      syn::Fields::Unnamed(_) => LookupKind::Unnamed,
      syn::Fields::Unit => LookupKind::Unit,
    };
    self.kind = Some(kind);
    visit::visit_fields(self, fs)
  }

  fn visit_fields_named(&mut self, fs: &'ast syn::FieldsNamed) {
    // skip!(fs.brace_token);
    let current_path = self.take_current_path();
    for (idx, f) in fs.named.iter().enumerate() {
      self.set_current_path(&mut current_path.clone());
      self.push_named_member(fs, idx);
      self.visit_field(f);
    }
  }

  fn visit_fields_unnamed(&mut self, fs: &'ast syn::FieldsUnnamed) {
    // skip!(fs.brace_token);
    let current_path = self.take_current_path();
    for (idx, f) in fs.unnamed.iter().enumerate() {
      self.set_current_path(&mut current_path.clone());
      self.push_unnamed_member(fs, idx);
      self.visit_field(f);
    }
  }

  fn visit_type(&mut self, ty: &'ast Type) {
    match ty {
      Type::Array(_) | Type::Path(_) | Type::Ptr(_) | Type::Reference(_) | Type::Slice(_) => {
        self.register_path(ty.clone())
      },
      Type::Paren(p) => self.visit_type(&*p.elem),
      Type::Tuple(ts) => {
        let current_path = self.take_current_path();
        for (i, ty) in ts.elems.iter().enumerate() {
          self.set_current_path(&mut current_path.clone());
          self.push_tuple_member(ts, i);
          self.visit_type(ty)
        }
      },
      // Type::BareFn(_) => todo!(),
      // Type::Group(_) => todo!(),
      // Type::ImplTrait(_) => todo!(),
      // Type::Infer(_) => todo!(),
      // Type::Macro(_) => todo!(),
      // Type::Never(_) => todo!(),
      // Type::TraitObject(_) => todo!(),
      // Type::Verbatim(_) => todo!(),
      _ => panic!("{}", ty.to_token_stream()),
    }
    // visit::visit_type(self, ty)
  }
}


impl<'curr> StructLookup<'_, 'curr> {
  pub const fn new_with(kind: LookupKind) -> Self {
    Self {
      kind: Some(kind),
      paths: Vec::new(),
      current_path: Vec::new(),
    }
  }

  pub const fn new() -> Self {
    Self {
      kind: None,
      paths: Vec::new(),
      current_path: Vec::new(),
    }
  }


  pub fn push_named_member<'b: 'curr>(&mut self, fs: &'b syn::FieldsNamed, idx: usize) {
    self.push_member(MemberOf::NamedFields(fs, idx))
  }

  pub fn push_unnamed_member<'b: 'curr>(&mut self, fs: &'b syn::FieldsUnnamed, idx: usize) {
    self.push_member(MemberOf::UnnamedFields(fs, idx))
  }

  pub fn push_tuple_member<'b: 'curr>(&mut self, ts: &'b syn::TypeTuple, idx: usize) {
    self.push_member(MemberOf::Tuple(ts, idx))
  }


  pub fn register_path(&mut self, ty: Type) {
    let mut tmp = StructLookup::new();
    swap(self, &mut tmp);
    tmp = tmp.register_path_own(ty);
    swap(self, &mut tmp);
  }


  pub fn kind(&self) -> Option<LookupKind> {
    self.kind
  }

  pub fn paths(&self) -> &Vec<Lookup> {
    &self.paths
  }

  pub fn current_path(&self) -> &Vec<MemberOf> {
    &self.current_path
  }
}


impl<'a, 'curr> StructLookup<'a, 'curr> {
  pub fn set_current_path(&mut self, current_path: &mut Vec<MemberOf<'curr>>) {
    swap(&mut self.current_path, current_path);
  }
  pub fn push_member<'b: 'a + 'curr>(&mut self, member: MemberOf<'b>) {
    self.current_path.push(member)
  }

  pub fn register_path_own<'b>(self, ty: Type) -> StructLookup<'a, 'b> {
    let Self { kind, mut paths, current_path } = self;
    let path = Punctuated::from_iter(current_path);
    paths.push(Lookup { path, ty });
    StructLookup { kind, paths, current_path: Vec::new() }
  }


  pub fn take_current_path_own<'b>(self) -> (StructLookup<'a, 'b>, Vec<MemberOf<'curr>>) {
    let Self { kind, paths, current_path } = self;
    (StructLookup { kind, paths, current_path: Vec::new() }, current_path)
  }
  pub fn take_current_path(&mut self) -> Vec<MemberOf<'curr>> {
    take(&mut self.current_path)
  }

  pub fn reset_current_path_own<'b>(self) -> StructLookup<'a, 'b> {
    let Self { kind, paths, current_path: _ } = self;
    StructLookup { kind, paths, current_path: Vec::new() }
  }
  pub fn reset_current_path(&mut self) {
    self.current_path = Vec::new()
  }
}



impl StructLookupPaths<'_> {
  pub fn with_access(self, expr: &Expr, access: LookupAccess) -> Vec<Expr> {
    self.paths.into_iter()
      .map(|p| p.with_access(expr, access).0)
      .collect()
  }

  pub fn with_access_and_call(self, expr: &Expr, access: LookupAccess, f: impl Fn(Type) -> ExprCall) -> Vec<Expr> {
    self.paths.into_iter()
      .map(|p| p.with_access_and_call(expr, access, &f))
      .collect()
  }

  pub fn with_many(self, exprs: &Vec<Expr>, access: LookupAccess, f: impl Fn(Type) -> ExprCall, g: impl Fn(Vec<Expr>) -> Expr) -> Vec<Expr> {
    let mut res = Vec::new();
    for p in self.paths.into_iter() {
      let exprs = exprs.iter()
        .map(|e| p.clone().with_access_and_call(e, access, &f))
        .collect();
      res.push(g(exprs))
    }
    res
  }

  fn with_many_and_collect(self, ident: &Ident, exprs: &Vec<Expr>, access: LookupAccess, f: impl Fn(Type) -> ExprCall, g: impl Fn(Vec<Expr>) -> Expr) -> Expr {
    fn pop_front_member<'a>(lookup: &mut Lookup<'a>) -> Option<MemberOf<'a>> {
      let mut xs = take(&mut lookup.path).into_iter();
      let member = xs.next();
      lookup.path = Punctuated::from_iter(xs);
      member
    }
    fn inner(member: MemberOf, mut exprs: Vec<(Lookup, Expr)>) -> Expr {
      // let ident = match member {
      //   Member::Named(ident) => ident,
      //   Member::Unnamed(_) => todo!(),
      // };
      // par
      if exprs.len() == 1 {
        let lookup = &mut exprs.first_mut().unwrap().0;
        if lookup.path.len() == 1 {
          exprs.pop().unwrap().1
        } else {
          let member = pop_front_member(lookup).unwrap();
          let expr = inner(member, exprs);
          parse_quote!((#expr, ))
        }
      } else{
        exprs.iter_mut().map(|e| pop_front_member(&mut e.0).unwrap());
        todo!()
      }
    }

    match self.kind {
      LookupKind::Named => {
        let (mut member_curr, mut exprs_curr) = match self.paths.first() {
          Some(lookup) => (lookup.path.first().unwrap().clone(), Vec::new()),
          None => return parse_quote!(#ident {}),
        };
        let exprs_base = self.clone().with_many(exprs, access, f, g);
        let mut exprs = Punctuated::<_, Token!(,)>::new();
        for (lookup, expr) in self.paths.into_iter().zip(exprs_base) {
          let member = lookup.path.first().unwrap();
          if member_curr != *member {
            exprs.push(inner(replace(&mut member_curr, member.clone()), take(&mut exprs_curr)))
          }
          exprs_curr.push((lookup, expr))
        }
        parse_quote!(#ident {#exprs})
      },
      LookupKind::Unnamed => {
        let exprs = self.with_many(exprs, access, f, g);
        let exprs = Punctuated::<_, Token!(,)>::from_iter(exprs);
        parse_quote!(#ident (#exprs))
      },
      LookupKind::Unit => parse_quote!(#ident),
    }
  }
}


impl Lookup<'_> {
  pub fn with_access(self, expr: &Expr, access: LookupAccess) -> (Expr, Type) {
    let Self { path, ty } = self;
    let expr_res = match access {
      LookupAccess::Own => parse_quote!(#expr.#path),
      LookupAccess::Ref => parse_quote!(&#expr.#path),
      LookupAccess::MutRef => parse_quote!(&mut #expr.#path),
      LookupAccess::Borrow => parse_quote!(&#expr.borrow().#path),
    };
    (expr_res, ty)
  }

  pub fn with_access_and_call(self, expr: &Expr, access: LookupAccess, f: impl FnOnce(Type) -> ExprCall) -> Expr {
    let (expr, ty) = self.with_access(expr, access);
    let f_ty = f(ty);
    match access {
      LookupAccess::Own => parse_quote!(#expr.#f_ty),
      _ => parse_quote!((#expr).#f_ty),
    }
  }
}


impl Default for StructLookup<'_, '_> {
  fn default() -> Self {
    Self::new()
  }
}


impl<'a, 'b> TryFrom<StructLookup<'a, 'b>> for StructLookupPaths<'a> {
  type Error = StructLookup<'a, 'b>;

  fn try_from(value: StructLookup<'a, 'b>) -> Result<Self, Self::Error> {
    match (value.current_path.is_empty(), value.kind) {
      (true, Some(kind)) => Ok(Self { kind, paths: value.paths }),
      _ => Err(value)
    }
  }
}


impl PartialEq for MemberOf<'_> {
  fn eq(&self, other: &Self) -> bool {
    fn normalize_tokens(t: &impl ToTokens) -> String {
      t.to_token_stream()
        .to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
    }
    fn eq_punctuated<A: ToTokens, B>(xs: &Punctuated<A, B>, ys: &Punctuated<A, B>, ) -> bool {
      if xs.len() != ys.len() {
        return false;
      }
      xs.iter().zip(ys)
        .find_map(|(x, y)| if normalize_tokens(x) != normalize_tokens(y) {Some(false)} else {None})
        .unwrap_or(true)
    }
    match (self, other) {
      (Self::Tuple(l0, l1), Self::Tuple(r0, r1)) => (l1 == r1) && eq_punctuated(&l0.elems, &r0.elems),
      (Self::UnnamedFields(l0, l1), Self::UnnamedFields(r0, r1)) => (l1 == r1) && eq_punctuated(&l0.unnamed, &r0.unnamed),
      (Self::NamedFields(l0, l1), Self::NamedFields(r0, r1)) => (l1 == r1) && eq_punctuated(&l0.named, &r0.named),
      _ => false,
    }
  }
}
impl Eq for MemberOf<'_> { }


impl ToTokens for MemberOf<'_> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      MemberOf::Tuple(_, idx) | MemberOf::UnnamedFields(_, idx) => {
        syn::Index::from(*idx).to_tokens(tokens)
      },
      MemberOf::NamedFields(fs, idx) => {
        fs.named[*idx].ident.as_ref().unwrap().to_tokens(tokens)
      },
    }
  }
}


#[test]
fn q() {
  let mut lookup = StructLookup::new();
  let input: syn::DeriveInput = parse_quote!(
    pub struct A<T> {
      internal: PhantomData<T>,
      pub value: (u32, u8),
    }
  );
  lookup.visit_derive_input(&input);
  let lookup: StructLookupPaths = match lookup.try_into() {
    Ok(x) => x,
    Err(_) => panic!("..."),
  };

  let mut paths = Punctuated::<_, Token!(,)>::new();
  for p in &lookup.paths {
    let ps = p.path.to_token_stream();
    let ty = &p.ty;
    paths.push(quote!(#ps: #ty));
  }
  let paths = paths.into_token_stream().to_string();
  let s = match lookup.kind {
    LookupKind::Named => format!("_ {{{paths}}}"),
    LookupKind::Unnamed => format!("_ ({paths})"),
    LookupKind::Unit => "_".to_string(),
  };
  panic!("{s}")
}


