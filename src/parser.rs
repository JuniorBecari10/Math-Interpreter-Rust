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

  fn token(&self) -> token::Token {
    if self.is_at_end() { return token::end_token(self.cursor) }

    self.input[self.cursor].clone()
  }

  fn token_equal_any(&self, kinds: &[token::TokenKind]) -> bool {
    for k in kinds {
      let kind = k.clone();

      if self.token().kind == kind { return true }
    }

    false
  }

  fn exp(&mut self) -> ast::Node {
    let mut exp = self.term();

    if self.token_equal_any(&[token::TokenKind::Plus, token::TokenKind::Minus]) {
      let op = self.token();
      self.advance();

      let right = self.term();

      exp = ast::Node::Bin(Box::new(exp), Box::new(right), op.lexeme.chars().nth(0).unwrap());
    }

    exp
  }

  fn term(&mut self) -> ast::Node {
    let mut exp = self.factor();

    if self.token_equal_any(&[token::TokenKind::Star, token::TokenKind::Slash]) {
      let op = self.token();
      self.advance();

      let right = self.factor();
      exp = ast::Node::Bin(Box::new(exp), Box::new(right), op.lexeme.chars().nth(0).unwrap());
    }

    exp
  }

  fn factor(&mut self) -> ast::Node {
    let tk = self.token();

    if tk.kind == token::TokenKind::LParen {
      self.advance();

      let exp = self.exp();

      if self.token().kind != token::TokenKind::RParen {
        self.report_error("Unclosed parentheses.", self.code.clone(), self.token().pos);

        return ast::Node::None;
      }

      self.advance();
      return exp;
    }

    if self.token_equal_any(&[token::TokenKind::Plus, token::TokenKind::Minus]) {
      self.advance();

      return ast::Node::Unary(Box::new(self.factor()), tk.lexeme.chars().nth(0).unwrap());
    }

    if tk.kind == token::TokenKind::Number {
      self.advance();

      let n = match tk.content {
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
