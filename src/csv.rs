extern crate csv;

use super::{exit, Error};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

#[derive(Deserialize, Serialize)]
pub struct CsvMap {
    #[serde(flatten)]
    values: HashMap<String, String>,
}

pub fn append<R: Read>(results: &mut Vec<CsvMap>, reader: R) {
    for row in Reader::from_reader(reader).deserialize() {
        let record: CsvMap = match row {
            Ok(values) => values,
            Err(e) => {
                eprintln!("Error parsing input: {e}");
                exit(Error::InputParsing as i32);
            }
        };
        results.push(record);
    }
}
