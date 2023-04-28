use std::ops::{Deref, DerefMut};

use syn::{parse_quote, Meta, punctuated::Punctuated, Token};


#[derive(Debug)]
pub struct OptField<T> {
  pub set_field: Option<T>,
  pub default_field: T,
}

pub struct AttrRepr {
  pub zero_path: OptField<syn::Path>,
  pub one_path: OptField<syn::Path>,
  pub inv_path: OptField<syn::Path>,
  pub gen_group_path: OptField<syn::Path>,
  pub gen_abel_group_path: OptField<syn::Path>,
  pub unit_idents: OptField<Vec<syn::Ident>>,
}


impl<T> OptField<T> {
  pub const fn new(default: T) -> Self {
    Self { set_field: None, default_field: default }
  }
}


impl AttrRepr {
  pub fn new_with<'a>(iter: impl Iterator<Item = &'a syn::Attribute>) -> Self {
    let mut res = Self::default();
    for x in iter.into_iter() {
      if let Meta::List(xs) = &x.meta {
        if let Some(ident) = xs.path.get_ident() {
          // println!("1: {}", xs.to_token_stream());
          match ident.to_string().as_str() {
            "num_traits_zero_path" => *res.zero_path = xs.parse_args().unwrap(),
            "num_traits_one_path" => *res.one_path = xs.parse_args().unwrap(),
            "num_traits_inv_path" => *res.inv_path = xs.parse_args().unwrap(),
            "gen_group_path" => *res.gen_group_path = xs.parse_args().unwrap(),
            "gen_abel_group_path" => *res.gen_abel_group_path = xs.parse_args().unwrap(),
            //TODO
            s => eprintln!("Unknown: {}", s)
          }
        }
      }
    }
    res
  }
}


impl<T> Deref for OptField<T> {
  type Target = T;
  fn deref(&self) -> &T {
    self.set_field.as_ref().unwrap_or(&self.default_field)
  }
}

impl<T> DerefMut for OptField<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.set_field.as_mut().unwrap_or(&mut self.default_field)
  }
}


impl Default for AttrRepr {
  fn default() -> Self {
    Self {
      zero_path: OptField::new(parse_quote!(::tensor::group::Zero)),
      one_path: OptField::new(parse_quote!(::tensor::group::One)),
      inv_path: OptField::new(parse_quote!(::tensor::group::Inv)),
      gen_group_path: OptField::new(parse_quote!(::tensor::GenGroup)),
      gen_abel_group_path: OptField::new(parse_quote!(::tensor::GenAbelGroup)),
      unit_idents: OptField::new({
        let idents: Punctuated::<_, Token![,]> = parse_quote![PhantomData];
        idents.into_iter().collect()
      }),
    }
  }
}



