use std::fmt;
use std::collections::HashMap;

macro_rules! bx {
    ($e:expr) => {
        Box::new($e)
    };
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
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

#[derive(Debug, Clone, PartialEq)]
enum Bruijn {
    Val(i32),
    Abs(Box<Bruijn>),
    App(Box<Bruijn>, Box<Bruijn>),
    Lit(String),
}

impl Bruijn {    
    fn eval(&self) -> Self {
        use Bruijn::*;
        match self {
            App(expr1, expr2) => {
                let new_expr1 = expr1.eval();
                if let Abs(ref body) = new_expr1 {
                    body.subst(0, &expr2)
                } else {
                    App(expr1.clone(), expr2.clone())
                }
            },
            a => a.clone() 
        }
    }

    fn subst(&self, cutt_off : i32, param : &Self) -> Self {
        use Bruijn::*;
        match self {
            Val(varid) => {
                if *varid == cutt_off {
                    param.clone()
                } else {
                    Val(*varid)
                }
            },
            App(expr1, expr2) => {
                let new_expr1 = expr1.subst(cutt_off, &param);
                let new_expr2 = expr2.subst(cutt_off, &param);
                
                App(Box::new(new_expr1), Box::new(new_expr2))
            },
            Abs(body) => {
                Abs(bx!(body.subst(cutt_off + 1, &param)))
            },
            Lit(s) => Lit(s.clone()),
        }
    }
}

impl fmt::Display for Bruijn {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        use Bruijn::*;
        match self{
            Val(index) => write!(f, "{}", index),
            Abs(body)        => write!(f, "(\\ {})", body),
            App(expr1, expr2) => write!(f, "({} {})", expr1, expr2),
            Lit(str) => write!(f, "#{}", str),
        }
    }
}

type Context = HashMap<String, i32>;

fn shift_context(context : &Context, shift_amount : i32) -> Context {
    let mut shifted_context: Context = HashMap::new();

    for (&ref key, &val) in context.iter(){
        shifted_context.insert(key.clone(), val + shift_amount);
    }

    shifted_context
}

fn expr_to_bruijn(expr : &Expr) -> Option<Bruijn> {
    let context: Context = HashMap::new();

    fn match_expr(expr : &Expr, context : &Context) -> Option<Bruijn> {
        match expr {
            Expr::Var(varid) => {
                if let Some(index) = context.get(varid) {
                    Some(Bruijn::Val(*index))
                } else {
                    Some(Bruijn::Lit(varid.clone()))
                }
            },
            Expr::Abs(head, body) => {
                let mut new_context: Context = shift_context(context, 1);
                new_context.insert(head.clone(), 0);
                
                let new_body = match_expr(body, &new_context)?;

                return Some(Bruijn::Abs(bx!(new_body)));
            },
            Expr::App(expr1, expr2) => {
                let new_expr1 = match_expr(expr1, &context)?;
                let new_expr2 = match_expr(expr2, &context)?;
                
                Some(Bruijn::App(bx!(new_expr1), bx!(new_expr2)))
            },
        }
    }

    match_expr(&expr, &context)
}


fn main() {
    use Expr::*;

    let expr_app = 
    Abs("a".to_string(), bx!(
        Abs("b".to_string(), bx!(
            App(bx!(Var("a".to_string())), bx!(Var("b".to_string())))
        ))
    ));

    let expr_id = Abs("a".to_string(), Box::new(Var("a".to_string())));
    
    let expr_to_eval = App(Box::new(App(Box::new(expr_app), Box::new(expr_id))), Box::new(Var("v".to_string())));
    println!("Expr = {}", expr_to_eval);

    let b_to_eval = expr_to_bruijn(&expr_to_eval).unwrap();

    println!("Bruijn = {}", b_to_eval);
    println!("Eval Bruijn = {}", b_to_eval.eval());
}
