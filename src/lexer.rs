use crate::token::{self, Token};

struct Lexer {
  input: Vec<char>,
  cursor: usize,
  had_error: bool,
}

impl Lexer {
  fn new(input: String) -> Self {
    Self { input: input.chars().collect(), cursor: 0, had_error: false }
  }

  fn advance(&mut self) {
    self.cursor += 1
  }

  fn is_at_end(&self) -> bool {
    self.cursor >= self.input.len()
  }

  fn prev_is_at_end(&self) -> bool {
    self.cursor - 1 >= self.input.len()
  }

  fn prev(&self) -> char {
    if self.prev_is_at_end() { return '\0' }

    self.input[self.cursor - 1]
  }

  fn char(&self) -> char {
    if self.is_at_end() { return '\0' }

    self.input[self.cursor]
  }

  fn skip_whitespace(&mut self) {
    while self.char() == ' ' { self.advance() }
  }

  fn next_token(&mut self) -> token::Token {
    self.skip_whitespace();

    if self.is_at_end() {
      return Token {
        kind: token::TokenKind::End,
        lexeme: "".into(),
        content: token::Value::None,
        pos: self.cursor
      };
    }

    let kind = match self.char() {
      '+' => token::TokenKind::Plus,
      '-' => token::TokenKind::Minus,
      '*' => token::TokenKind::Star,
      '/' => token::TokenKind::Slash,

      '(' => token::TokenKind::LParen,
      ')' => token::TokenKind::RParen,

      _ => {
        if is_number(self.char()) {
          let pos = self.cursor;
          self.advance();

          while is_number(self.char()) { self.advance() }

          let buf = self.input[pos..self.cursor].to_vec();
          let res = buf.iter().cloned().collect::<String>().parse::<f64>();

          let n = match res {
            Ok(n) => n,
            Err(_) => {
              self.report_error("Couldn't parse number.", self.input.iter().cloned().collect::<String>(), pos);
              0.0
            }
          };

          return token::Token {
            kind: token::TokenKind::Number,
            lexeme: buf.iter().cloned().collect::<String>(),
            content: token::Value::Number(n),
            pos: pos,
          };
          
        }

        token::TokenKind::Never
      }
    };

    self.advance();

    token::Token {
      kind: kind,
      lexeme: self.prev().to_string(),
      content: token::Value::Char(self.prev()),
      pos: self.cursor - 1
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

fn is_number(c: char) -> bool {
  c.is_numeric() || c == '.'
}

pub fn lex(input: String) -> (Vec<token::Token>, bool) {
  let mut l = Lexer::new(input);
  let mut tokens: Vec<token::Token> = Vec::new();
  let mut tk = l.next_token();

  while tk.kind != token::TokenKind::End {
    tokens.push(tk);
    tk = l.next_token();
  }

  (tokens, l.had_error)
}
