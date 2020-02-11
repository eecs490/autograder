//use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;

pub fn write(string: &str, path: &str) -> std::io::Result<()> {
    let mut buffer = File::create(path)?;
    // Writes some prefix of the byte string, not necessarily all of it.
    buffer.write(string.as_bytes())?;

    Ok(())
}
