use crate::token;
use crate::ast;

struct Parser {
  input: Vec<token::Token>,
  code: String,
  cursor: usize,
  had_error: bool
}

impl Parser {
  fn new(input: Vec<token::Token>, code: String) -> Self {
    Self { input: input, code: code, cursor: 0, had_error: false }
  }

  fn advance(&mut self) {
    self.cursor += 1
  }

  fn is_at_end(&self) -> bool {
    self.cursor >= self.input.len()
  }

  fn peek_is_at_end(&self) -> bool {
    self.cursor + 1 >= self.input.len()
  }

  fn prev_is_at_end(&self) -> bool {
    self.cursor - 1 >= self.input.len()
  }

  fn token(&self) -> token::Token {
    if self.is_at_end() { return token::end_token(self.cursor) }

    self.input[self.cursor].clone()
  }

  fn peek(&self) -> token::Token {
    if self.peek_is_at_end() { return token::end_token(self.cursor) }

    self.input[self.cursor + 1].clone()
  }

  fn prev(&self) -> token::Token {
    if self.prev_is_at_end() { return token::end_token(self.cursor) }

    self.input[self.cursor - 1].clone()
  }

  fn token_equal_any(&self, kinds: &[token::TokenKind]) -> bool {
    for k in kinds {
      let kind = k.clone();

      if self.token().kind == kind { return true }
    }

    false
  }

  fn peek_equal_any(&self, kinds: &[token::TokenKind]) -> bool {
    for k in kinds {
      let kind = k.clone();

      if self.peek().kind == kind { return true }
    }

    false
  }

  fn exp(&mut self) -> ast::Node {
    let mut exp = self.term();

    if self.token_equal_any(&[token::TokenKind::Plus, token::TokenKind::Minus]) {
      self.advance();

      let right = self.term();

      exp = ast::Node::Bin(Box::new(exp), Box::new(right), self.prev().lexeme.chars().nth(0).unwrap());
    }

    exp
  }

  fn term(&mut self) -> ast::Node {
    let mut exp = self.factor();

    if self.peek_equal_any(&[token::TokenKind::Star, token::TokenKind::Slash]) {
      self.advance();

      let right = self.factor();
      exp = ast::Node::Bin(Box::new(exp), Box::new(right), self.prev().lexeme.chars().nth(0).unwrap());
    }

    exp
  }

  fn factor(&mut self) -> ast::Node {
    if self.token().kind == token::TokenKind::LParen {
      self.advance();

      if self.token().kind != token::TokenKind::RParen {
        self.report_error("Unclosed parentheses.", self.code.clone(), self.token().pos);

        return ast::Node::None;
      }
    }

    if self.token_equal_any(&[token::TokenKind::Plus, token::TokenKind::Minus]) {
      let tk = self.token();
      self.advance();

      return ast::Node::Unary(Box::new(self.factor()), tk.lexeme.chars().nth(0).unwrap());
    }

    if self.token().kind == token::TokenKind::Number {
      let n = match self.token().content {
        token::Value::Number(n) => n,
        _ => 0.0
      };
      
      return ast::Node::NumberLit(n);
    }

    self.report_error(format!("Couldn't find expression node for '{}'.", self.token().lexeme).as_str(), self.code.clone(), self.token().pos);
    ast::Node::None
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

pub fn parse(input: Vec<token::Token>, code: String) -> (ast::Node, bool) {
  let mut l = Parser::new(input, code);

  (l.exp(), l.had_error)
}
