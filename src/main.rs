
pub mod lambda;
use lambda::named::*;
use lambda::*;

fn main() {
    use Expr::*;

    let expr_app = 
    Abs("a".to_string(), Box::new(
        Abs("b".to_string(), Box::new(
            App(Box::new(Var("a".to_string())), Box::new(Var("b".to_string())))
        ))
    ));

    let expr_id = Abs("a".to_string(), Box::new(Var("a".to_string())));
    
    let expr_to_eval = App(Box::new(App(Box::new(expr_app), Box::new(expr_id))), Box::new(Var("v".to_string())));
    println!("Expr = {}", expr_to_eval);

    let b_to_eval = expr_to_bruijn(&expr_to_eval).unwrap();

    println!("Bruijn = {}", b_to_eval);
    println!("Eval Bruijn = {}", b_to_eval.eval());
}
