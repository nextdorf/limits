use syn::{Attribute, Path, parse_str};

fn get_opt_path(attrs: &Vec<Attribute>, ident: &str) -> Option<Path> {
  let attr = attrs.iter().find(|a| a.path().is_ident(ident))?;
  let res = attr.parse_args::<Path>()
    .expect("Unexpected format");
  Some(res)
}

fn get_path(attrs: &Vec<Attribute>, ident: &str, default: &str) -> Path {
  match get_opt_path(attrs, ident) {
    Some(p) => p,
    None => parse_str(default).expect(
      format!("default {} not a valid path", default).as_str()
    ),
  }
}

pub fn get_zero_path(attrs: &Vec<Attribute>) -> Path {
  get_path(attrs, "zero_path", "::tensor::Zero")
}

pub fn get_one_path(attrs: &Vec<Attribute>) -> Path {
  get_path(attrs, "one_path", "::tensor::One")
}

pub fn get_derive_generic(attrs: &Vec<Attribute>) -> Option<Path> {
  get_opt_path(attrs, "derive_generic")
}



