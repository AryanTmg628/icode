use std::io::prelude::*;

pub struct Buffer {
    pub file: Option<String>,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn from_file(file: &str) -> Result<(std::fs::File, Vec<String>), std::io::Error> {
        //reading from the file

        let mut file = std::fs::File::open(file)?;
        let mut lines = String::new();

        file.read_to_string(&mut lines)?;
        let all_lines: Vec<String> = lines.lines().map(|s| s.to_string()).collect();
        Ok((file, all_lines))
    }
}
