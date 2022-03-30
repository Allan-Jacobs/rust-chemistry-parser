use core::fmt::Debug;

/// A parenthesis type.
/// '(' is `OPEN` and ')' is `CLOSE`
#[derive(Debug, PartialEq)]
pub enum ParenType {
    OPEN,
    CLOSE
}

/// A token which may have attached data
#[derive(Debug, PartialEq)]
pub enum Tokens {
    /// Elements e.g. Fe. The data is "Fe"
    Element(String),
    /// Numbers e.g. 13. The data is 13u16
    Number(u16),
    /// Parenthesis e.g. ) The data is ParenType::CLOSE
    Paren(ParenType),
    /// Plus sign +
    Plus,
    /// Yields sign -> 
    Yields,
}