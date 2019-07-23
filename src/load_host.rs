use crate::err::Error;
use serde_json;
use std::fs::File;
use std::path::PathBuf;

/// Load host by json file
pub fn load_host(path: PathBuf) -> Result<Vec<String>, Error> {
    let fl = File::open(path)?;
    let data: Vec<String> = serde_json::from_reader(fl)?;
    Ok(data)
}
