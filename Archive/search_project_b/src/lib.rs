// lib.rs

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub fn search_file(query: &str, file_path: &str, _ignore_case: bool) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        match line.contains(&query) {
            true => println!("{}", line),
            false => {}
        }
    }

    Ok(())
}
