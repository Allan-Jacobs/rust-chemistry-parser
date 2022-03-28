use self::{ast_types::Node, lexer::LazyTokenStream};

mod token_types;
mod ast_types;
mod lexer;
mod parser;

pub fn parse(string: String) -> Result<Box<Node>, String> {
    let stream = LazyTokenStream::new(&string);
    parser::parse(stream)
}