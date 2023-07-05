#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenKind {
  Number,
  Never,
  End,

  Plus,
  Minus,
  Star,
  Slash,

  LParen,
  RParen,
}

#[derive(Debug, Clone)]
pub enum Value {
  Char(char),
  Number(f64),
  None
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub content: Value,
  pub pos: usize,
}

pub fn end_token(pos: usize) -> Token {
  Token {
    kind: TokenKind::End,
    lexeme: "".into(),
    content: Value::None,
    pos
  }
}
