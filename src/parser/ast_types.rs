trait ASTNode {}


trait Groupable: std::fmt::Debug {}

#[derive(Debug)]
struct Element {
    value: &'static str
}

#[derive(Debug)]
struct ForumulaUnit {
    value: Vec<Box<dyn Groupable>>,
    coeffecient: i16
}

#[derive(Debug)]
struct Expression {
    value: Vec<Box<ForumulaUnit>>
}

#[derive(Debug)]
struct Equation(Expression, Expression);

#[derive(Debug)]
struct Group {
    value: Vec<Box<dyn Groupable>>
}

impl Groupable for Element {}
impl Groupable for Group {}

impl ASTNode for Element {}
impl ASTNode for Group {}
impl ASTNode for ForumulaUnit {}
impl ASTNode for Expression {}
impl ASTNode for Equation {}