use std::error::Error;
use chem_parse::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let string = String::from("4Fe+3O2->2Fe2O");
    let ast = parse(string)?;
    // Node: comment broken up into multiple lines
    // Ast: Equation(
    //   Reactants([ForumulaUnit(4, [Element(1, "Fe")]), ForumulaUnit(3, [Element(2, "O")])]),
    //   Products([ForumulaUnit(2, [Element(2, "Fe"), Element(1, "O")])])
    // )
    println!("Ast: {:?}", ast);
    Ok(())
}