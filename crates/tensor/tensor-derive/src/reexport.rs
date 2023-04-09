use syn::{parse_quote, Meta};


#[derive(Debug)]
pub struct OptField<T> {
  pub set_field: Option<T>,
  pub default_field: T,
}

pub struct AttrRepr {
  pub zero_path: OptField<syn::Path>,
}


impl<T> OptField<T> {
  pub const fn new(default: T) -> Self {
    Self { set_field: None, default_field: default }
  }

  pub fn get(&self) -> &T {
    self.set_field.as_ref().unwrap_or(&self.default_field)
  }

  pub fn set(&mut self, value: T) {
    self.set_field = Some(value)
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
            "num_traits_export" => res.zero_path.set(xs.parse_args().unwrap()),
            s => println!("Unknown: {}", s)
          }
        }
      }
    }
    res
  }
}

impl Default for AttrRepr {
  fn default() -> Self {
    Self {
      zero_path: OptField::new(parse_quote!(tensor::group::Zero))
    }
  }
}

