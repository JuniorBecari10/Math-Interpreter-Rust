#[derive(PartialEq)]
pub enum Node {
  NumberLit(f64),
  Bin(Box<Node>, Box<Node>),
  Unary(Box<Node>),
}
