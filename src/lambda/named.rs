use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Var(String),
  Abs(String, Box<Expr>),
  App(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use Expr::*;
    match self {
        Var(varid) => write!(f, "{}", varid),
        Abs(head, body) => write!(f, "(\\{} -> {})", head, body),
        App(expr1, expr2) => write!(f, "({} {})", expr1, expr2),
    }
  }
}
