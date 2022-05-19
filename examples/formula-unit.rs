use chem_parse::parse;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let string = String::from("Fe2O3");
    let ast = parse(string)?;
    // Ast: ForumulaUnit(1, [Element(2, "Fe"), Element(3, "O")])
    println!("Ast: {:?}", ast);
    Ok(())
}
