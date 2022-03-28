
// TODO: change Node to be proper types once
// we get variants as types, eventually
#[derive(Debug, PartialEq)]
pub enum Node {
    Element(u16, String),
    Group(u16, Vec<Node>),
    ForumulaUnit(u16, Vec<Node>),
    Reactants(Vec<Node>),
    Products(Vec<Node>),
    Equation(Box<Node>, Box<Node>)
}