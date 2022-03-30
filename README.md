# chem-parse [![Crates.io](https://img.shields.io/crates/v/chem-parse)](https://crates.io/crates/chem-parse)
A parser for simple chemical formulas.

## Get Started
Add the dependency to your `Cargo.toml` file.

```toml
[dependencies]
chem-parse = "0.1.0"
```

Parse a forumula unit

```rs
use std::error::Error;
use chem_parse::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let string = String::from("Fe2O3");
    let ast = parse(string)?;
    // Ast: ForumulaUnit(1, [Element(2, "Fe"), Element(3, "O")])
    println!("Ast: {:?}", ast);
    Ok(())
}
```

Parse an equation
```rs
use std::error::Error;
use chem_parse::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let string = String::from("4Fe+3O2->2Fe2O");
    let ast = parse(string)?;
    // Node: comment broken up into multiple lines to save space
    // Ast: Equation(
    //   Reactants([ForumulaUnit(4, [Element(1, "Fe")]), ForumulaUnit(3, [Element(2, "O")])]),
    //   Products([ForumulaUnit(2, [Element(2, "Fe"), Element(1, "O")])])
    // )
    println!("Ast: {:?}", ast);
    Ok(())
}
```
