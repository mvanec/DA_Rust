use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub fn search_file(query: &str, file_path: &str, ignore_case: bool) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Process the lines in the file. If the ignore-case flag is set
    for line in reader.lines() {
        let line = line?;
        match ignore_case {
            true => { // Compare everything in lower case. Might not work with all character sets
                if line.to_string().to_lowercase().contains(&query.to_lowercase()) {
                    println!("{}", line);
                }
            }
            false => {
                if line.to_string().contains(query) {
                    println!("{}", line);
                }
            }
        }
    }

    Ok(())
}
