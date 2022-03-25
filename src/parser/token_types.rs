pub trait Token {}

#[derive(Debug)]
enum ParenType {
    OPEN,
    CLOSE
}

#[derive(Debug)]
pub struct Element {
    value: &'static str // TODO: use enum
}

#[derive(Debug)]
pub struct Number {
    value: i16
}

#[derive(Debug)]
pub struct Paren {
    value: ParenType
}

#[derive(Debug)]
pub struct Plus();

#[derive(Debug)]
pub struct Yields();

impl Token for Element {}
impl Token for Number {}
impl Token for Paren {}
impl Token for Plus {}
impl Token for Yields {}