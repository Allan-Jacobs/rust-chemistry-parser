use core::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum ParenType {
    OPEN,
    CLOSE
}

#[derive(Debug, PartialEq)]
pub enum Tokens {
    Element(String),
    Number(u16),
    Paren(ParenType),
    Plus,
    Yields,
}