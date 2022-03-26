pub trait ASTNode {}


pub trait Groupable: std::fmt::Debug {}

#[derive(Debug)]
pub struct Element {
    pub value: String
}

#[derive(Debug)]
pub struct ForumulaUnit {
    pub value: Vec<Box<dyn Groupable>>,
    pub coeffecient: i16
}

#[derive(Debug)]
pub struct Expression {
    pub value: Vec<Box<ForumulaUnit>>
}

#[derive(Debug)]
pub struct Equation(Expression, Expression);

#[derive(Debug)]
pub struct Group {
    pub value: Vec<Box<dyn Groupable>>
}

impl Groupable for Element {}
impl Groupable for Group {}

impl ASTNode for Element {}
impl ASTNode for Group {}
impl ASTNode for ForumulaUnit {}
impl ASTNode for Expression {}
impl ASTNode for Equation {}