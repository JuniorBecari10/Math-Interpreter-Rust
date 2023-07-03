use crate::ast;

#[derive(PartialEq)]
struct Number {
  value: f64
}

fn interpret(n: ast::Node) -> Number {
  let f = get_fn(n);


}

fn get_func(n: ast::Node) -> Option<impl Fn(ast::Node) -> Number> {
  
}
