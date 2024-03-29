use std::fmt::Display;

pub enum StageId { }

impl StageId {
  pub fn asStr(&self) -> &str {
    todo!( )
  }
}

impl Display for StageId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.asStr( ))
  }
}