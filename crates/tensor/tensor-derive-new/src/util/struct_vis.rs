use std::{mem::{take, swap, replace}, collections::HashMap, fmt::Debug};

use petgraph::{data::Build, Graph, graph::NodeIndex, visit::IntoNodeReferences};
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
  pub const fn new_empty(kind: Option<LookupKind>) -> Self {
    Self {
      kind,
      paths: Vec::new(),
      current_path: Vec::new(),
    }
  }

  pub const fn new_with(kind: LookupKind) -> Self {
    Self::new_empty(Some(kind))
  }

  pub const fn new() -> Self {
    Self::new_empty(None)
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
    // swap(&mut self.current_path, current_path);
    self.current_path = take(current_path)
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


/// Build a "tuple tower"
/// fn(vec[a.u.x, a.u.y, a.v, b.w.z]) -> [(a, ((a.u.x, a.u.y), a.v)), (b, (b.w.z, ))]
pub fn collect_exprs_at_lookup_old<'a>(exprs_at_lookup: Vec<(Lookup<'a>, Expr)>) -> Vec<(MemberOf<'a>, Expr)> {
  fn pop_front_member<'a>(lookup: &mut Lookup<'a>) -> Option<MemberOf<'a>> {
    let mut xs = take(&mut lookup.path).into_iter();
    let member = xs.next();
    lookup.path = Punctuated::from_iter(xs);
    member
  }

  let mut curr_member = match exprs_at_lookup.first() {
    Some((lookup, _)) => *lookup.path.first().expect("msg"),
    None => return vec![],
  };
  let mut res = Vec::new();
  let mut curr_exprs = Vec::<(Lookup, Expr)>::new();
  fn put_from_curr_into_res(curr_member: MemberOf, curr_exprs: &mut Vec<(Lookup, Expr)>) {
    {
      for (lookup, _) in &*curr_exprs {
        let m0 = *lookup.path.first().unwrap();
        debug_assert!(curr_member == m0)
      }
    }
  };
  for (lookup, expr) in exprs_at_lookup {
    let member = *lookup.path.first().expect("msg");
    if curr_member != member {
      put_from_curr_into_res(curr_member, &mut curr_exprs)
    }
    curr_exprs.push((lookup, expr))
  }
  if !curr_exprs.is_empty() {
    put_from_curr_into_res(curr_member, &mut curr_exprs)
  }

  res
}

#[derive(Clone)]
enum GraphNode<'a> {
  Member(usize, MemberOf<'a>),
  Expr(Expr),
  Start
}
impl<'a> GraphNode<'a> {
  pub const fn from_member((idx, member): (usize, MemberOf<'a>)) -> Self {
    Self::Member(idx, member)
  }
}
impl GraphNode<'_> {
  pub const fn from_expr(expr: Expr) -> Self {
    Self::Expr(expr)
  }
}
impl Debug for GraphNode<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Member(arg0, arg1) => {
        let arg1 = arg1.to_token_stream().to_string();
        f.debug_tuple("Member").field(arg0).field(&arg1).finish()
      },
      Self::Expr(arg0) => {
        let arg0 = arg0.to_token_stream().to_string();
        f.debug_tuple("Expr").field(&arg0).finish()
      },
      Self::Start => write!(f, "Start"),
    }
  }
}

/// Build a "tuple tower"
/// fn(vec[a.u.x, a.u.y, a.v, b.w.z]) -> [(a, ((a.u.x, a.u.y), a.v)), (b, (b.w.z, ))]
fn collect_exprs_at_lookup_as_graph<'a>(exprs_at_lookup: Vec<(Lookup<'a>, Expr)>) -> (Graph<GraphNode<'a>, ()>, NodeIndex) {
  let mut graph = Graph::new();
  let mut graph_nodes = Vec::new();
  let idx0 = graph.add_node(GraphNode::Start);
  for (lookup, expr) in exprs_at_lookup {
    let mut old_idx = idx0;
    for x in lookup.path.into_iter().enumerate() {
      let new_idx = match graph_nodes.iter().find(|(y, _)| x==*y) {
        Some((_, idx)) => *idx,
        None => {
          let idx = graph.add_node(GraphNode::from_member(x));
          graph_nodes.push((x, idx));
          idx
        }
      };
      graph.update_edge(
        replace(&mut old_idx, new_idx),
        new_idx,
        ()
      );
    }
    let new_idx = graph.add_node(GraphNode::from_expr(expr));
    graph.update_edge(old_idx, new_idx, ());
  }

  // panic!("{:?}", graph);
  (graph, idx0)
}

