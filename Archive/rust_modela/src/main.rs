mod calculator;
mod csv_reader;

fn main() {
    let mut calculator = calculator::Calculator::new(10.0);
    calculator.add(5.0);
    calculator.multiply(2.0);
    println!("{}", calculator.format_value());

    let mut csv_reader = csv_reader::CsvReader::new("W:\\DataAnnotation\\Rust\\203-424.csv").unwrap();
    csv_reader.print_records().unwrap();

}