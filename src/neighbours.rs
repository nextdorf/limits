pub struct Infinitesimal<Dir, Scale> {
  pub dir: Dir,
  pub scale: Scale,
}

pub struct Neighbour<At, Inf> {
  pub at: At,
  pub dir: Dir,
  pub inf: Inf,
}




