pub mod bruijn;
pub mod named;

use std::collections::HashMap;

use bruijn::*;
use named::*;

type Context = HashMap<String, i32>;

pub fn expr_to_bruijn(expr: &Expr) -> Option<Bruijn> {
    fn shift_context(context: &Context, shift_amount: i32) -> Context {
        let mut shifted_context: Context = HashMap::new();

        for (&ref key, &val) in context.iter() {
            shifted_context.insert(key.clone(), val + shift_amount);
        }

        shifted_context
    }
    let context: Context = HashMap::new();

    fn match_expr(expr: &Expr, context: &Context) -> Option<Bruijn> {
        match expr {
            Expr::Var(varid) => {
                if let Some(index) = context.get(varid) {
                    Some(Bruijn::Val(*index))
                } else {
                    Some(Bruijn::Lit(varid.clone()))
                }
            }
            Expr::Abs(head, body) => {
                let mut new_context: Context = shift_context(context, 1);
                new_context.insert(head.clone(), 0);

                let new_body = match_expr(body, &new_context)?;

                return Some(Bruijn::Abs(Box::new(new_body)));
            }
            Expr::App(expr1, expr2) => {
                let new_expr1 = match_expr(expr1, &context)?;
                let new_expr2 = match_expr(expr2, &context)?;

                Some(Bruijn::App(Box::new(new_expr1), Box::new(new_expr2)))
            }
        }
    }

    match_expr(&expr, &context)
}
