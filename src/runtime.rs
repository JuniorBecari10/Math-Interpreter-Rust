use crate::ast;

struct Interpreter {
  node: ast::Node,
  code: String,
  had_error: bool,
}

impl Interpreter {
  fn new(n: ast::Node, code: String) -> Self {
    Interpreter { node: n, code, had_error: false }
  }

  fn get_value(&mut self) -> f64 {
    self.interpret(self.node.clone())
  }

  fn interpret(&mut self, node: ast::Node) -> f64 {
    let v = match node {
      ast::Node::NumberLit(n) => Some(self.number_lit(n)),
      ast::Node::Bin(a, b, op) => Some(self.bin(*a, *b, op)),
      ast::Node::Unary(n, op) => Some(self.unary(*n, op)),
      ast::Node::None => {
        self.report_error("Couldn't interpret node.", self.code.clone(), 0);
        None
      },
    };

    v.unwrap_or(0.0)
  }

  // ---

  fn number_lit(&self, n: f64) -> f64 {
    n
  }

  fn bin(&mut self, a: ast::Node, b: ast::Node, op: char) -> f64 {
    let na = self.interpret(a);
    let nb = self.interpret(b);

    match op {
      '+' => na + nb,
      '-' => na - nb,
      '*' => na * nb,
      '/' => {
        let mut v = 0.0;

        if nb != 0.0 {
          v = na / nb;
        }
        else {
          self.report_error("Can't divide by zero.", self.code.clone(), 0);
        }

        v
      },

      _ => 0.0
    }
  }

  fn unary(&mut self, n: ast::Node, op: char) -> f64 {
    let v = self.interpret(n);

    match op {
      '+' => v.abs(),
      '-' => -v,

      _ => v
    }
  }

  fn report_error(&mut self, msg: &str, code: String, pos: usize) {
    self.had_error = true;

    println!("\nError: {}\n", msg);
    println!(" {}", code);

    for _ in 0..pos + 1 {
      print!(" ");
    }
    
    println!("^");
  }
}

pub fn interpret(n: ast::Node, code: String) -> (f64, bool) {
  let mut int = Interpreter::new(n, code);
  
  (int.get_value(), int.had_error)
}
