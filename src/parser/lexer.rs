use std::iter::IntoIterator;
use super::token_types::Token;


pub trait TokenStream: IntoIterator {
    fn from(string: String) -> Self;
}

struct MemoryTokenStream {
    tokens: Vec<Box<dyn Token>>
}

impl IntoIterator for MemoryTokenStream {
    type Item = Box<dyn Token>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl TokenStream for MemoryTokenStream {
    fn from(string: String) -> Self {
        let tokens = vec!();
        
        MemoryTokenStream {
            tokens
        }
    }
}