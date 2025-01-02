use bincode::deserialize_from;
use camino::Utf8PathBuf;
use std::collections::HashMap;
use std::fs::File;

mod error;
mod model;
use crate::model::target::Target;
pub use error::{Error, Result};

pub fn bincode2targets(input: &Utf8PathBuf) -> Result<HashMap<String, Target>> {
    let f = File::open(input)?;
    let targets: HashMap<String, Target> = deserialize_from(f)?;
    println!("{} targets returned", targets.len());
    Ok(targets)
}
