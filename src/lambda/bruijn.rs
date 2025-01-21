use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Bruijn {
    Val(i32),
    Abs(Box<Bruijn>),
    App(Box<Bruijn>, Box<Bruijn>),
    Lit(String),
}

impl Bruijn {
    pub fn eval(&self) -> Self {
        use Bruijn::*;
        match self {
            App(expr1, expr2) => {
                let new_expr1 = expr1.eval();
                if let Abs(ref body) = new_expr1 {
                    body.subst(0, &expr2)
                } else {
                    App(expr1.clone(), expr2.clone())
                }
            }
            a => a.clone(),
        }
    }

    fn subst(&self, cutt_off: i32, param: &Self) -> Self {
        use Bruijn::*;
        match self {
            Val(varid) => {
                if *varid == cutt_off {
                    param.clone()
                } else {
                    Val(*varid)
                }
            }
            App(expr1, expr2) => {
                let new_expr1 = expr1.subst(cutt_off, &param);
                let new_expr2 = expr2.subst(cutt_off, &param);

                App(Box::new(new_expr1), Box::new(new_expr2))
            }
            Abs(body) => Abs(Box::new(body.subst(cutt_off + 1, &param))),
            Lit(s) => Lit(s.clone()),
        }
    }
}

impl fmt::Display for Bruijn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Bruijn::*;
        match self {
            Val(index) => write!(f, "{}", index),
            Abs(body) => write!(f, "(\\ {})", body),
            App(expr1, expr2) => write!(f, "({} {})", expr1, expr2),
            Lit(str) => write!(f, "#{}", str),
        }
    }
}
