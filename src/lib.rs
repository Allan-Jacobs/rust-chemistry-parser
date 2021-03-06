#![feature(box_patterns)]

pub mod ast_types;
pub mod lexer;
pub mod parser;
pub mod token_types;

use self::{ast_types::Node, lexer::LazyTokenStream};

/// Parse a string and return a result with either the node or the error
pub fn parse(string: String) -> Result<Box<Node>, String> {
    let stream = LazyTokenStream::new(&string);
    parser::parse(stream)
}
