// statement_processor.rs
use std::fs;
use csv::ReaderBuilder;
use regex::Regex;
use chrono::{NaiveDate, NaiveTime};
use prettytable::{Table, Row, Cell};
use crate::option_removal::OptionRemoval;

pub fn process_statement_file(path: &String) -> Result<Vec<OptionRemoval>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;

    let mut csv_data = vec![];
    let mut processing_csv = false;

    for line in data.lines() {
        if line.is_empty() && processing_csv {
            break;
        }

        if processing_csv {
            csv_data.push(line.to_string());
        } else if line.starts_with("Cash Balance") {
            processing_csv = true;
        }
    }

    let csv_string = csv_data.join("\n");
    let mut reader = ReaderBuilder::new()
        .from_reader(csv_string.as_bytes());

    let mut records = vec![];

    let re = Regex::new(r"(?ix)
        \s  # whitespace
        (?P<quantity>-?\d+\.\d+)    # quantity
        \s
        \$?
        (?P<symbol>[A-Z]+)          # symbol
        \.?[A-Z]*                   # optional suffix
        \s+
        \d+
        \s*
        (?:\(WEEKLY\)\s*)?          # optional weekly suffix
        (?P<expiration>\d+)         # expiration
        \s
        (?P<month>\w+)              # month
        \s
        (?P<year>\d+)               # year
        \s+
        (?P<strike_price>[.\d]+)    # strike price
        \s+
        (?P<side>CALL|PUT)          # side
    ").unwrap();

    for result in reader.records() {
        let record = result?;

        if let Some(caps) = re.captures(record.get(4).unwrap()) {
            let date: NaiveDate = NaiveDate::parse_from_str(record.get(0).unwrap(), "%m/%d/%y")?;
            let time: NaiveTime = NaiveTime::parse_from_str(record.get(1).unwrap(), "%H:%M:%S")?;
            let quantity: f64 = caps["quantity"].parse()?;
            let symbol: String = caps["symbol"].to_string();
            let expiration: NaiveDate = NaiveDate::parse_from_str(&format!("{} {} {}", &caps["year"], &caps["month"], &caps["expiration"]), "%Y %b %d")?;
            let strike_price: f64 = caps["strike_price"].parse()?;
            let side: String = caps["side"].to_string();

            records.push(OptionRemoval::new(date, time, quantity, symbol, expiration, strike_price, side));
        }
    }

    Ok(records)
}

pub fn print_records(records: Vec<OptionRemoval>) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Date"),
        Cell::new("Time"),
        Cell::new("Quantity"),
        Cell::new("Symbol"),
        Cell::new("Expiration"),
        Cell::new("Strike Price"),
        Cell::new("Side"),
    ]));

    for record in records {
        table.add_row(Row::new(vec![
            Cell::new(&record.date.to_string()),
            Cell::new(&record.time.to_string()),
            Cell::new(&record.quantity.to_string()),
            Cell::new(&record.symbol),
            Cell::new(&record.expiration.to_string()),
            Cell::new(&record.strike_price.to_string()),
            Cell::new(&record.side),
        ]));
    }

    table.printstd();
}
