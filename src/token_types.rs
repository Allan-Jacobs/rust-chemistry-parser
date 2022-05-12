use core::fmt::Debug;

/// A parenthesis type.
/// '(' is `OPEN` and ')' is `CLOSE`
#[derive(Debug, PartialEq)]
pub enum ParenType {
    OPEN,
    CLOSE
}

/// A token which may have attached data
#[derive(Debug)]
pub enum Tokens {
    /// Elements e.g. Fe. The data is "Fe"
    Element { data: String, meta: TokenMetadata },
    /// Numbers e.g. 13. The data is 13u16
    Number { data: u16, meta: TokenMetadata },
    /// Parenthesis e.g. ) The data is ParenType::CLOSE
    Paren { data: ParenType, meta: TokenMetadata },
    /// Plus sign +
    Plus { meta: TokenMetadata},
    /// Yields sign -> 
    Yields { meta: TokenMetadata },
}

impl Tokens {
    pub fn meta(&self) -> TokenMetadata {
        match *self {
            Self::Element { meta, data: _ } => meta,
            Self::Number { meta, data: _ } => meta,
            Self::Paren { meta, data: _ } => meta,
            Self::Plus { meta} => meta,
            Self::Yields { meta} => meta
        }
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Element { data: l_data, meta: _ }, Self::Element { data: r_data, meta: r_meta }) => l_data == r_data,
            (Self::Number { data: l_data, meta: _ }, Self::Number { data: r_data, meta: r_meta }) => l_data == r_data,
            (Self::Paren { data: l_data, meta: _ }, Self::Paren { data: r_data, meta: r_meta }) => l_data == r_data,
            (Self::Plus { meta: l_meta }, Self::Plus { meta: _ }) => true,
            (Self::Yields { meta: l_meta }, Self::Yields { meta: _ }) => true,
        }
    }
}

#[derive(Debug)]
pub struct TokenMetadata {
    raw: String,
    location: usize
}


impl TokenMetadata {
    pub fn new(raw: &str, location: usize) -> Self {
        Self {
            raw: raw.into(),
            location
        }
    }
}