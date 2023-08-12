use syn::{Attribute, Type, punctuated::Punctuated, Token};

pub fn get_unit_types(attrs: &Vec<Attribute>) -> Option<Vec<Type>> {
  let attr = attrs.iter().find(|a| a.path().is_ident("unit_types"))?;
  let elems = attr.parse_args_with(
    Punctuated::<Type, Token!(,)>::parse_terminated
  ).ok()?;
  let res = elems.into_iter().collect();
  Some(res)
}



