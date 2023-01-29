use std::{path::Path, io, fs};

pub fn read_file(path: &Path) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}