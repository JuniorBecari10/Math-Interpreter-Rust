use crate::token;
use crate::ast;

struct Parser {
  input: Vec<token::Token>,
  code: String,
  cursor: usize,
  hadError: bool
}

impl Parser {
  fn new(input: Vec<token::Token>, code: String) -> Self {
    Self { input: input, code: code, cursor: 0, hadError: false }
  }

  fn advance(&mut self) {
    self.cursor += 1
  }

  fn is_at_end(&self) -> bool {
    self.cursor >= self.input.len()
  }

  fn token(&self) -> token::Token {
    if self.is_at_end() { return token::end_token(self.cursor) }

    self.input[self.cursor].clone()
  }

  fn exp(&mut self) -> ast::Node {

  }

  fn term(&mut self) -> ast::Node {

  }

  fn factor(&mut self) -> ast::Node {

  }
}
