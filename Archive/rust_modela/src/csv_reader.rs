use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use prettytable::{Table, Row, Cell};

pub struct CsvReader {
    reader: csv::Reader<BufReader<File>>,
}

impl CsvReader {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        Ok(CsvReader { reader })
    }

    pub fn print_records(&mut self) -> io::Result<()> {
        let headers = self.reader.headers()?;
        let mut table = Table::new();
        table.add_row(Row::new(headers.iter().map(|h| Cell::new(h)).collect()));

        for result in self.reader.records() {
            let record = result?;
            table.add_row(Row::new(record.iter().map(|r| Cell::new(r)).collect()));
        }

        table.printstd();
        Ok(())
    }
}