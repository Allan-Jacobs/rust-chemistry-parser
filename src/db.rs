use std::io::{self, prelude::*};
use std::fs::File;
use std::error::Error;

use serde::Deserialize;

struct PeriodicTable {
    value: Vec<ElementRecord>
}

#[derive(Debug, Deserialize)]
struct ElementRecord {
    AtomicNumber: u8,
    Element: String,
    Symbol: String,
    AtomicMass: f64,
    NumberofNeutrons: u8,
    NumberofProtons: u8,
    NumberofElectrons: u8,
    Period: u8,
    Group: Option<u8>,
    Phase: String,
    Radioactive: String,
    Natural: String,
    Metal: String,
    Nonmetal: String,
    Metalloid: String,
    Type: String,
    AtomicRadius: Option<f64>,
    Electronegativity: Option<f64>,
    FirstIonization: Option<f64>,
    Density: Option<f64>,
    MeltingPoint: Option<f64>,
    BoilingPoint: Option<f64>,
    NumberOfIsotopes: Option<u8>,
    Discoverer: String,
    Year: Option<u16>,
    SpecificHeat: Option<f64>,
    NumberofShells: u8,
    NumberofValence: Option<u8>
}

#[derive(Debug)]
pub struct Element {
    atomic_number: u8,
    element: String,
    symbol: String,
    atomic_mass: f64,
}

impl From<ElementRecord> for Element {
    fn from(record: ElementRecord) -> Self {
        Self {
            atomic_number: record.AtomicNumber,
            element: record.Element,
            symbol: record.Symbol,
            atomic_mass: record.AtomicMass,
        }
    }
}


pub struct DB {
    vec: Option<Vec<Element>>
}

impl DB {
    pub fn new() -> Self {
        Self {
            vec: None
        }
    }

    pub fn get_elements(&mut self) -> Result<&Vec<Element>, Box<dyn Error>> {
        match self.vec {
            Some(ref vec) => Ok(&vec),
            None => {
                self.vec = Some(read_csv()?);
                return Ok(self.vec.as_ref().unwrap());
            }
        }
    }
}

fn read_csv() -> Result<Vec<Element>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let file = File::open("Periodic Table of Elements.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut vec: Vec<Element> = vec!();
    for result in rdr.deserialize() {
        let record: ElementRecord = result?;
        vec.push(record.into());
    }
    Ok(vec)
}