// fn unwrap_exprs_at_lookup_graph<'a>(graph: &mut Graph<GraphNode<'a>, ()>, idx: NodeIndex) -> Vec<(Option<MemberOf<'a>>, Expr)> {
//   let mut edges = graph.neighbors(idx).detach();
//   let n0 = match edges.next_node(graph) {
//     Some(n0) => n0,
//     None => return Vec::new(),
//   };
//   match edges.next_node(graph) {
//     None => {
//       // let edge = *(edges[0]);
//       // let node = graph.node_references().collect::<Vec<_>>()[0].
//       match *&graph[n0] {
//         GraphNode::Member(_, m) => match m {
//           MemberOf::Tuple(_, _) => todo!(),
//           MemberOf::UnnamedFields(_, _) => todo!(),
//           MemberOf::NamedFields(_, _) => todo!(),
//         },
//         GraphNode::Expr(expr) => panic!("No MemberOf"),
//         GraphNode::Start => unreachable!(),
//       }
//       todo!()
//     },
//     Some(n1) => {
//       let idxs = Some(()).into_iter().cycle().map_while(|()| edges.next_node(graph));
//       let idxs = [n0, n1].into_iter().chain(idxs);
//       todo!()
//     }
//   }
// }
fn rewrap_exprs_at_lookup_from_graph<'a>(graph: &Graph<GraphNode<'a>, ()>, idx: NodeIndex) -> Vec<(Option<MemberOf<'a>>, Expr)> {
  let mut edges = graph.neighbors(idx).detach();
  let mut res = Some(()).into_iter()
    .cycle()
    .map_while(|()| match edges.next_node(graph) {
      Some(idx) => Some(match &graph[idx] {
        GraphNode::Member(_, member) => {
          let exprs = rewrap_exprs_at_lookup_from_graph(graph, idx);
          let set_member = exprs.first().unwrap().0.is_some();
          #[cfg(debug_assertions)]
          {
            for (m, _) in exprs.iter().skip(1) {
              debug_assert_eq!(m.is_some(), set_member)
            }
          }
          let expr = match (set_member, exprs.len()) {
            (false, 1) => exprs.first().unwrap().1.clone(),
            (true, 1) => {
              let expr = &exprs.first().unwrap().1;
              parse_quote!((#expr,))
            },
            (true, _) => {
              let exprs = exprs.into_iter().map(|(_, e)| e);
              let exprs = Punctuated::<_, Token!(,)>::from_iter(exprs);
              parse_quote!((#exprs))
            },
            (false, _) => unreachable!(),
          };
          (Some(*member), expr)
        },
        GraphNode::Expr(expr) => (None, expr.clone()),
        GraphNode::Start => unreachable!(),
      }),
      None => None,
    }).collect::<Vec<_>>();
  if let Some((Some(_), _)) = res.first() {
    res.sort_by_key(|(m, _)| match m.unwrap() {
      MemberOf::Tuple(_, idx) => idx,
      MemberOf::UnnamedFields(_, idx) => idx,
      MemberOf::NamedFields(_, idx) => idx,
    })
  }
  res
}


pub fn collect_exprs_at_lookup<'a>(exprs_at_lookup: Vec<(Lookup<'a>, Expr)>) -> Vec<(MemberOf<'a>, Expr)> {
  let (graph, idx) = collect_exprs_at_lookup_as_graph(exprs_at_lookup);
  let res = rewrap_exprs_at_lookup_from_graph(&graph, idx);
  res.into_iter()
    .map(|(m, e)| (m.unwrap(), e))
    .collect()
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

  pub fn with_many(self, exprs: &Vec<Expr>, access: LookupAccess, f: Option<impl Fn(Type) -> ExprCall>, g: impl Fn(Vec<Expr>) -> Expr) -> Vec<Expr> {
    let mut res = Vec::new();
    for p in self.paths.into_iter() {
      let exprs = exprs.iter()
        .map(|e| match &f {
          Some(f) => p.clone().with_access_and_call(e, access, f),
          None => p.clone().with_access(e, access).0,
        }).collect();
      res.push(g(exprs))
    }
    res
  }

  fn with_many_and_collect(self, ident: &Ident, exprs: &Vec<Expr>, access: LookupAccess, f: Option<impl Fn(Type) -> ExprCall>, g: impl Fn(Vec<Expr>) -> Expr) -> Expr {
    fn pop_front_member<'a>(lookup: &mut Lookup<'a>) -> Option<MemberOf<'a>> {
      let mut xs = take(&mut lookup.path).into_iter();
      let member = xs.next();
      lookup.path = Punctuated::from_iter(xs);
      member
    }

    match self.kind {
      LookupKind::Named | LookupKind::Unnamed => {
        let map_fn = match self.kind {
          LookupKind::Named => |(m, e): (_, Expr)| quote!(#m: #e),
          LookupKind::Unnamed => |(_, e): (_, Expr)| e.into_token_stream(),
          _ => unreachable!(),
        };
        let exprs_base = self.clone().with_many(exprs, access, f, g);
        let exprs_at_lookup = self.paths.into_iter().zip(exprs_base);
        let exprs = collect_exprs_at_lookup(exprs_at_lookup.collect());
        // panic!("{:?}", exprs.into_iter().map(|(m, e)| (m.to_token_stream().to_string(), e.to_token_stream().to_string())).collect::<Vec<_>>());
        let exprs = exprs.into_iter().map(map_fn);
        let exprs = Punctuated::<_, Token!(,)>::from_iter(exprs);
        match self.kind {
          LookupKind::Named => parse_quote!(#ident {#exprs}),
          LookupKind::Unnamed => parse_quote!(#ident (#exprs)),
          _ => unreachable!(),
        }
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
  // Create the data structure which represents the input and allows for partial representation.
  // This means it's intended to be mutable and the content might be garbage until all visit_*
  // functions are called.
  let mut lookup = StructLookup::new();
  // The input to represent
  let input: syn::DeriveInput = parse_quote!(
    pub struct A<T> {
      internal: PhantomData<T>,
      pub value: (u32, u8),
    }
  );
  // Visit the input and try to build the final data-structure. Unlike StructLookup StructLookupPaths
  // isn't a partial representation.
  lookup.visit_derive_input(&input);
  let lookup: StructLookupPaths = match lookup.try_into() {
    Ok(x) => x,
    Err(_) => panic!("..."),
  };

  // Print the found representation with a panic
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

#[test]
fn test_add_fn() {
  // Create the data structure which represents the input and allows for partial representation.
  // This means it's intended to be mutable and the content might be garbage until all visit_*
  // functions are called.
  let mut lookup = StructLookup::new();
  // The input to represent
  let input: syn::DeriveInput = parse_quote!(
    pub struct X {
      f_value: f64,
      pub value: (i32, i8),
      s_value: ((i8,), )
    }
  );
  // Visit the input and try to build the final data-structure. Unlike StructLookup StructLookupPaths
  // isn't a partial representation.
  lookup.visit_derive_input(&input);
  let lookup: StructLookupPaths = match lookup.try_into() {
    Ok(x) => x,
    Err(_) => panic!("..."),
  };

  let ident = parse_quote!(x);
  let (u, v) = (parse_quote!(u), parse_quote!(v));
  let exprs = vec![u, v];
  // let f = |_| parse_quote!(add_with());
  let f = if false {Some(|_| parse_quote!(add_with()))} else {None};
  let g = |es: Vec<_>| es.into_iter().next().unwrap();
  let res = lookup.with_many_and_collect(&ident, &exprs, LookupAccess::Ref, f, g);
  panic!("{}", res.to_token_stream().to_string())
}


