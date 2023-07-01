#[derive(Eq, PartialEq, Debug)]
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

#[derive(Debug)]
pub enum Value {
  Char(char),
  Number(f64),
  None
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub content: Value,
  pub pos: usize,
}
