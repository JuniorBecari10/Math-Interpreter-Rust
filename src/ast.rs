#[derive(PartialEq, Debug, Clone)]
pub enum Node {
  NumberLit(f64),
  Bin(Box<Node>, Box<Node>, char),
  Unary(Box<Node>, char),

  None
}
