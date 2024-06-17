// main.rs
mod app_options;
mod option_removal;
mod statement_processor;

#[cfg(test)]
mod statement_processor_test;

use crate::app_options::process_options;
use crate::statement_processor::process_statement_file;
use crate::statement_processor::print_records;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = process_options();
    let records = process_statement_file(&options.file)?;
    print_records(records);
    Ok(())
}
