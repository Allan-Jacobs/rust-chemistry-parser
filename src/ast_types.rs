// TODO: change Node to be proper types once
// we get variants as types, eventually
/// A Node in the AST (abstract syntax tree)
#[derive(Debug, PartialEq)]
pub enum Node {
    /// Chemical elements e.g. O2.
    /// The number is the subscript, in this case 2
    Element(u16, String),
    /// Groups of elements or other groups (Polyatoms) e.g. (2FeO2)4
    /// The number is the subscript, in this case 4
    Group(u16, Vec<Node>),
    /// Forumula Unit, a few elements and/or polyatoms eg 7(NH4)2SO4
    /// This number is the coeffecient, in this case 7
    ForumulaUnit(u16, Vec<Node>),
    /// The reactants side (left) of yields (->)
    Reactants(Vec<Node>),
    /// The products side (right) of yeilds (->)
    Products(Vec<Node>),
    /// The whole equation
    Equation(Box<Node>, Box<Node>),
}
